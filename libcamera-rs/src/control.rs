use std::{marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;
use thiserror::Error;

use crate::{
    control_value::{ControlValue, ControlValueError},
    controls::{self, ControlId},
    properties::{self, PropertyId},
};

#[derive(Debug, Error)]
pub enum ControlError {
    #[error("Control id {0} not found")]
    NotFound(u32),
    #[error("Control value error: {0}")]
    ValueError(#[from] ControlValueError),
}

pub trait ControlEntry:
    Clone + Into<ControlValue> + TryFrom<ControlValue, Error = ControlValueError> + core::fmt::Debug
{
    const ID: u32;
}

pub trait Control: ControlEntry {}
pub trait Property: ControlEntry {}

/// Dynamic Control, which does not have strong typing.
pub trait DynControlEntry: core::fmt::Debug {
    fn id(&self) -> u32;
    fn value(&self) -> ControlValue;
}

impl<T: ControlEntry> DynControlEntry for T {
    fn id(&self) -> u32 {
        Self::ID
    }

    fn value(&self) -> ControlValue {
        self.clone().into()
    }
}

pub struct ControlInfoMapRef<'d> {
    _ptr: NonNull<libcamera_control_info_map_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlInfoMapRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_control_info_map_t>) -> Self {
        Self {
            _ptr: ptr,
            _phantom: Default::default(),
        }
    }
}

pub struct ControlListRef<'d> {
    pub(crate) ptr: NonNull<libcamera_control_list_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> ControlListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_control_list_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn get<C: Control>(&self) -> Result<C, ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr.as_ptr(), C::ID as _) })
            .ok_or(ControlError::NotFound(C::ID))?;

        let val = unsafe { ControlValue::read(val_ptr) }?;
        Ok(C::try_from(val)?)
    }

    pub fn set<C: Control>(&mut self, val: C) -> Result<(), ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr.as_ptr(), C::ID as _) })
            .ok_or(ControlError::NotFound(C::ID))?;

        let ctrl_val: ControlValue = val.into();
        unsafe { ctrl_val.write(val_ptr) };
        Ok(())
    }
}

impl<'d> IntoIterator for &'d ControlListRef<'d> {
    type Item = (u32, ControlValue);

    type IntoIter = ControlListRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        ControlListRefIterator {
            it: NonNull::new(unsafe { libcamera_control_list_iter(self.ptr.as_ptr()) }).unwrap(),
            _phantom: Default::default(),
        }
    }
}

impl<'d> core::fmt::Debug for ControlListRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (id, val) in self.into_iter() {
            match ControlId::try_from(id) {
                // Try to parse dynamic control, if not successful, just display the raw ControlValue
                Ok(id) => match controls::make_dyn(id, val.clone()) {
                    Ok(val) => map.entry(&id, &val),
                    Err(_) => map.entry(&id, &val),
                },
                // If ControlId is unknown just use u32 as key
                Err(_) => map.entry(&id, &val),
            };
        }
        map.finish()
    }
}

pub struct PropertyListRef<'d> {
    pub(crate) ptr: NonNull<libcamera_control_list_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> PropertyListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_control_list_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn get<C: Property>(&self) -> Result<C, ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr.as_ptr(), C::ID as _) })
            .ok_or(ControlError::NotFound(C::ID))?;

        let val = unsafe { ControlValue::read(val_ptr) }?;
        Ok(C::try_from(val)?)
    }

    pub fn set<C: Property>(&mut self, val: C) -> Result<(), ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr.as_ptr(), C::ID as _) })
            .ok_or(ControlError::NotFound(C::ID))?;

        let ctrl_val: ControlValue = val.into();
        unsafe { ctrl_val.write(val_ptr) };
        Ok(())
    }
}

impl<'d> IntoIterator for &'d PropertyListRef<'d> {
    type Item = (u32, ControlValue);

    type IntoIter = ControlListRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        ControlListRefIterator {
            it: NonNull::new(unsafe { libcamera_control_list_iter(self.ptr.as_ptr()) }).unwrap(),
            _phantom: Default::default(),
        }
    }
}

impl<'d> core::fmt::Debug for PropertyListRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (id, val) in self.into_iter() {
            match PropertyId::try_from(id) {
                // Try to parse dynamic property, if not successful, just display the raw ControlValue
                Ok(id) => match properties::make_dyn(id, val.clone()) {
                    Ok(val) => map.entry(&id, &val),
                    Err(_) => map.entry(&id, &val),
                },
                // If PropertyId is unknown just use u32 as key
                Err(_) => map.entry(&id, &val),
            };
        }
        map.finish()
    }
}

pub struct ControlListRefIterator<'d> {
    it: NonNull<libcamera_control_list_iter_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Iterator for ControlListRefIterator<'d> {
    type Item = (u32, ControlValue);

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { libcamera_control_list_iter_end(self.it.as_ptr()) } {
            None
        } else {
            let id = unsafe { libcamera_control_list_iter_id(self.it.as_ptr()) };
            let val_ptr =
                NonNull::new(unsafe { libcamera_control_list_iter_value(self.it.as_ptr()).cast_mut() }).unwrap();
            let val = unsafe { ControlValue::read(val_ptr) }.unwrap();

            unsafe { libcamera_control_list_iter_next(self.it.as_ptr()) };

            Some((id, val))
        }
    }
}

impl<'d> Drop for ControlListRefIterator<'d> {
    fn drop(&mut self) {
        unsafe { libcamera_control_list_iter_destroy(self.it.as_ptr()) }
    }
}
