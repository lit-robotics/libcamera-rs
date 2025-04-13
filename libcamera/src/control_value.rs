use std::ptr::NonNull;

use libcamera_sys::*;
use smallvec::{smallvec, SmallVec};
use thiserror::Error;

use crate::geometry::{Point, Rectangle, Size};

#[derive(Error, Debug)]
pub enum ControlValueError {
    /// Control value type does not match the one being read/written
    #[error("Expected type {expected}, found {found}")]
    InvalidType { expected: u32, found: u32 },
    /// Control value type is not recognized
    #[error("Unknown control type {0}")]
    UnknownType(u32),
    /// Control value dimensionality mismatch
    #[error("Expected {expected} elements, found {found}")]
    InvalidLength { expected: usize, found: usize },
    /// Control value type is correct, but it could not be converted into enum variant
    #[error("Unknown enum variant {0:?}")]
    UnknownVariant(ControlValue),
}

/// A value of a control or a property.
#[derive(Debug, Clone)]
pub enum ControlValue {
    None,
    Bool(SmallVec<[bool; 1]>),
    Byte(SmallVec<[u8; 1]>),
    Int32(SmallVec<[i32; 1]>),
    Int64(SmallVec<[i64; 1]>),
    Float(SmallVec<[f32; 1]>),
    String(String),
    Rectangle(SmallVec<[Rectangle; 1]>),
    Size(SmallVec<[Size; 1]>),
    Point(SmallVec<[Point; 1]>),
}

macro_rules! impl_control_value {
    ($p:path, $type:ty) => {
        impl From<$type> for ControlValue {
            fn from(val: $type) -> Self {
                $p(smallvec![val])
            }
        }

        impl TryFrom<ControlValue> for $type {
            type Error = ControlValueError;

            fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
                match value {
                    $p(mut val) => {
                        if val.len() == 1 {
                            Ok(val.pop().unwrap())
                        } else {
                            Err(ControlValueError::InvalidLength {
                                expected: 1,
                                found: val.len(),
                            })
                        }
                    }
                    _ => Err(ControlValueError::InvalidType {
                        // not really efficient, but eh, only on error
                        expected: $p(Default::default()).ty(),
                        found: value.ty(),
                    }),
                }
            }
        }
    };
}

impl_control_value!(ControlValue::Bool, bool);
impl_control_value!(ControlValue::Byte, u8);
impl_control_value!(ControlValue::Int32, i32);
impl_control_value!(ControlValue::Int64, i64);
impl_control_value!(ControlValue::Float, f32);
impl_control_value!(ControlValue::Rectangle, Rectangle);
impl_control_value!(ControlValue::Size, Size);
impl_control_value!(ControlValue::Point, Point);

macro_rules! impl_control_value_vec {
    ($p:path, $type:ty) => {
        impl From<Vec<$type>> for ControlValue {
            fn from(val: Vec<$type>) -> Self {
                $p(SmallVec::from_vec(val))
            }
        }

        impl TryFrom<ControlValue> for Vec<$type> {
            type Error = ControlValueError;

            fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
                match value {
                    $p(val) => Ok(val.into_vec()),
                    _ => Err(ControlValueError::InvalidType {
                        // not really efficient, but eh, only on error
                        expected: $p(Default::default()).ty(),
                        found: value.ty(),
                    }),
                }
            }
        }
    };
}

impl_control_value_vec!(ControlValue::Bool, bool);
impl_control_value_vec!(ControlValue::Byte, u8);
impl_control_value_vec!(ControlValue::Int32, i32);
impl_control_value_vec!(ControlValue::Int64, i64);
impl_control_value_vec!(ControlValue::Float, f32);
impl_control_value_vec!(ControlValue::Rectangle, Rectangle);
impl_control_value_vec!(ControlValue::Size, Size);
impl_control_value_vec!(ControlValue::Point, Point);

macro_rules! impl_control_value_array {
    ($p:path, $type:ty) => {
        impl<const N: usize> From<[$type; N]> for ControlValue {
            fn from(val: [$type; N]) -> Self {
                $p(SmallVec::from_slice(&val))
            }
        }

        impl<const N: usize> TryFrom<ControlValue> for [$type; N] {
            type Error = ControlValueError;

            fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
                match value {
                    $p(val) => {
                        Ok(val
                            .into_vec()
                            .try_into()
                            .map_err(|e: Vec<$type>| ControlValueError::InvalidLength {
                                expected: N,
                                found: e.len(),
                            })?)
                    }
                    _ => Err(ControlValueError::InvalidType {
                        // not really efficient, but eh, only on error
                        expected: $p(Default::default()).ty(),
                        found: value.ty(),
                    }),
                }
            }
        }

        impl<const N: usize, const M: usize> From<[[$type; M]; N]> for ControlValue {
            fn from(val: [[$type; M]; N]) -> Self {
                $p(SmallVec::from_slice(&unsafe {
                    core::slice::from_raw_parts(val.as_ptr().cast(), N * M)
                }))
            }
        }

        impl<const N: usize, const M: usize> TryFrom<ControlValue> for [[$type; M]; N] {
            type Error = ControlValueError;

            fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
                match value {
                    $p(val) => {
                        if val.len() == N * M {
                            let mut iter = val.into_iter();
                            Ok([[(); M]; N].map(|a| a.map(|_| iter.next().unwrap())))
                        } else {
                            Err(ControlValueError::InvalidLength {
                                expected: N * M,
                                found: val.len(),
                            })
                        }
                    }
                    _ => Err(ControlValueError::InvalidType {
                        // not really efficient, but eh, only on error
                        expected: $p(Default::default()).ty(),
                        found: value.ty(),
                    }),
                }
            }
        }
    };
}

impl_control_value_array!(ControlValue::Bool, bool);
impl_control_value_array!(ControlValue::Byte, u8);
impl_control_value_array!(ControlValue::Int32, i32);
impl_control_value_array!(ControlValue::Int64, i64);
impl_control_value_array!(ControlValue::Float, f32);
impl_control_value_array!(ControlValue::Rectangle, Rectangle);
impl_control_value_array!(ControlValue::Size, Size);
impl_control_value_array!(ControlValue::Point, Point);

impl From<String> for ControlValue {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}

impl TryFrom<ControlValue> for String {
    type Error = ControlValueError;

    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        match value {
            ControlValue::String(v) => Ok(v),
            _ => Err(ControlValueError::InvalidType {
                expected: libcamera_control_type::LIBCAMERA_CONTROL_TYPE_STRING,
                found: value.ty(),
            }),
        }
    }
}

impl ControlValue {
    pub(crate) unsafe fn read(val: NonNull<libcamera_control_value_t>) -> Result<Self, ControlValueError> {
        let ty = unsafe { libcamera_control_value_type(val.as_ptr()) };
        let num_elements = unsafe { libcamera_control_value_num_elements(val.as_ptr()) };
        let data = unsafe { libcamera_control_value_get(val.as_ptr()) };

        use libcamera_control_type::*;
        match ty {
            LIBCAMERA_CONTROL_TYPE_NONE => Ok(Self::None),
            LIBCAMERA_CONTROL_TYPE_BOOL => {
                let slice = core::slice::from_raw_parts(data as *const bool, num_elements);
                Ok(Self::Bool(SmallVec::from_slice(slice)))
            }
            LIBCAMERA_CONTROL_TYPE_BYTE => {
                let slice = core::slice::from_raw_parts(data as *const u8, num_elements);
                Ok(Self::Byte(SmallVec::from_slice(slice)))
            }
            LIBCAMERA_CONTROL_TYPE_INT32 => {
                let slice = core::slice::from_raw_parts(data as *const i32, num_elements);
                Ok(Self::Int32(SmallVec::from_slice(slice)))
            }
            LIBCAMERA_CONTROL_TYPE_INT64 => {
                let slice = core::slice::from_raw_parts(data as *const i64, num_elements);
                Ok(Self::Int64(SmallVec::from_slice(slice)))
            }
            LIBCAMERA_CONTROL_TYPE_FLOAT => {
                let slice = core::slice::from_raw_parts(data as *const f32, num_elements);
                Ok(Self::Float(SmallVec::from_slice(slice)))
            }
            LIBCAMERA_CONTROL_TYPE_STRING => {
                let slice = core::slice::from_raw_parts(data as *const u8, num_elements);
                Ok(Self::String(core::str::from_utf8(slice).unwrap().to_string()))
            }
            LIBCAMERA_CONTROL_TYPE_RECTANGLE => {
                let slice = core::slice::from_raw_parts(data as *const libcamera_rectangle_t, num_elements);
                Ok(Self::Rectangle(SmallVec::from_iter(
                    slice.iter().map(|r| Rectangle::from(*r)),
                )))
            }
            LIBCAMERA_CONTROL_TYPE_SIZE => {
                let slice = core::slice::from_raw_parts(data as *const libcamera_size_t, num_elements);
                Ok(Self::Size(SmallVec::from_iter(slice.iter().map(|r| Size::from(*r)))))
            }
            LIBCAMERA_CONTROL_TYPE_POINT => {
                let slice = core::slice::from_raw_parts(data as *const libcamera_point_t, num_elements);
                Ok(Self::Point(SmallVec::from_iter(slice.iter().map(|r| Point::from(*r)))))
            }
            _ => Err(ControlValueError::UnknownType(ty)),
        }
    }

    pub(crate) unsafe fn write(&self, val: NonNull<libcamera_control_value_t>) {
        let (data, len) = match self {
            ControlValue::None => (core::ptr::null(), 0),
            ControlValue::Bool(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Byte(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Int32(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Int64(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Float(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::String(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Rectangle(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Size(v) => (v.as_ptr().cast(), v.len()),
            ControlValue::Point(v) => (v.as_ptr().cast(), v.len()),
        };

        let ty = self.ty();
        let is_array = if ty == libcamera_control_type::LIBCAMERA_CONTROL_TYPE_STRING {
            true
        } else {
            len != 1
        };

        libcamera_control_value_set(val.as_ptr(), self.ty(), data, is_array, len as _);
    }

    pub fn ty(&self) -> u32 {
        use libcamera_control_type::*;
        match self {
            ControlValue::None => LIBCAMERA_CONTROL_TYPE_NONE,
            ControlValue::Bool(_) => LIBCAMERA_CONTROL_TYPE_BOOL,
            ControlValue::Byte(_) => LIBCAMERA_CONTROL_TYPE_BYTE,
            ControlValue::Int32(_) => LIBCAMERA_CONTROL_TYPE_INT32,
            ControlValue::Int64(_) => LIBCAMERA_CONTROL_TYPE_INT64,
            ControlValue::Float(_) => LIBCAMERA_CONTROL_TYPE_FLOAT,
            ControlValue::String(_) => LIBCAMERA_CONTROL_TYPE_STRING,
            ControlValue::Rectangle(_) => LIBCAMERA_CONTROL_TYPE_RECTANGLE,
            ControlValue::Size(_) => LIBCAMERA_CONTROL_TYPE_SIZE,
            ControlValue::Point(_) => LIBCAMERA_CONTROL_TYPE_POINT,
        }
    }
}
