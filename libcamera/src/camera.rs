use std::{
    collections::HashMap,
    ffi::CStr,
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Mutex,
};

use libcamera_sys::*;

use crate::{
    control::{ControlInfoMap, ControlList, PropertyList},
    request::Request,
    stream::{StreamConfigurationRef, StreamRole},
    utils::Immutable,
};

/// Status of [CameraConfiguration]
#[derive(Debug, Clone, Copy)]
pub enum CameraConfigurationStatus {
    /// Camera configuration was validated without issues.
    Valid,
    /// Camera configuration is valid, but some of the fields were adjusted by libcamera.
    Adjusted,
    /// Camera configuration is invalid.
    Invalid,
}

impl CameraConfigurationStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    pub fn is_adjusted(&self) -> bool {
        matches!(self, Self::Adjusted)
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid)
    }
}

impl TryFrom<libcamera_camera_configuration_status_t> for CameraConfigurationStatus {
    type Error = ();

    fn try_from(value: libcamera_camera_configuration_status_t) -> Result<Self, Self::Error> {
        match value {
            libcamera_camera_configuration_status::LIBCAMERA_CAMERA_CONFIGURATION_STATUS_VALID => Ok(Self::Valid),
            libcamera_camera_configuration_status::LIBCAMERA_CAMERA_CONFIGURATION_STATUS_ADJUSTED => Ok(Self::Adjusted),
            libcamera_camera_configuration_status::LIBCAMERA_CAMERA_CONFIGURATION_STATUS_INVALID => Ok(Self::Invalid),
            _ => Err(()),
        }
    }
}

/// Camera configuration.
///
/// Contains [StreamConfigurationRef] for each stream used by the camera.
pub struct CameraConfiguration {
    ptr: NonNull<libcamera_camera_configuration_t>,
}

impl CameraConfiguration {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_configuration_t>) -> Self {
        Self { ptr }
    }

    /// Returns immutable [StreamConfigurationRef] for the camera stream.
    ///
    /// # Parameters
    ///
    /// * `index` - Camera stream index.
    pub fn get(&self, index: usize) -> Option<Immutable<StreamConfigurationRef<'_>>> {
        let ptr = unsafe { libcamera_camera_configuration_at(self.ptr.as_ptr(), index as _) };
        NonNull::new(ptr).map(|p| Immutable(unsafe { StreamConfigurationRef::from_ptr(p) }))
    }

    /// Returns mutable [StreamConfigurationRef] for the camera stream.
    ///
    /// # Parameters
    ///
    /// * `index` - Camera stream index.
    pub fn get_mut(&mut self, index: usize) -> Option<StreamConfigurationRef<'_>> {
        let ptr = unsafe { libcamera_camera_configuration_at(self.ptr.as_ptr(), index as _) };
        NonNull::new(ptr).map(|p| unsafe { StreamConfigurationRef::from_ptr(p) })
    }

    /// Returns number of streams within camera configuration.
    pub fn len(&self) -> usize {
        unsafe { libcamera_camera_configuration_size(self.ptr.as_ptr()) }
    }

    /// Returns `true` if camera configuration has no streams.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Validates camera configuration.
    pub fn validate(&mut self) -> CameraConfigurationStatus {
        unsafe { libcamera_camera_configuration_validate(self.ptr.as_ptr()) }
            .try_into()
            .unwrap()
    }
}

impl core::fmt::Debug for CameraConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len() {
            list.entry(&self.get(i).unwrap().0);
        }
        list.finish()
    }
}

impl Drop for CameraConfiguration {
    fn drop(&mut self) {
        unsafe { libcamera_camera_configuration_destroy(self.ptr.as_ptr()) }
    }
}

/// A read-only instance of a camera.
///
/// Can be used to obtain camera parameters or supported stream configurations.
/// In order to be used for capturing, it must be turned into an [ActiveCamera] by [Camera::acquire()].
pub struct Camera<'d> {
    pub(crate) ptr: NonNull<libcamera_camera_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Camera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    /// ID of the camera.
    ///
    /// This usually contains hardware path within the system and is not human-friendly.
    /// Use [properties::Model](crate::properties::Model) from [Camera::properties()] to obtain a human readable
    /// identification instead.
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_id(self.ptr.as_ptr())) }
            .to_str()
            .unwrap()
    }

    /// Returns a list of available camera controls and their limit.
    pub fn controls(&self) -> &ControlInfoMap {
        unsafe {
            ControlInfoMap::from_ptr(NonNull::new(libcamera_camera_controls(self.ptr.as_ptr()).cast_mut()).unwrap())
        }
    }

    /// Returns a list of camera properties.
    ///
    /// See [properties](crate::properties) for available items.
    pub fn properties(&self) -> &PropertyList {
        unsafe {
            PropertyList::from_ptr(NonNull::new(libcamera_camera_properties(self.ptr.as_ptr()).cast_mut()).unwrap())
        }
    }

    /// Generates default camera configuration for the given [StreamRole]s.
    ///
    /// The resulting [CameraConfiguration] contains stream configurations for each of the requested roles.
    ///
    /// Generated configuration can be adjusted as needed and then passed onto [ActiveCamera::configure()] to apply.
    pub fn generate_configuration(&self, roles: &[StreamRole]) -> Option<CameraConfiguration> {
        let roles: Vec<libcamera_stream_role::Type> = roles.iter().map(|r| (*r).into()).collect();
        let cfg =
            unsafe { libcamera_camera_generate_configuration(self.ptr.as_ptr(), roles.as_ptr(), roles.len() as _) };
        NonNull::new(cfg).map(|p| unsafe { CameraConfiguration::from_ptr(p) })
    }

    /// Acquires exclusive rights to the camera, which allows changing configuration and capturing.
    pub fn acquire(&self) -> io::Result<ActiveCamera<'_>> {
        let ret = unsafe { libcamera_camera_acquire(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(unsafe { ActiveCamera::from_ptr(NonNull::new(libcamera_camera_copy(self.ptr.as_ptr())).unwrap()) })
        }
    }
}

impl<'d> Drop for Camera<'d> {
    fn drop(&mut self) {
        unsafe { libcamera_camera_destroy(self.ptr.as_ptr()) }
    }
}

extern "C" fn camera_request_completed_cb(ptr: *mut core::ffi::c_void, req: *mut libcamera_request_t) {
    let mut state = unsafe { &*(ptr as *const Mutex<ActiveCameraState<'_>>) }
        .lock()
        .unwrap();
    let req = state.requests.remove(&req).unwrap();

    if let Some(cb) = &mut state.request_completed_cb {
        cb(req);
    }
}

#[derive(Default)]
struct ActiveCameraState<'d> {
    /// List of queued requests that are yet to be executed.
    /// Used to temporarily store [Request] before returning it back to the user.
    requests: HashMap<*mut libcamera_request_t, Request>,
    /// Callback for libcamera `requestCompleted` signal.
    request_completed_cb: Option<Box<dyn FnMut(Request) + Send + 'd>>,
}

/// An active instance of a camera.
///
/// This gives exclusive access to the camera and allows capturing and modifying configuration.
///
/// Obtained by [Camera::acquire()].
pub struct ActiveCamera<'d> {
    cam: Camera<'d>,
    /// Handle to disconnect `requestCompleted` signal.
    request_completed_handle: *mut libcamera_callback_handle_t,
    /// Internal state that is shared with callback handlers.
    state: Box<Mutex<ActiveCameraState<'d>>>,
}

impl<'d> ActiveCamera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_t>) -> Self {
        let mut state = Box::new(Mutex::new(ActiveCameraState::default()));

        let request_completed_handle = unsafe {
            libcamera_camera_request_completed_connect(
                ptr.as_ptr(),
                Some(camera_request_completed_cb),
                // state is valid for the lifetime of `ActiveCamera` and this callback will be disconnected on drop.
                state.as_mut() as *mut Mutex<ActiveCameraState<'_>> as *mut _,
            )
        };

        Self {
            cam: Camera::from_ptr(ptr),
            request_completed_handle,
            state,
        }
    }

    /// Sets a callback for completed camera requests.
    ///
    /// Callback is executed in the libcamera thread context so it is best to setup a channel to send all requests for
    /// processing elsewhere.
    ///
    /// Only one callback can be set at a time. If there was a previously set callback, it will be discarded when
    /// setting a new one.
    pub fn on_request_completed(&mut self, cb: impl FnMut(Request) + Send + 'd) {
        let mut state = self.state.lock().unwrap();
        state.request_completed_cb = Some(Box::new(cb));
    }

    /// Applies camera configuration.
    ///
    /// Default configuration can be obtained from [Camera::generate_configuration()] and then adjusted as needed.
    pub fn configure(&mut self, config: &mut CameraConfiguration) -> io::Result<()> {
        let ret = unsafe { libcamera_camera_configure(self.ptr.as_ptr(), config.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    /// Creates a capture [`Request`].
    ///
    /// To perform a capture, it must firstly be initialized by attaching a framebuffer with [Request::add_buffer()] and
    /// then queued for execution by [ActiveCamera::queue_request()].
    ///
    /// # Arguments
    ///
    /// * `cookie` - An optional user-provided u64 identifier that can be used to uniquely identify request in request
    ///   completed callback.
    pub fn create_request(&mut self, cookie: Option<u64>) -> Option<Request> {
        let req = unsafe { libcamera_camera_create_request(self.ptr.as_ptr(), cookie.unwrap_or(0)) };
        NonNull::new(req).map(|p| unsafe { Request::from_ptr(p) })
    }

    /// Queues [`Request`] for execution. Completed requests are returned in request completed callback, set by the
    /// `ActiveCamera::on_request_completed()`.
    ///
    /// Requests that do not have attached framebuffers are invalid and are rejected without being queued.
    pub fn queue_request(&self, req: Request) -> io::Result<()> {
        let ptr = req.ptr.as_ptr();
        self.state.lock().unwrap().requests.insert(ptr, req);

        let ret = unsafe { libcamera_camera_queue_request(self.ptr.as_ptr(), ptr) };

        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    /// Starts camera capture session.
    ///
    /// Once started, [ActiveCamera::queue_request()] is permitted and camera configuration can no longer be changed.
    pub fn start(&mut self, controls: Option<&ControlList>) -> io::Result<()> {
        let ctrl_ptr = controls.map(|c| c.ptr()).unwrap_or(core::ptr::null_mut());
        let ret = unsafe { libcamera_camera_start(self.ptr.as_ptr(), ctrl_ptr) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    /// Stops camera capture session.
    ///
    /// Once stopped, [ActiveCamera::queue_request()] is no longer permitted and camera configuration can be adjusted.
    pub fn stop(&mut self) -> io::Result<()> {
        let ret = unsafe { libcamera_camera_stop(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }
}

impl<'d> Deref for ActiveCamera<'d> {
    type Target = Camera<'d>;

    fn deref(&self) -> &Self::Target {
        &self.cam
    }
}

impl<'d> DerefMut for ActiveCamera<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cam
    }
}

impl<'d> Drop for ActiveCamera<'d> {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_request_completed_disconnect(self.ptr.as_ptr(), self.request_completed_handle);
            libcamera_camera_stop(self.ptr.as_ptr());
            libcamera_camera_release(self.ptr.as_ptr());
        }
    }
}
