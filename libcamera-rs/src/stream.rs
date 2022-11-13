use std::marker::PhantomData;

use libcamera_sys::*;

use crate::{
    geometry::{Size, SizeRange},
    pixel_format::{PixelFormatRef, PixelFormats},
};

#[derive(Debug, Clone, Copy)]
pub enum StreamRole {
    Raw,
    StillCapture,
    VideoRecording,
    ViewFinder,
}

impl TryFrom<libcamera_stream_role::Type> for StreamRole {
    type Error = ();

    fn try_from(value: libcamera_stream_role::Type) -> Result<Self, Self::Error> {
        match value {
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_RAW => Ok(StreamRole::Raw),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_STILL_CAPTURE => Ok(StreamRole::StillCapture),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING => Ok(StreamRole::VideoRecording),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIEW_FINDER => Ok(StreamRole::ViewFinder),
            _ => Err(()),
        }
    }
}

impl From<StreamRole> for libcamera_stream_role::Type {
    fn from(role: StreamRole) -> Self {
        match role {
            StreamRole::Raw => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_RAW,
            StreamRole::StillCapture => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_STILL_CAPTURE,
            StreamRole::VideoRecording => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING,
            StreamRole::ViewFinder => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIEW_FINDER,
        }
    }
}

pub struct StreamFormatsRef<'d> {
    ptr: *const libcamera_stream_formats_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> StreamFormatsRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_stream_formats_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn pixel_formats(&self) -> PixelFormats {
        unsafe { PixelFormats::from_ptr(libcamera_stream_formats_pixel_formats(self.ptr)) }
    }

    pub fn sizes(&self, pixel_format: &PixelFormatRef) -> Vec<Size> {
        let sizes = unsafe { libcamera_stream_formats_sizes(self.ptr, pixel_format.ptr) };
        let len = unsafe { libcamera_sizes_size(sizes) } as usize;
        let data = unsafe { libcamera_sizes_data(sizes) };

        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            out.push(Size::from(unsafe { *data.offset(i as _) }));
        }
        out
    }

    pub fn range(&self, pixel_format: &PixelFormatRef) -> SizeRange {
        SizeRange::from(unsafe { libcamera_stream_formats_range(self.ptr, pixel_format.ptr) })
    }
}

pub struct StreamConfigurationRef<'d> {
    ptr: *mut libcamera_stream_configuration_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> StreamConfigurationRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_stream_configuration_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn formats(&self) -> StreamFormatsRef {
        unsafe { StreamFormatsRef::from_ptr(libcamera_stream_configuration_formats(self.ptr)) }
    }
}
