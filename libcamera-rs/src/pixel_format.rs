use std::ffi::CStr;

use libcamera_sys::*;

#[derive(Clone, Copy)]
pub struct PixelFormat(pub(crate) libcamera_pixel_format_t);

impl PixelFormat {
    pub fn fourcc(&self) -> u32 {
        self.0.fourcc
    }

    pub fn modifier(&self) -> u64 {
        self.0.modifier
    }

    pub fn to_string(&self) -> String {
        let mut buf = [0u8; 64];
        unsafe { libcamera_pixel_format_str(&self.0, buf.as_mut_ptr() as _, buf.len() as u64 - 1) };
        unsafe { CStr::from_bytes_with_nul_unchecked(&buf) }
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl core::fmt::Debug for PixelFormat {
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

    pub fn get(&self, index: usize) -> Option<PixelFormat> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> PixelFormat {
        PixelFormat(unsafe { libcamera_pixel_formats_get(self.ptr, index as _) })
    }
}

impl<'d> IntoIterator for &'d PixelFormats {
    type Item = PixelFormat;

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
    type Item = PixelFormat;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.formats.get(self.index) {
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }
}
