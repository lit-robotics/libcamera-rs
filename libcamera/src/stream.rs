use std::{marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;

use crate::{
    geometry::{Size, SizeRange},
    pixel_format::{PixelFormat, PixelFormats},
    utils::Immutable,
};

/// Stream role hint for generating configuration.
///
/// Used in [Camera::generate_configuration()](crate::camera::Camera::generate_configuration).
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

/// A list of available stream formats.
pub struct StreamFormatsRef<'d> {
    ptr: NonNull<libcamera_stream_formats_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> StreamFormatsRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_stream_formats_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    /// Returns all available [PixelFormat]s.
    pub fn pixel_formats(&self) -> Immutable<PixelFormats> {
        Immutable(unsafe {
            PixelFormats::from_ptr(NonNull::new(libcamera_stream_formats_pixel_formats(self.ptr.as_ptr())).unwrap())
        })
    }

    /// Returns all supported stream [Size]s for a given [PixelFormat].
    pub fn sizes(&self, pixel_format: PixelFormat) -> Vec<Size> {
        let sizes = unsafe { libcamera_stream_formats_sizes(self.ptr.as_ptr(), &pixel_format.0) };
        let len = unsafe { libcamera_sizes_size(sizes) };

        (0..len)
            .map(|i| Size::from(unsafe { *libcamera_sizes_at(sizes, i as _) }))
            .collect()
    }

    /// Returns a [SizeRange] of supported stream sizes for a given [PixelFormat].
    pub fn range(&self, pixel_format: PixelFormat) -> SizeRange {
        SizeRange::from(unsafe { libcamera_stream_formats_range(self.ptr.as_ptr(), &pixel_format.0) })
    }
}

impl<'d> core::fmt::Debug for StreamFormatsRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for pixel_format in self.pixel_formats().into_iter() {
            map.entry(&pixel_format, &self.sizes(pixel_format));
        }
        map.finish()
    }
}

pub struct StreamConfigurationRef<'d> {
    ptr: NonNull<libcamera_stream_configuration_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> StreamConfigurationRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_stream_configuration_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        PixelFormat(unsafe { self.ptr.as_ref() }.pixel_format)
    }

    pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        unsafe { self.ptr.as_mut() }.pixel_format = pixel_format.0;
    }

    pub fn get_size(&self) -> Size {
        unsafe { self.ptr.as_ref() }.size.into()
    }

    pub fn set_size(&mut self, size: Size) {
        unsafe { self.ptr.as_mut() }.size = size.into()
    }

    pub fn get_stride(&self) -> u32 {
        unsafe { self.ptr.as_ref() }.stride
    }

    pub fn set_stride(&mut self, stride: u32) {
        unsafe { self.ptr.as_mut() }.stride = stride
    }

    pub fn get_frame_size(&self) -> u32 {
        unsafe { self.ptr.as_ref() }.frame_size
    }

    pub fn set_frame_size(&mut self, frame_size: u32) {
        unsafe { self.ptr.as_mut() }.frame_size = frame_size
    }

    pub fn get_buffer_count(&self) -> u32 {
        unsafe { self.ptr.as_ref() }.buffer_count
    }

    pub fn set_buffer_count(&mut self, buffer_count: u32) {
        unsafe { self.ptr.as_mut() }.buffer_count = buffer_count;
    }

    /// Returns initialized [Stream] for this configuration.
    ///
    /// Stream is only available once this configuration is applied with
    /// [ActiveCamera::configure()](crate::camera::ActiveCamera::configure). It is invalidated if camera is
    /// reconfigured.
    pub fn stream(&self) -> Option<Stream> {
        let stream = unsafe { libcamera_stream_configuration_stream(self.ptr.as_ptr()) };
        // Stream is valid after camera->configure(), but might be invalidated after following reconfigurations.
        // Unfortunatelly, it's hard to handle it with lifetimes so invalid StreamRef's are possible.
        NonNull::new(stream).map(|p| unsafe { Stream::from_ptr(p) })
    }

    /// Returns a list of available stream formats for this configuration.
    pub fn formats(&self) -> StreamFormatsRef<'_> {
        unsafe {
            StreamFormatsRef::from_ptr(
                NonNull::new(libcamera_stream_configuration_formats(self.ptr.as_ptr()).cast_mut()).unwrap(),
            )
        }
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

/// Handle to a camera stream.
///
/// Obtained from [StreamConfigurationRef::stream()] and is valid as long as camera configuration is unchanged.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Stream {
    /// libcamera_stream_t is used as unique key across various libcamera structures
    /// and adding a lifetime would be really inconvenient. Dangling pointer should not
    /// cause any harm by itself as collection loopup will fail gracefully, however,
    /// it is important to never dereference this pointer to obtain libcamera_stream_configuration_t.
    pub(crate) ptr: NonNull<libcamera_stream_t>,
}

impl Stream {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_stream_t>) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Stream {}
