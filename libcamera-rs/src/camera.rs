use std::{ffi::CStr, marker::PhantomData};

use libcamera_sys::*;

pub struct Camera<'d> {
    ptr: *mut libcamera_camera_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Camera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_camera_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(libcamera_camera_id(self.ptr)) }
            .to_str()
            .unwrap()
    }
}

impl<'d> Drop for Camera<'d> {
    fn drop(&mut self) {
        unsafe { libcamera_camera_destroy(self.ptr) }
    }
}
