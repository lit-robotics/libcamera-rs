use std::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;

use crate::camera::Camera;

/// Camera manager used to enumerate available cameras in the system.
pub struct CameraManager {
    ptr: NonNull<libcamera_camera_manager_t>,
}

impl CameraManager {
    /// Initializes `libcamera` and creates [Self].
    pub fn new() -> std::io::Result<Self> {
        let ptr = NonNull::new(unsafe { libcamera_camera_manager_create() }).unwrap();
        let ret = unsafe { libcamera_camera_manager_start(ptr.as_ptr()) };

        if ret < 0 {
            Err(std::io::Error::from_raw_os_error(ret))
        } else {
            Ok(CameraManager { ptr })
        }
    }

    /// Returns version string of the linked libcamera.
    pub fn version(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_manager_version(self.ptr.as_ptr())) }
            .to_str()
            .unwrap()
    }

    /// Enumerates cameras within the system.
    pub fn cameras(&self) -> CameraList<'_> {
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

    /// Number of cameras
    pub fn len(&self) -> usize {
        unsafe { libcamera_camera_list_size(self.ptr.as_ptr()) as usize }
    }

    /// Returns `true` if there are no cameras available
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns camera at a given index.
    ///
    /// Returns [None] if index is out of range of available cameras.
    pub fn get(&self, index: usize) -> Option<Camera<'_>> {
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
