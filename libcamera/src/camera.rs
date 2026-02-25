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
    camera_manager::CameraTracker,
    control::{ControlInfoMap, ControlList, PropertyList},
    geometry::{Rectangle, Size},
    request::Request,
    stream::{Stream, StreamConfigurationRef, StreamRole},
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

/// Desired orientation of the captured image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Rotate0,
    Rotate0Mirror,
    Rotate180,
    Rotate180Mirror,
    Rotate90Mirror,
    Rotate270,
    Rotate270Mirror,
    Rotate90,
}

impl TryFrom<libcamera_orientation_t> for Orientation {
    type Error = ();

    fn try_from(value: libcamera_orientation_t) -> Result<Self, Self::Error> {
        match value {
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0 => Ok(Self::Rotate0),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0_MIRROR => Ok(Self::Rotate0Mirror),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180 => Ok(Self::Rotate180),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180_MIRROR => Ok(Self::Rotate180Mirror),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90_MIRROR => Ok(Self::Rotate90Mirror),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270 => Ok(Self::Rotate270),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270_MIRROR => Ok(Self::Rotate270Mirror),
            libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90 => Ok(Self::Rotate90),
            _ => Err(()),
        }
    }
}

impl From<Orientation> for libcamera_orientation_t {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Rotate0 => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0,
            Orientation::Rotate0Mirror => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0_MIRROR,
            Orientation::Rotate180 => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180,
            Orientation::Rotate180Mirror => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180_MIRROR,
            Orientation::Rotate90Mirror => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90_MIRROR,
            Orientation::Rotate270 => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270,
            Orientation::Rotate270Mirror => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270_MIRROR,
            Orientation::Rotate90 => libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90,
        }
    }
}

pub struct SensorConfiguration {
    item: NonNull<libcamera_sensor_configuration_t>,
}

impl SensorConfiguration {
    pub fn new() -> Self {
        let ptr = NonNull::new(unsafe { libcamera_sensor_configuration_create() }).unwrap();
        Self { item: ptr }
    }

    pub fn from_ptr(ptr: NonNull<libcamera_sensor_configuration_t>) -> Self {
        Self { item: ptr }
    }

    pub fn set_bit_depth(&mut self, depth: u32) {
        unsafe { libcamera_sensor_configuration_set_bit_depth(self.item.as_ptr(), depth) }
    }

    pub fn set_output_size(&mut self, width: u32, height: u32) {
        unsafe { libcamera_sensor_configuration_set_output_size(self.item.as_ptr(), width, height) }
    }

    pub fn set_analog_crop(&mut self, crop: Rectangle) {
        let rect: libcamera_rectangle_t = crop.into();
        unsafe { libcamera_sensor_configuration_set_analog_crop(self.item.as_ptr(), &rect) }
    }

    pub fn set_binning(&mut self, x: u32, y: u32) {
        unsafe { libcamera_sensor_configuration_set_binning(self.item.as_ptr(), x, y) }
    }

    pub fn set_skipping(&mut self, x_odd: u32, x_even: u32, y_odd: u32, y_even: u32) {
        unsafe { libcamera_sensor_configuration_set_skipping(self.item.as_ptr(), x_odd, x_even, y_odd, y_even) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { libcamera_sensor_configuration_is_valid(self.item.as_ptr()) }
    }

    pub fn bit_depth(&self) -> u32 {
        unsafe { libcamera_sensor_configuration_get_bit_depth(self.item.as_ptr()) }
    }

    pub fn output_size(&self) -> Size {
        let size = unsafe { libcamera_sensor_configuration_get_output_size(self.item.as_ptr()) };
        size.into()
    }

    pub fn analog_crop(&self) -> Rectangle {
        unsafe { libcamera_sensor_configuration_get_analog_crop(self.item.as_ptr()).into() }
    }

    pub fn binning(&self) -> (u32, u32) {
        let mut x = 0;
        let mut y = 0;
        unsafe { libcamera_sensor_configuration_get_binning(self.item.as_ptr(), &mut x, &mut y) };
        (x, y)
    }

    pub fn skipping(&self) -> (u32, u32, u32, u32) {
        let (mut x_odd, mut x_even, mut y_odd, mut y_even) = (0, 0, 0, 0);
        unsafe {
            libcamera_sensor_configuration_get_skipping(
                self.item.as_ptr(),
                &mut x_odd,
                &mut x_even,
                &mut y_odd,
                &mut y_even,
            )
        };
        (x_odd, x_even, y_odd, y_even)
    }
}

impl Default for SensorConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SensorConfiguration {
    fn drop(&mut self) {
        unsafe { libcamera_sensor_configuration_destroy(self.item.as_ptr()) }
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

    /// Append a new stream configuration for a given role.
    pub fn add_configuration(&mut self) -> Option<StreamConfigurationRef<'_>> {
        let ptr = unsafe { libcamera_camera_configuration_add_configuration(self.ptr.as_ptr()) };
        NonNull::new(ptr).map(|p| unsafe { StreamConfigurationRef::from_ptr(p) })
    }

    /// Append a new stream configuration by cloning an existing configuration.
    ///
    /// This mirrors libcamera's `addConfiguration(const StreamConfiguration&)` and produces
    /// a valid entry instead of an empty placeholder, allowing multi-stream configurations
    /// to validate successfully.
    pub fn add_configuration_like(
        &mut self,
        template: &StreamConfigurationRef<'_>,
    ) -> Option<StreamConfigurationRef<'_>> {
        let ptr =
            unsafe { libcamera_camera_configuration_add_configuration_from(self.ptr.as_ptr(), template.as_ptr()) };
        NonNull::new(ptr).map(|p| unsafe { StreamConfigurationRef::from_ptr(p) })
    }

    /// Append multiple stream configurations by cloning existing templates.
    ///
    /// Returns the newly appended configurations (in order) for further adjustment.
    pub fn add_configurations_like<'a>(
        &mut self,
        templates: &[&StreamConfigurationRef<'a>],
    ) -> Vec<StreamConfigurationRef<'_>> {
        let mut appended = Vec::with_capacity(templates.len());
        for tmpl in templates {
            let ptr =
                unsafe { libcamera_camera_configuration_add_configuration_from(self.ptr.as_ptr(), tmpl.as_ptr()) };
            if let Some(cfg) = NonNull::new(ptr).map(|p| unsafe { StreamConfigurationRef::from_ptr(p) }) {
                appended.push(cfg);
            }
        }
        appended
    }

    pub fn set_sensor_configuration(&mut self, mode: SensorConfiguration) {
        unsafe { libcamera_camera_set_sensor_configuration(self.ptr.as_ptr(), mode.item.as_ptr()) }
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

    /// Returns the desired orientation of the captured image.
    pub fn orientation(&self) -> Orientation {
        unsafe { libcamera_camera_configuration_get_orientation(self.ptr.as_ptr()) }
            .try_into()
            .unwrap()
    }

    /// Sets the desired orientation of the captured image.
    pub fn set_orientation(&mut self, orientation: Orientation) {
        unsafe { libcamera_camera_configuration_set_orientation(self.ptr.as_ptr(), orientation.into()) }
    }

    /// Returns the sensor configuration if one is set by the application or pipeline.
    pub fn sensor_configuration(&self) -> Option<SensorConfiguration> {
        let ptr = unsafe { libcamera_camera_configuration_get_sensor_configuration(self.ptr.as_ptr()) };
        NonNull::new(ptr).map(SensorConfiguration::from_ptr)
    }

    /// Re-validate and print stride/frame_size adjustments for each stream (helper for debugging).
    pub fn validate_and_log(&mut self) -> CameraConfigurationStatus {
        let status = self.validate();
        for i in 0..self.len() {
            if let Some(cfg) = self.get(i) {
                eprintln!(
                    "Stream {} after validate(): stride={}, frame_size={}",
                    i,
                    cfg.get_stride(),
                    cfg.get_frame_size()
                );
            }
        }
        status
    }

    /// Return the libcamera textual representation of this configuration.
    pub fn to_string_repr(&self) -> String {
        unsafe {
            let ptr = libcamera_camera_configuration_to_string(self.ptr.as_ptr());
            if ptr.is_null() {
                return String::new();
            }
            let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr.cast());
            s
        }
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
    pub(crate) _token: Option<std::sync::Arc<()>>,
}

impl<'d> Camera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
            _token: None,
        }
    }

    pub(crate) unsafe fn from_ptr_tracked(
        ptr: NonNull<libcamera_camera_t>,
        tracker: Option<std::sync::Arc<CameraTracker>>,
    ) -> Self {
        let token = tracker.map(|t| t.track());
        Self {
            ptr,
            _phantom: Default::default(),
            _token: token,
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

    /// Returns the set of active streams for this camera.
    pub fn streams(&self) -> Vec<Stream> {
        let set = unsafe { libcamera_camera_streams(self.ptr.as_ptr()) };
        if set.is_null() {
            return Vec::new();
        }
        let count = unsafe { libcamera_stream_set_size(set) };
        let streams = (0..count)
            .filter_map(|i| unsafe { NonNull::new(libcamera_stream_set_get(set, i)).map(|p| Stream::from_ptr(p)) })
            .collect();
        unsafe { libcamera_stream_set_destroy(set as *mut _) };
        streams
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

    /// Try roles in order and return the first generated configuration that succeeds.
    pub fn generate_first_supported_configuration(
        &self,
        roles: &[StreamRole],
    ) -> Option<(CameraConfiguration, StreamRole)> {
        roles
            .iter()
            .find_map(|role| self.generate_configuration(&[*role]).map(|cfg| (cfg, *role)))
    }

    /// Acquires exclusive rights to the camera, which allows changing configuration and capturing.
    pub fn acquire(&self) -> io::Result<ActiveCamera<'d>> {
        let ret = unsafe { libcamera_camera_acquire(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(-ret))
        } else {
            Ok(unsafe {
                ActiveCamera::from_ptr_with_token(
                    NonNull::new(libcamera_camera_copy(self.ptr.as_ptr())).unwrap(),
                    self._token.clone(),
                )
            })
        }
    }
}

impl Drop for Camera<'_> {
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

extern "C" fn camera_buffer_completed_cb(
    ptr: *mut core::ffi::c_void,
    req: *mut libcamera_request_t,
    fb: *mut libcamera_framebuffer_t,
) {
    let mut state = unsafe { &*(ptr as *const Mutex<ActiveCameraState<'_>>) }
        .lock()
        .unwrap();

    let (req_ptr, stream) = match state
        .requests
        .get_mut(&req)
        .and_then(|r| r.stream_for_buffer_ptr(fb).map(|s| (r as *mut Request, s)))
    {
        Some(v) => v,
        None => return,
    };

    if let Some(cb) = state.buffer_completed_cb.as_mut() {
        // Safety: req_ptr is valid while held in the map; we only borrow it temporarily.
        unsafe {
            cb(&mut *req_ptr, stream);
        }
    }
}

extern "C" fn camera_disconnected_cb(ptr: *mut core::ffi::c_void) {
    let mut state = unsafe { &*(ptr as *const Mutex<ActiveCameraState<'_>>) }
        .lock()
        .unwrap();
    if let Some(cb) = state.disconnected_cb.as_mut() {
        cb();
    }
}

#[derive(Default)]
struct ActiveCameraState<'d> {
    /// List of queued requests that are yet to be executed.
    /// Used to temporarily store [Request] before returning it back to the user.
    requests: HashMap<*mut libcamera_request_t, Request>,
    /// Callback for libcamera `requestCompleted` signal.
    request_completed_cb: Option<Box<dyn FnMut(Request) + Send + 'd>>,
    /// Callback for libcamera `bufferCompleted` signal.
    buffer_completed_cb: Option<BufferCompletedCb<'d>>,
    /// Callback for libcamera `disconnected` signal.
    disconnected_cb: Option<Box<dyn FnMut() + Send + 'd>>,
}

type BufferCompletedCb<'d> = Box<dyn FnMut(&mut Request, Stream) + Send + 'd>;

/// An active instance of a camera.
///
/// This gives exclusive access to the camera and allows capturing and modifying configuration.
///
/// Obtained by [Camera::acquire()].
pub struct ActiveCamera<'d> {
    cam: Camera<'d>,
    /// Handle to disconnect `requestCompleted` signal.
    request_completed_handle: *mut libcamera_callback_handle_t,
    /// Handle to disconnect `bufferCompleted` signal.
    buffer_completed_handle: *mut libcamera_callback_handle_t,
    /// Handle to disconnect `disconnected` signal.
    disconnected_handle: *mut libcamera_callback_handle_t,
    /// Internal state that is shared with callback handlers.
    state: Box<Mutex<ActiveCameraState<'d>>>,
    _token: Option<std::sync::Arc<()>>,
}

/// Lightweight buffer completion event for channel delivery.
#[derive(Debug, Clone, Copy)]
pub struct BufferCompletedEvent {
    pub stream: Stream,
    pub request_cookie: u64,
    pub request_sequence: u32,
    pub buffer_ptr: usize,
}

impl<'d> ActiveCamera<'d> {
    pub(crate) unsafe fn from_ptr_with_token(
        ptr: NonNull<libcamera_camera_t>,
        token: Option<std::sync::Arc<()>>,
    ) -> Self {
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
            buffer_completed_handle: core::ptr::null_mut(),
            disconnected_handle: core::ptr::null_mut(),
            state,
            _token: token,
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

    /// Sets a callback for per-buffer completion events.
    ///
    /// This fires for every buffer as libcamera emits `bufferCompleted`; the corresponding `Request` remains queued
    /// until the `requestCompleted` signal fires.
    pub fn on_buffer_completed(&mut self, cb: impl FnMut(&mut Request, Stream) + Send + 'd) {
        {
            let mut state = self.state.lock().unwrap();
            state.buffer_completed_cb = Some(Box::new(cb));
        }
        if self.buffer_completed_handle.is_null() {
            let data = self.state.as_mut() as *mut Mutex<ActiveCameraState<'_>> as *mut _;
            self.buffer_completed_handle = unsafe {
                libcamera_camera_buffer_completed_connect(self.ptr.as_ptr(), Some(camera_buffer_completed_cb), data)
            };
        }
    }

    /// Subscribe to request completed events via a channel (async-friendly).
    ///
    /// The returned receiver yields owned `Request`s as libcamera completes them.
    pub fn subscribe_request_completed(&mut self) -> std::sync::mpsc::Receiver<Request> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.on_request_completed(move |req| {
            let _ = tx.send(req);
        });
        rx
    }

    /// Subscribe to per-buffer completion events via a channel (async-friendly).
    ///
    /// The receiver yields lightweight `BufferCompletedEvent` snapshots; the underlying Request
    /// remains owned by libcamera until requestCompleted fires.
    pub fn subscribe_buffer_completed(&mut self) -> std::sync::mpsc::Receiver<BufferCompletedEvent> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.on_buffer_completed(move |req, stream| {
            let event = BufferCompletedEvent {
                stream,
                request_cookie: req.cookie(),
                request_sequence: req.sequence(),
                buffer_ptr: req.find_buffer(&stream).map_or(0, |p| p as usize),
            };
            let _ = tx.send(event);
        });
        rx
    }

    /// Sets a callback for camera disconnected events.
    pub fn on_disconnected(&mut self, cb: impl FnMut() + Send + 'd) {
        {
            let mut state = self.state.lock().unwrap();
            state.disconnected_cb = Some(Box::new(cb));
        }
        if self.disconnected_handle.is_null() {
            let data = self.state.as_mut() as *mut Mutex<ActiveCameraState<'_>> as *mut _;
            self.disconnected_handle =
                unsafe { libcamera_camera_disconnected_connect(self.ptr.as_ptr(), Some(camera_disconnected_cb), data) };
        }
    }

    /// Applies camera configuration.
    ///
    /// Default configuration can be obtained from [Camera::generate_configuration()] and then adjusted as needed.
    pub fn configure(&mut self, config: &mut CameraConfiguration) -> io::Result<()> {
        let ret = unsafe { libcamera_camera_configure(self.ptr.as_ptr(), config.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(-ret))
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
    pub fn queue_request(&self, req: Request) -> Result<(), (Request, io::Error)> {
        let ptr = req.ptr.as_ptr();
        // Keep the request alive locally until we know queuing succeeded.
        let mut pending = Some(req);
        let mut state = self.state.lock().unwrap();
        let ret = unsafe { libcamera_camera_queue_request(self.ptr.as_ptr(), ptr) };

        if ret < 0 {
            Err((pending.take().unwrap(), io::Error::from_raw_os_error(-ret)))
        } else {
            state.requests.insert(ptr, pending.take().unwrap());
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
            Err(io::Error::from_raw_os_error(-ret))
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
            Err(io::Error::from_raw_os_error(-ret))
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

impl DerefMut for ActiveCamera<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cam
    }
}

impl Drop for ActiveCamera<'_> {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_request_completed_disconnect(self.ptr.as_ptr(), self.request_completed_handle);
            if !self.buffer_completed_handle.is_null() {
                libcamera_camera_buffer_completed_disconnect(self.ptr.as_ptr(), self.buffer_completed_handle);
            }
            if !self.disconnected_handle.is_null() {
                libcamera_camera_disconnected_disconnect(self.ptr.as_ptr(), self.disconnected_handle);
            }
            libcamera_camera_stop(self.ptr.as_ptr());
            libcamera_camera_release(self.ptr.as_ptr());
        }
    }
}
