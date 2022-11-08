use std::marker::PhantomData;

use libcamera_sys::*;

use crate::{ControlValue, ControlValueError};

pub trait Control: TryFrom<Self::T, Error = ControlValueError> + Into<Self::T> {
    const ID: u32;
    type T: ControlValue;
}

pub struct ControlInfoMapRef<'d> {
    _ptr: *mut libcamera_control_info_map_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlInfoMapRef<'d> {
    pub(crate) unsafe fn from_ptr(_ptr: *mut libcamera_control_info_map_t) -> Self {
        Self {
            _ptr,
            _phantom: Default::default(),
        }
    }
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

    pub fn get<C: Control>(&self) -> Result<C, ControlValueError> {
        let val_ptr = unsafe { libcamera_control_list_get(self.ptr, C::ID as _) };
        let val = unsafe { ControlValue::read(val_ptr) }?;
        C::try_from(val)
    }
}
