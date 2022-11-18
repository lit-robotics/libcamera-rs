use std::marker::PhantomData;

use libcamera_sys::*;

use crate::{
    geometry::{Size, SizeRange},
    pixel_format::{PixelFormat, PixelFormats},
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

    pub fn sizes(&self, pixel_format: PixelFormat) -> Vec<Size> {
        let sizes = unsafe { libcamera_stream_formats_sizes(self.ptr, &pixel_format.0) };
        let len = unsafe { libcamera_sizes_size(sizes) } as usize;
        let data = unsafe { libcamera_sizes_data(sizes) };

        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            out.push(Size::from(unsafe { *data.offset(i as _) }));
        }
        out
    }

    pub fn range(&self, pixel_format: PixelFormat) -> SizeRange {
        SizeRange::from(unsafe { libcamera_stream_formats_range(self.ptr, &pixel_format.0) })
    }
}

impl<'d> core::fmt::Debug for StreamFormatsRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for pixel_format in &self.pixel_formats() {
            map.entry(&pixel_format, &self.sizes(pixel_format));
        }
        map.finish()
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

    pub fn get_pixel_format(&self) -> PixelFormat {
        PixelFormat(unsafe { &*self.ptr }.pixel_format)
    }

    pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        unsafe { &mut *self.ptr }.pixel_format = pixel_format.0;
    }

    pub fn get_size(&self) -> Size {
        unsafe { &*self.ptr }.size.into()
    }

    pub fn set_size(&mut self, size: Size) {
        unsafe { &mut *self.ptr }.size = size.into()
    }

    pub fn get_stride(&self) -> u32 {
        unsafe { &*self.ptr }.stride
    }

    pub fn set_stride(&mut self, stride: u32) {
        unsafe { &mut *self.ptr }.stride = stride
    }

    pub fn get_frame_size(&self) -> u32 {
        unsafe { &*self.ptr }.frame_size
    }

    pub fn set_frame_size(&mut self, frame_size: u32) {
        unsafe { &mut *self.ptr }.frame_size = frame_size
    }

    pub fn get_buffer_count(&self) -> u32 {
        unsafe { &*self.ptr }.buffer_count
    }

    pub fn set_buffer_count(&mut self, buffer_count: u32) {
        unsafe { &mut *self.ptr }.buffer_count = buffer_count;
    }

    pub fn stream(&self) -> Option<Stream> {
        let stream = unsafe { libcamera_stream_configuration_stream(self.ptr) };
        // Stream is valid after camera->configure(), but might be invalidated after following reconfigurations.
        // Unfortunatelly, it's hard to handle it with lifetimes so invalid StreamRef's are possible.
        if stream.is_null() {
            None
        } else {
            Some(unsafe { Stream::from_ptr(stream) })
        }
    }

    pub fn formats(&self) -> StreamFormatsRef {
        unsafe { StreamFormatsRef::from_ptr(libcamera_stream_configuration_formats(self.ptr)) }
    }
}

impl<'d> core::fmt::Debug for StreamConfigurationRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamConfigurationRef")
            .field("pixel_format", &self.get_pixel_format())
            .field("size", &self.get_size())
            .field("stride", &self.get_stride())
            .field("frame_size", &self.get_frame_size())
            .field("buffer_count", &self.get_buffer_count())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct Stream {
    /// libcamera_stream_t is used as unique key across various libcamera structures
    /// and adding a lifetime would be really inconvenient. Dangling pointer should not
    /// cause any harm by itself as collection loopup will fail gracefully, however,
    /// it is important to never dereference this pointer to obtain libcamera_stream_configuration_t.
    pub(crate) ptr: *mut libcamera_stream_t,
}

impl Stream {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_stream_t) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}
