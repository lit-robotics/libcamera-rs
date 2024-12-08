use std::{marker::PhantomData, ptr::NonNull, slice};

use libcamera_sys::*;
use thiserror::Error;

use crate::{
    control_value::{ControlValue, ControlValueError},
    controls::{self, ControlId},
    properties::{self, PropertyId},
    utils::{UniquePtr, UniquePtrTarget},
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

#[repr(transparent)]
pub struct ControlInfo(libcamera_control_info_t);

impl ControlInfo {
    pub(crate) unsafe fn from_ptr<'a>(ptr: NonNull<libcamera_control_info_t>) -> &'a mut Self {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &mut *(ptr.as_ptr() as *mut Self)
    }

    pub(crate) fn ptr(&self) -> *const libcamera_control_info_t {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &self.0 as *const libcamera_control_info_t
    }

    pub fn min(&self) -> ControlValue {
        unsafe {
            ControlValue::read(NonNull::new(libcamera_control_info_min(self.ptr().cast_mut()).cast_mut()).unwrap())
                .unwrap()
        }
    }

    pub fn max(&self) -> ControlValue {
        unsafe {
            ControlValue::read(NonNull::new(libcamera_control_info_max(self.ptr().cast_mut()).cast_mut()).unwrap())
                .unwrap()
        }
    }

    pub fn def(&self) -> ControlValue {
        unsafe {
            ControlValue::read(NonNull::new(libcamera_control_info_def(self.ptr().cast_mut()).cast_mut()).unwrap())
                .unwrap()
        }
    }

    pub fn values(&self) -> Vec<ControlValue> {
        unsafe {
            let mut size: usize = 0;
            let values_ptr = libcamera_control_info_values(self.ptr(), &mut size as *mut usize);

            if values_ptr.is_null() || size == 0 {
                return Vec::new();
            }

            let raw_slice = slice::from_raw_parts(values_ptr, size);

            let mut control_values = Vec::with_capacity(size);
            for raw_val in raw_slice {
                let val_ptr = NonNull::new(raw_val as *const libcamera_control_value_t as *mut libcamera_control_value_t)
                    .expect("Received a null pointer in raw_slice");

                match ControlValue::read(val_ptr) {
                    Ok(control_val) => control_values.push(control_val),
                    Err(e) => {
                        eprintln!("Failed to read ControlValue: {:?}", e);
                    }
                }
            }

            control_values
        }
    }

}

#[repr(transparent)]
pub struct ControlInfoMap(libcamera_control_info_map_t);

impl ControlInfoMap {
    pub(crate) unsafe fn from_ptr<'a>(ptr: NonNull<libcamera_control_info_map_t>) -> &'a mut Self {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &mut *(ptr.as_ptr() as *mut Self)
    }

    pub(crate) fn ptr(&self) -> *const libcamera_control_info_map_t {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &self.0 as *const libcamera_control_info_map_t
    }

    pub fn at(&self, key: u32) -> &ControlInfo {
        unsafe {
            let ptr = libcamera_control_info_map_at(self.ptr().cast_mut(), key);
            println!("ptr {:?}", ptr);
            ControlInfo::from_ptr(NonNull::new(ptr.cast_mut()).unwrap())
        }
    }

    pub fn count(&self, key: u32) -> usize {
        unsafe { libcamera_control_info_map_count(self.ptr().cast_mut(), key) }
    }

    pub fn find(&self, key: u32) -> &ControlInfo {
        unsafe {
            ControlInfo::from_ptr(
                NonNull::new(libcamera_control_info_map_find(self.ptr().cast_mut(), key).cast_mut()).unwrap(),
            )
        }
    }

    pub fn size(&self) -> usize {
        unsafe { libcamera_control_info_map_size(self.ptr().cast_mut()) }
    }
}

impl<'a> IntoIterator for &'a ControlInfoMap {
    type Item = (u32, &'a ControlInfo);
    type IntoIter = ControlInfoMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ControlInfoMapIter::new(self).expect("Failed to create ControlInfoMap iterator")
    }
}

#[repr(transparent)]
pub struct ControlList(libcamera_control_list_t);

impl UniquePtrTarget for ControlList {
    unsafe fn ptr_new() -> *mut Self {
        libcamera_control_list_create() as *mut Self
    }

    unsafe fn ptr_drop(ptr: *mut Self) {
        libcamera_control_list_destroy(ptr as *mut libcamera_control_list_t)
    }
}

impl ControlList {
    pub fn new() -> UniquePtr<Self> {
        UniquePtr::new()
    }

    pub(crate) unsafe fn from_ptr<'a>(ptr: NonNull<libcamera_control_list_t>) -> &'a mut Self {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &mut *(ptr.as_ptr() as *mut Self)
    }

    pub(crate) fn ptr(&self) -> *const libcamera_control_list_t {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &self.0 as *const libcamera_control_list_t
    }

    pub fn get<C: Control>(&self) -> Result<C, ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr().cast_mut(), C::ID as _).cast_mut() })
            .ok_or(ControlError::NotFound(C::ID))?;

        let val = unsafe { ControlValue::read(val_ptr) }?;
        Ok(C::try_from(val)?)
    }

    /// Sets control value.
    ///
    /// This can fail if control is not supported by the camera, but due to libcamera API limitations an error will not
    /// be returned. Use [ControlList::get] if you need to ensure that value was set.
    pub fn set<C: Control>(&mut self, val: C) -> Result<(), ControlError> {
        let ctrl_val: ControlValue = val.into();

        unsafe {
            let val_ptr = NonNull::new(libcamera_control_value_create()).unwrap();
            ctrl_val.write(val_ptr);
            libcamera_control_list_set(self.ptr().cast_mut(), C::ID as _, val_ptr.as_ptr());
            libcamera_control_value_destroy(val_ptr.as_ptr());
        }

        Ok(())
    }
}

impl<'d> IntoIterator for &'d ControlList {
    type Item = (u32, ControlValue);

    type IntoIter = ControlListRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        ControlListRefIterator {
            it: NonNull::new(unsafe { libcamera_control_list_iter(self.ptr().cast_mut()) }).unwrap(),
            _phantom: Default::default(),
        }
    }
}

impl core::fmt::Debug for ControlList {
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

#[repr(transparent)]
pub struct PropertyList(libcamera_control_list_t);

impl PropertyList {
    pub(crate) unsafe fn from_ptr<'a>(ptr: NonNull<libcamera_control_list_t>) -> &'a mut Self {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &mut *(ptr.as_ptr() as *mut Self)
    }

    pub(crate) fn ptr(&self) -> *const libcamera_control_list_t {
        // Safety: we can cast it because of `#[repr(transparent)]`
        &self.0 as *const libcamera_control_list_t
    }

    pub fn get<C: Property>(&self) -> Result<C, ControlError> {
        let val_ptr = NonNull::new(unsafe { libcamera_control_list_get(self.ptr().cast_mut(), C::ID as _).cast_mut() })
            .ok_or(ControlError::NotFound(C::ID))?;

        let val = unsafe { ControlValue::read(val_ptr) }?;
        Ok(C::try_from(val)?)
    }

    /// Sets property value.
    ///
    /// This can fail if property is not supported by the camera, but due to libcamera API limitations an error will not
    /// be returned. Use [PropertyList::get] if you need to ensure that value was set.
    pub fn set<C: Property>(&mut self, val: C) -> Result<(), ControlError> {
        let ctrl_val: ControlValue = val.into();

        unsafe {
            let val_ptr = NonNull::new(libcamera_control_value_create()).unwrap();
            ctrl_val.write(val_ptr);
            libcamera_control_list_set(self.ptr().cast_mut(), C::ID as _, val_ptr.as_ptr());
            libcamera_control_value_destroy(val_ptr.as_ptr());
        }

        Ok(())
    }
}

impl<'d> IntoIterator for &'d PropertyList {
    type Item = (u32, ControlValue);

    type IntoIter = ControlListRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        ControlListRefIterator {
            it: NonNull::new(unsafe { libcamera_control_list_iter(self.ptr().cast_mut()) }).unwrap(),
            _phantom: Default::default(),
        }
    }
}

impl core::fmt::Debug for PropertyList {
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

pub struct ControlInfoMapIter<'a> {
    iter: *mut libcamera_control_info_map_iter_t,
    marker: PhantomData<&'a libcamera_control_info_map_t>,
}

impl<'a> ControlInfoMapIter<'a> {
    pub fn new(map: &'a ControlInfoMap) -> Option<Self> {
        unsafe {
            let iter = libcamera_control_info_map_iter_create(map.ptr());
            if iter.is_null() {
                None
            } else {
                Some(ControlInfoMapIter {
                    iter,
                    marker: PhantomData,
                })
            }
        }
    }

    pub fn next(&mut self) -> Option<(u32, &'a ControlInfo)> {
        unsafe {
            if libcamera_control_info_map_iter_has_next(self.iter) {
                let key = libcamera_control_info_map_iter_key(self.iter);
                let value_ptr = libcamera_control_info_map_iter_value(self.iter);
                if value_ptr.is_null() {
                    None
                } else {
                    let control_info = &*(value_ptr as *const ControlInfo);
                    libcamera_control_info_map_iter_next(self.iter);
                    Some((key, control_info))
                }
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for ControlInfoMapIter<'a> {
    type Item = (u32, &'a ControlInfo);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a> Drop for ControlInfoMapIter<'a> {
    fn drop(&mut self) {
        unsafe {
            libcamera_control_info_map_iter_destroy(self.iter);
        }
    }
}
