use std::marker::PhantomData;

use libcamera_sys::*;

pub trait Control: TryFrom<Self::T> + Into<Self::T> {
    type T;
}

pub struct ControlInfoMap {
    ptr: *mut libcamera_control_info_map_t,
}

pub struct ControlInfoMapRef<'d> {
    ptr: *mut libcamera_control_info_map_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlInfoMapRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_control_info_map_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }
}

pub struct ControlList {
    ptr: *mut libcamera_control_list_t,
}

pub struct ControlListRef<'d> {
    ptr: *mut libcamera_control_list_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_control_list_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }
}

pub enum ControlId {
    AeEnable,
    AeLocked,
}
