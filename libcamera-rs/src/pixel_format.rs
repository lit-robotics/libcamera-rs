use std::{ffi::CStr, marker::PhantomData};

use libcamera_sys::*;

pub struct PixelFormatRef<'d> {
    pub(crate) ptr: *const libcamera_pixel_format_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> PixelFormatRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_pixel_format_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn fourcc(&self) -> u32 {
        unsafe { libcamera_pixel_format_fourcc(self.ptr) }
    }

    pub fn modifier(&self) -> u64 {
        unsafe { libcamera_pixel_format_modifier(self.ptr) }
    }

    pub fn to_string(&self) -> String {
        let mut buf = [0u8; 64];
        unsafe { libcamera_pixel_format_str(self.ptr, buf.as_mut_ptr() as _, buf.len() as u64 - 1) };
        unsafe { CStr::from_bytes_with_nul_unchecked(&buf) }
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl<'d> core::fmt::Debug for PixelFormatRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

pub struct PixelFormats {
    ptr: *mut libcamera_pixel_formats_t,
}

impl PixelFormats {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_pixel_formats_t) -> Self {
        Self { ptr }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_pixel_formats_size(self.ptr) as _ }
    }

    pub fn get(&self, index: usize) -> Option<PixelFormatRef> {
        let ptr = unsafe { libcamera_pixel_formats_get(self.ptr, index as _) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { PixelFormatRef::from_ptr(ptr) })
        }
    }
}

impl<'d> IntoIterator for &'d PixelFormats {
    type Item = PixelFormatRef<'d>;

    type IntoIter = PixelFormatsIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        PixelFormatsIterator {
            formats: self,
            index: 0,
        }
    }
}

impl Drop for PixelFormats {
    fn drop(&mut self) {
        unsafe { libcamera_pixel_formats_destroy(self.ptr) }
    }
}

pub struct PixelFormatsIterator<'d> {
    formats: &'d PixelFormats,
    index: usize,
}

impl<'d> Iterator for PixelFormatsIterator<'d> {
    type Item = PixelFormatRef<'d>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.formats.get(self.index) {
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }
}
