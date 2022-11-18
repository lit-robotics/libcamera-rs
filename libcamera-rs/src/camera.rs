use std::{
    ffi::CStr,
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use libcamera_sys::*;

use crate::{
    control::{ControlInfoMapRef, ControlListRef, PropertyListRef},
    request::Request,
    stream::{StreamConfigurationRef, StreamRole},
    utils::Immutable,
};

#[derive(Debug, Clone, Copy)]
pub enum CameraConfigurationStatus {
    Valid,
    Adjusted,
    Invalid,
}

impl CameraConfigurationStatus {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid => true,
            _ => false,
        }
    }

    pub fn is_adjusted(&self) -> bool {
        match self {
            Self::Adjusted => true,
            _ => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Self::Invalid => true,
            _ => false,
        }
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

pub struct CameraConfiguration {
    ptr: NonNull<libcamera_camera_configuration_t>,
}

impl CameraConfiguration {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_configuration_t>) -> Self {
        Self { ptr }
    }

    pub fn get(&self, index: usize) -> Option<Immutable<StreamConfigurationRef>> {
        let ptr = unsafe { libcamera_camera_configuration_at(self.ptr.as_ptr(), index as _) };
        NonNull::new(ptr).map(|p| Immutable(unsafe { StreamConfigurationRef::from_ptr(p) }))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<StreamConfigurationRef> {
        let ptr = unsafe { libcamera_camera_configuration_at(self.ptr.as_ptr(), index as _) };
        NonNull::new(ptr).map(|p| unsafe { StreamConfigurationRef::from_ptr(p) })
    }

    pub fn len(&self) -> usize {
        return unsafe { libcamera_camera_configuration_size(self.ptr.as_ptr()) } as _;
    }

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

    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_id(self.ptr.as_ptr())) }
            .to_str()
            .unwrap()
    }

    pub fn controls(&self) -> Immutable<ControlInfoMapRef> {
        Immutable(unsafe {
            ControlInfoMapRef::from_ptr(NonNull::new(libcamera_camera_controls(self.ptr.as_ptr()).cast_mut()).unwrap())
        })
    }

    pub fn properties(&self) -> Immutable<PropertyListRef> {
        Immutable(unsafe {
            PropertyListRef::from_ptr(NonNull::new(libcamera_camera_properties(self.ptr.as_ptr()).cast_mut()).unwrap())
        })
    }

    pub fn generate_configuration(&self, roles: &[StreamRole]) -> Option<CameraConfiguration> {
        let roles: Vec<libcamera_stream_role::Type> = roles.iter().map(|r| (*r).into()).collect();
        let cfg =
            unsafe { libcamera_camera_generate_configuration(self.ptr.as_ptr(), roles.as_ptr(), roles.len() as _) };
        NonNull::new(cfg).map(|p| unsafe { CameraConfiguration::from_ptr(p) })
    }

    pub fn acquire(&self) -> io::Result<ActiveCamera> {
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

extern "C" fn request_completed_cb(ptr: *mut core::ffi::c_void, req: *mut libcamera_request_t) {
    let cb: &mut Box<dyn FnMut(Request) + Send + Sync> = unsafe { core::mem::transmute(ptr) };
    let req = unsafe { Request::from_ptr(NonNull::new(req).unwrap()) };
    cb(req);
}

/// A [Camera] with exclusive access granted by [Camera::acquire()].
pub struct ActiveCamera<'d> {
    cam: Camera<'d>,
    request_completed_handle: Option<(
        *mut libcamera_callback_handle_t,
        *mut Box<dyn FnMut(Request) + Send + 'd>,
    )>,
}

impl<'d> ActiveCamera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_t>) -> Self {
        Self {
            cam: Camera::from_ptr(ptr),
            request_completed_handle: None,
        }
    }

    pub fn on_request_completed(&mut self, cb: impl FnMut(Request) + Send + 'd) {
        self.disconnect_request_completed();

        let cb: Box<Box<dyn FnMut(Request) + Send>> = Box::new(Box::new(cb));
        let cb_ptr = Box::into_raw(cb);

        self.request_completed_handle = Some((
            unsafe {
                libcamera_camera_request_completed_connect(
                    self.ptr.as_ptr(),
                    Some(request_completed_cb),
                    cb_ptr as *mut _,
                )
            },
            cb_ptr,
        ));
    }

    pub fn disconnect_request_completed(&mut self) {
        if let Some((handle, cb_ptr)) = self.request_completed_handle {
            unsafe { libcamera_camera_request_completed_disconnect(self.ptr.as_ptr(), handle) };
            unsafe { drop(Box::from_raw(cb_ptr)) };
        }
    }

    pub fn configure(&mut self, config: &mut CameraConfiguration) -> io::Result<()> {
        let ret = unsafe { libcamera_camera_configure(self.ptr.as_ptr(), config.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn create_request(&mut self, cookie: Option<u64>) -> Option<Request> {
        let req = unsafe { libcamera_camera_create_request(self.ptr.as_ptr(), cookie.unwrap_or(0)) };
        NonNull::new(req).map(|p| unsafe { Request::from_ptr(p) })
    }

    pub fn queue_request(&mut self, req: Request) -> io::Result<()> {
        let ret = unsafe { libcamera_camera_queue_request(self.ptr.as_ptr(), req.ptr.as_ptr()) };

        // Request will be recreated in callback from raw pointer
        core::mem::forget(req);

        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn start(&mut self, controls: Option<ControlListRef>) -> io::Result<()> {
        let ctrl_ptr = controls.map(|c| c.ptr.as_ptr()).unwrap_or(core::ptr::null_mut());
        let ret = unsafe { libcamera_camera_start(self.ptr.as_ptr(), ctrl_ptr) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

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
        self.disconnect_request_completed();

        unsafe {
            libcamera_camera_stop(self.ptr.as_ptr());
            libcamera_camera_release(self.ptr.as_ptr());
        }
    }
}
