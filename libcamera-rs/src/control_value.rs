use libcamera_sys::*;
use thiserror::Error;

use crate::geometry::{Rectangle, Size};

#[derive(Error, Debug)]
pub enum ControlValueError {
    /// Control value type does not match the one being read/written
    #[error("Expected type {expected}, found {found}")]
    InvalidType { expected: u32, found: u32 },
    /// Control value type is correct, but conversion failed (i.e. invalid utf8 string)
    #[error("Control contains a valid type, but data conversion failed")]
    InvalidData,
    /// Control value dimensionality mismatch
    #[error("Expected {expected} elements, found {found}")]
    InvalidLength { expected: usize, found: usize },
}

pub trait ControlValue: Sized {
    const LIBCAMERA_TYPE: libcamera_control_type::Type;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError>;
    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError>;

    unsafe fn check_type(val: *const libcamera_control_value_t) -> Result<(), ControlValueError> {
        let found = unsafe { libcamera_control_value_type(val) };
        if found != Self::LIBCAMERA_TYPE as _ {
            Err(ControlValueError::InvalidType {
                expected: Self::LIBCAMERA_TYPE as _,
                found,
            })
        } else {
            Ok(())
        }
    }

    unsafe fn is_array(val: *const libcamera_control_value_t) -> bool {
        return unsafe { libcamera_control_value_is_array(val) };
    }

    unsafe fn num_elements(val: *const libcamera_control_value_t) -> usize {
        return unsafe { libcamera_control_value_num_elements(val) }.try_into().unwrap();
    }
}

impl ControlValue for bool {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_BOOL;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        Ok(unsafe { *(libcamera_control_value_get(val) as *const bool) })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, self as *const bool as _, 1);
        }

        Ok(())
    }
}

impl ControlValue for i32 {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_INT32;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        Ok(unsafe { *(libcamera_control_value_get(val) as *const i32) })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, self as *const i32 as _, 1);
        }

        Ok(())
    }
}

impl ControlValue for i64 {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_INT64;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        Ok(unsafe { *(libcamera_control_value_get(val) as *const i64) })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, self as *const i64 as _, 1);
        }

        Ok(())
    }
}

impl ControlValue for f32 {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_FLOAT;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        Ok(unsafe { *(libcamera_control_value_get(val) as *const f32) })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, self as *const f32 as _, 1);
        }

        Ok(())
    }
}

impl ControlValue for String {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_STRING;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if !Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        let len = unsafe { Self::num_elements(val) };
        let data = unsafe { core::slice::from_raw_parts(libcamera_control_value_get(val) as *const u8, len) };
        let val = core::str::from_utf8(data).unwrap().to_string();

        Ok(val)
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, self.as_ptr() as _, self.len() as _);
        }

        Ok(())
    }
}

impl ControlValue for Rectangle {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_RECTANGLE;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        let vals = unsafe { core::slice::from_raw_parts(libcamera_control_value_get(val) as *const i32, 4) };

        Ok(Self {
            x: vals[0],
            y: vals[1],
            width: vals[2] as u32,
            height: vals[3] as u32,
        })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        let data = [self.x, self.y, self.width as i32, self.height as i32];

        unsafe {
            libcamera_control_value_set(val, Self::LIBCAMERA_TYPE, &data as *const i32 as _, 1);
        }

        Ok(())
    }
}

impl ControlValue for Size {
    const LIBCAMERA_TYPE: libcamera_control_type::Type = libcamera_control_type::LIBCAMERA_CONTROL_TYPE_SIZE;

    unsafe fn read(val: *const libcamera_control_value_t) -> Result<Self, ControlValueError> {
        Self::check_type(val)?;

        if Self::is_array(val) {
            return Err(ControlValueError::InvalidData);
        }

        let vals = unsafe { core::slice::from_raw_parts(libcamera_control_value_get(val) as *const u32, 2) };

        Ok(Self {
            width: vals[0] as u32,
            height: vals[1] as u32,
        })
    }

    unsafe fn write(&self, val: *mut libcamera_control_value_t) -> Result<(), ControlValueError> {
        unsafe {
            libcamera_control_value_set(
                val,
                Self::LIBCAMERA_TYPE,
                &[self.width, self.height] as *const u32 as _,
                1,
            );
        }

        Ok(())
    }
}
