use std::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;

use crate::camera::Camera;

pub struct CameraManager {
    ptr: NonNull<libcamera_camera_manager_t>,
}

impl CameraManager {
    pub fn new() -> std::io::Result<Self> {
        let ptr = NonNull::new(unsafe { libcamera_camera_manager_create() }).unwrap();
        let ret = unsafe { libcamera_camera_manager_start(ptr.as_ptr()) };

        if ret < 0 {
            Err(std::io::Error::from_raw_os_error(ret))
        } else {
            Ok(CameraManager { ptr })
        }
    }

    pub fn version(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_manager_version(self.ptr.as_ptr())) }
            .to_str()
            .unwrap()
    }

    pub fn cameras(&self) -> CameraList {
        unsafe { CameraList::from_ptr(NonNull::new(libcamera_camera_manager_cameras(self.ptr.as_ptr())).unwrap()) }
    }
}

impl Drop for CameraManager {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_manager_stop(self.ptr.as_ptr());
            libcamera_camera_manager_destroy(self.ptr.as_ptr());
        }
    }
}

pub struct CameraList<'d> {
    ptr: NonNull<libcamera_camera_list_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> CameraList<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_camera_list_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_camera_list_size(self.ptr.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Option<Camera> {
        let cam_ptr = unsafe { libcamera_camera_list_get(self.ptr.as_ptr(), index as _) };
        NonNull::new(cam_ptr).map(|p| unsafe { Camera::from_ptr(p) })
    }
}

impl<'d> Drop for CameraList<'d> {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_list_destroy(self.ptr.as_ptr());
        }
    }
}
