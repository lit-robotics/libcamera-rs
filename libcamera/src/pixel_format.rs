use std::{ffi::CStr, ptr::NonNull};

use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
use libcamera_sys::*;

/// Represents `libcamera::PixelFormat`, which itself is a pair of fourcc code and u64 modifier as defined in `libdrm`.
#[derive(Clone, Copy)]
pub struct PixelFormat(pub(crate) libcamera_pixel_format_t);

impl PixelFormat {
    /// Constructs new [PixelFormat] from given fourcc code and modifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libcamera::pixel_format::PixelFormat;
    /// // Constructs MJPEG pixel format
    /// const PIXEL_FORMAT_MJPEG: PixelFormat =
    ///     PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);
    /// ```
    pub const fn new(fourcc: u32, modifier: u64) -> Self {
        Self(libcamera_pixel_format_t { fourcc, modifier })
    }

    pub fn fourcc(&self) -> u32 {
        self.0.fourcc
    }

    pub fn set_fourcc(&mut self, fourcc: u32) {
        self.0.fourcc = fourcc;
    }

    pub fn modifier(&self) -> u64 {
        self.0.modifier
    }

    pub fn set_modifier(&mut self, modifier: u64) {
        self.0.modifier = modifier;
    }
}

impl PartialEq for PixelFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0.fourcc.eq(&other.0.fourcc) && self.0.modifier.eq(&other.0.modifier)
    }
}

impl Eq for PixelFormat {}

impl core::fmt::Debug for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = unsafe { libcamera_pixel_format_str(&self.0) };
        let out = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        f.write_str(out)?;
        unsafe { libc::free(ptr.cast()) };
        Ok(())
    }
}

impl TryFrom<PixelFormat> for DrmFormat {
    type Error = drm_fourcc::UnrecognizedFourcc;

    fn try_from(value: PixelFormat) -> Result<Self, Self::Error> {
        let code = DrmFourcc::try_from(value.0.fourcc)?;
        let modifier = DrmModifier::from(value.0.modifier);
        Ok(DrmFormat { code, modifier })
    }
}

impl From<DrmFormat> for PixelFormat {
    fn from(f: DrmFormat) -> Self {
        PixelFormat::new(f.code as u32, f.modifier.into())
    }
}

/// Vector of [PixelFormat]
pub struct PixelFormats {
    ptr: NonNull<libcamera_pixel_formats_t>,
}

impl PixelFormats {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_pixel_formats_t>) -> Self {
        Self { ptr }
    }

    /// Number of [PixelFormat]
    pub fn len(&self) -> usize {
        unsafe { libcamera_pixel_formats_size(self.ptr.as_ptr()) as _ }
    }

    /// Returns `true` if there there are no pixel formats
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns [PixelFormat] at a given index.
    ///
    /// Return None if index is out of range.
    pub fn get(&self, index: usize) -> Option<PixelFormat> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    /// Returns [PixelFormat] at a given index without checking bounds.
    ///
    /// # Safety
    ///
    /// `index` must be less than [PixelFormats::len()].
    pub unsafe fn get_unchecked(&self, index: usize) -> PixelFormat {
        PixelFormat(unsafe { libcamera_pixel_formats_get(self.ptr.as_ptr(), index as _) })
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
        unsafe { libcamera_pixel_formats_destroy(self.ptr.as_ptr()) }
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
