use std::{
    ffi::CStr,
    io::{Error, Result},
    marker::PhantomData,
};

use libcamera_sys::*;

use crate::{utils::Immutable, ControlInfoMapRef, ControlListRef};

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

    pub fn controls(&self) -> Immutable<ControlInfoMapRef> {
        unsafe { Immutable(ControlInfoMapRef::from_ptr(libcamera_camera_controls(self.ptr) as _)) }
    }

    pub fn properties(&self) -> Immutable<ControlListRef> {
        unsafe { Immutable(ControlListRef::from_ptr(libcamera_camera_properties(self.ptr) as _)) }
    }

    pub fn acquire(&self) -> Result<ActiveCamera> {
        let ret = unsafe { libcamera_camera_acquire(self.ptr) };
        if ret < 0 {
            Err(Error::from_raw_os_error(ret))
        } else {
            Ok(unsafe { ActiveCamera::from_ptr(libcamera_camera_copy(self.ptr)) })
        }
    }
}

impl<'d> Drop for Camera<'d> {
    fn drop(&mut self) {
        unsafe { libcamera_camera_destroy(self.ptr) }
    }
}

/// A [Camera] with exclusive access granted by [Camera::acquire()].
pub struct ActiveCamera<'d> {
    ptr: *mut libcamera_camera_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ActiveCamera<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_camera_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }
}

impl<'d> Drop for ActiveCamera<'d> {
    fn drop(&mut self) {
        unsafe {
            libcamera_camera_stop(self.ptr);
            libcamera_camera_release(self.ptr);
            libcamera_camera_destroy(self.ptr);
        }
    }
}
