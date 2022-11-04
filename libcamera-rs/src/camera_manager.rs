use std::ffi::CStr;

use libcamera_sys::*;

pub struct CameraManager {
    ptr: *mut libcamera_camera_manager_t,
}

impl CameraManager {
    pub fn new() -> std::io::Result<Self> {
        let ptr = unsafe { libcamera_camera_manager_create() };
        let ret = unsafe { libcamera_camera_manager_start(ptr) };

        if ret < 0 {
            Err(std::io::Error::from_raw_os_error(ret))
        } else {
            Ok(CameraManager { ptr })
        }
    }

    pub fn version(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_manager_version(self.ptr)) }
            .to_str()
            .unwrap()
    }
}

impl Drop for CameraManager {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_manager_stop(self.ptr);
            libcamera_camera_manager_destroy(self.ptr);
        }
    }
}
