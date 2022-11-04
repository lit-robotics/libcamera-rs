use std::{ffi::CStr, marker::PhantomData};

use libcamera_sys::*;

use crate::Camera;

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

    pub fn cameras(&self) -> CameraList {
        unsafe { CameraList::from_ptr(libcamera_camera_manager_cameras(self.ptr)) }
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

pub struct CameraList<'d> {
    ptr: *mut libcamera_camera_list_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> CameraList<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_camera_list_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_camera_list_size(self.ptr) as usize }
    }

    pub fn get(&self, index: usize) -> Option<Camera> {
        let cam_ptr = unsafe { libcamera_camera_list_get(self.ptr, index as _) };

        if cam_ptr.is_null() {
            None
        } else {
            Some(unsafe { Camera::from_ptr(cam_ptr) })
        }
    }
}

impl<'d> Drop for CameraList<'d> {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_list_destroy(self.ptr);
        }
    }
}
