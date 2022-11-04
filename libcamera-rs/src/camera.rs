use std::ffi::CStr;

use libcamera_sys::*;

pub struct Camera {
    ptr: *mut libcamera_camera_t,
}

impl Camera {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_camera_t) -> Self {
        Self { ptr }
    }

    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_id(self.ptr)) }
            .to_str()
            .unwrap()
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        unsafe { libcamera_camera_destroy(self.ptr) }
    }
}
