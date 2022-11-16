use std::marker::PhantomData;

use libcamera_sys::*;
use thiserror::Error;

use crate::{
    control_value::{ControlValue, ControlValueError},
    controls::{self, ControlId},
    properties::{self, PropertyId},
    utils::Immutable,
};

#[derive(Debug, Error)]
pub enum ControlError {
    #[error("Control id {0} not found")]
    NotFound(u32),
    #[error("Control value error: {0}")]
    ValueError(#[from] ControlValueError),
}

pub trait Control:
    Clone + Into<ControlValue> + TryFrom<ControlValue, Error = ControlValueError> + core::fmt::Debug
{
    const ID: u32;
}

pub trait DynControl: core::fmt::Debug {
    fn id(&self) -> u32;
    fn value(&self) -> ControlValue;
}

impl<T: Control> DynControl for T {
    fn id(&self) -> u32 {
        Self::ID
    }

    fn value(&self) -> ControlValue {
        self.clone().into()
    }
}

pub struct ControlInfoMapRef<'d> {
    _ptr: *mut libcamera_control_info_map_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlInfoMapRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_control_info_map_t) -> Immutable<Self> {
        Immutable(Self {
            _ptr: ptr as _,
            _phantom: Default::default(),
        })
    }
}

pub struct ControlListRef<'d> {
    pub(crate) ptr: *mut libcamera_control_list_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_control_list_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    pub(crate) unsafe fn from_ptr_mut(ptr: *mut libcamera_control_list_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn get<C: Control>(&self) -> Result<C, ControlError> {
        let val_ptr = unsafe { libcamera_control_list_get(self.ptr, C::ID as _) };

        if val_ptr.is_null() {
            Err(ControlError::NotFound(C::ID))
        } else {
            let val = unsafe { ControlValue::read(val_ptr) }?;
            Ok(C::try_from(val)?)
        }
    }

    pub fn set<C: Control>(&mut self, val: C) {
        let val_ptr = unsafe { libcamera_control_list_get(self.ptr, C::ID as _) };

        let ctrl_val: ControlValue = val.into();
        unsafe { ctrl_val.write(val_ptr) };
    }

    pub fn debug_controls(&self) -> ControlListRefDebugControls {
        ControlListRefDebugControls { list: self }
    }

    pub fn debug_properties(&self) -> ControlListRefDebugProperties {
        ControlListRefDebugProperties { list: self }
    }
}

impl<'d> IntoIterator for &'d ControlListRef<'d> {
    type Item = (u32, ControlValue);

    type IntoIter = ControlListRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        ControlListRefIterator {
            it: unsafe { libcamera_control_list_iter(self.ptr) },
            _phantom: Default::default(),
        }
    }
}

pub struct ControlListRefIterator<'d> {
    it: *mut libcamera_control_list_iter_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Iterator for ControlListRefIterator<'d> {
    type Item = (u32, ControlValue);

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { libcamera_control_list_iter_end(self.it) } {
            None
        } else {
            let id = unsafe { libcamera_control_list_iter_id(self.it) };
            let val_ptr = unsafe { libcamera_control_list_iter_value(self.it) };
            let val = unsafe { ControlValue::read(val_ptr) }.unwrap();

            unsafe { libcamera_control_list_iter_next(self.it) };

            Some((id, val))
        }
    }
}

pub struct ControlListRefDebugControls<'d> {
    list: &'d ControlListRef<'d>,
}

impl<'d> core::fmt::Debug for ControlListRefDebugControls<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (id, val) in self.list.into_iter() {
            match ControlId::try_from(id) {
                Ok(id) => match controls::make_dyn(id, val.clone()) {
                    Ok(val) => map.entry(&id, &val),
                    Err(_) => map.entry(&id, &val),
                },
                Err(_) => map.entry(&id, &val),
            };
        }
        map.finish()
    }
}

pub struct ControlListRefDebugProperties<'d> {
    list: &'d ControlListRef<'d>,
}

impl<'d> core::fmt::Debug for ControlListRefDebugProperties<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (id, val) in self.list.into_iter() {
            match PropertyId::try_from(id) {
                Ok(id) => match properties::make_dyn(id, val.clone()) {
                    Ok(val) => map.entry(&id, &val),
                    Err(_) => map.entry(&id, &val),
                },
                Err(_) => map.entry(&id, &val),
            };
        }
        map.finish()
    }
}
