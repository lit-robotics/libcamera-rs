use std::{marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::utils::Immutable;

#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum FrameMetadataStatus {
    Success = libcamera_frame_metadata_status::LIBCAMERA_FRAME_METADATA_STATUS_SUCCESS,
    Error = libcamera_frame_metadata_status::LIBCAMERA_FRAME_METADATA_STATUS_ERROR,
    Cancelled = libcamera_frame_metadata_status::LIBCAMERA_FRAME_METADATA_STATUS_CANCELLED,
}

pub type FrameMetadataPlane = libcamera_frame_metadata_plane_t;

pub struct FrameMetadataPlanes {
    pub(crate) ptr: NonNull<libcamera_frame_metadata_planes_t>,
}

impl FrameMetadataPlanes {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_frame_metadata_planes_t>) -> Self {
        Self { ptr }
    }

    /// Number of planes within framebuffer metadata.
    ///
    /// Should be consistent with other planes within framebuffer.
    pub fn len(&self) -> usize {
        unsafe { libcamera_frame_metadata_planes_size(self.ptr.as_ptr()) as _ }
    }

    /// Returns `true` if there are no planes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns framebuffer plane metadata at a given index.
    ///
    /// Return None if given index is out of range of available planes.
    pub fn get(&self, index: usize) -> Option<FrameMetadataPlane> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { libcamera_frame_metadata_planes_at(self.ptr.as_ptr(), index as _).read() })
        }
    }
}

impl core::fmt::Debug for FrameMetadataPlanes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for plane in self.into_iter() {
            list.entry(&plane);
        }
        list.finish()
    }
}

impl Drop for FrameMetadataPlanes {
    fn drop(&mut self) {
        unsafe { libcamera_frame_metadata_planes_destroy(self.ptr.as_ptr()) }
    }
}

impl<'d> IntoIterator for &'d FrameMetadataPlanes {
    type Item = FrameMetadataPlane;

    type IntoIter = FrameMetadataPlanesIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        FrameMetadataPlanesIterator { planes: self, index: 0 }
    }
}

pub struct FrameMetadataPlanesIterator<'d> {
    planes: &'d FrameMetadataPlanes,
    index: usize,
}

impl<'d> Iterator for FrameMetadataPlanesIterator<'d> {
    type Item = FrameMetadataPlane;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(plane) = self.planes.get(self.index) {
            self.index += 1;
            Some(plane)
        } else {
            None
        }
    }
}

pub struct FrameMetadataRef<'d> {
    pub(crate) ptr: NonNull<libcamera_frame_metadata_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameMetadataRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_frame_metadata_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn status(&self) -> FrameMetadataStatus {
        FrameMetadataStatus::try_from(unsafe { libcamera_frame_metadata_status(self.ptr.as_ptr()) }).unwrap()
    }

    pub fn sequence(&self) -> u32 {
        unsafe { libcamera_frame_metadata_sequence(self.ptr.as_ptr()) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { libcamera_frame_metadata_timestamp(self.ptr.as_ptr()) }
    }

    pub fn planes(&self) -> FrameMetadataPlanes {
        unsafe {
            FrameMetadataPlanes::from_ptr(NonNull::new(libcamera_frame_metadata_planes(self.ptr.as_ptr())).unwrap())
        }
    }
}

impl<'d> core::fmt::Debug for FrameMetadataRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameMetadataRef")
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("timestamp", &self.timestamp())
            .field("planes", &self.planes())
            .finish()
    }
}

pub struct FrameBufferPlaneRef<'d> {
    pub(crate) ptr: NonNull<libcamera_framebuffer_plane_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferPlaneRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_framebuffer_plane_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    /// File descriptor to the framebuffer plane data.
    ///
    /// Multiple planes may point to the same file descriptor at different offsets.
    pub fn fd(&self) -> i32 {
        unsafe { libcamera_framebuffer_plane_fd(self.ptr.as_ptr()) }
    }

    /// Offset of data within the file descriptor.
    pub fn offset(&self) -> Option<usize> {
        if unsafe { libcamera_framebuffer_plane_offset_valid(self.ptr.as_ptr()) } {
            Some(unsafe { libcamera_framebuffer_plane_offset(self.ptr.as_ptr()) as _ })
        } else {
            None
        }
    }

    /// Data length of the plane in bytes
    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_plane_length(self.ptr.as_ptr()) as _ }
    }

    /// Returns `true` if plane has no data
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'d> core::fmt::Debug for FrameBufferPlaneRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameBufferPlaneRef")
            .field("fd", &self.fd())
            .field("offset", &self.offset())
            .field("len", &self.len())
            .finish()
    }
}

pub struct FrameBufferPlanesRef<'d> {
    pub(crate) ptr: NonNull<libcamera_framebuffer_planes_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferPlanesRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_framebuffer_planes_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    /// Number of planes within framebuffer
    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_planes_size(self.ptr.as_ptr()) as _ }
    }

    /// Returns `true` if framebuffer has no planes
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns framebuffer plane at a given index
    pub fn get(&self, index: usize) -> Option<Immutable<FrameBufferPlaneRef<'_>>> {
        if index >= self.len() {
            None
        } else {
            Some(Immutable(unsafe {
                FrameBufferPlaneRef::from_ptr(
                    NonNull::new(libcamera_framebuffer_planes_at(self.ptr.as_ptr(), index as _)).unwrap(),
                )
            }))
        }
    }
}

impl<'d> core::fmt::Debug for FrameBufferPlanesRef<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for plane in self.into_iter() {
            list.entry(&plane);
        }
        list.finish()
    }
}

impl<'d> IntoIterator for &'d FrameBufferPlanesRef<'d> {
    type Item = Immutable<FrameBufferPlaneRef<'d>>;

    type IntoIter = FrameBufferPlanesRefIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufferPlanesRefIterator { planes: self, index: 0 }
    }
}

pub struct FrameBufferPlanesRefIterator<'d> {
    planes: &'d FrameBufferPlanesRef<'d>,
    index: usize,
}

impl<'d> Iterator for FrameBufferPlanesRefIterator<'d> {
    type Item = Immutable<FrameBufferPlaneRef<'d>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(plane) = self.planes.get(self.index) {
            self.index += 1;
            Some(plane)
        } else {
            None
        }
    }
}

pub trait AsFrameBuffer: Send {
    /// Returns raw framebuffer used by libcamera.
    ///
    /// It is expected that metadata status field is initialized with u32::MAX on a new buffer, which indicates that
    /// metadata is not yet available. This "hackfix" prevents read of uninitialized data in [Self::metadata()].
    ///
    /// # Safety
    ///
    /// This function must return a valid instance of `libcamera::FrameBuffer`.
    unsafe fn ptr(&self) -> NonNull<libcamera_framebuffer_t>;

    /// Returns framebuffer metadata information.
    ///
    /// Only available after associated [Request](crate::request::Request) has completed.
    fn metadata(&self) -> Option<Immutable<FrameMetadataRef<'_>>> {
        let ptr = NonNull::new(unsafe { libcamera_framebuffer_metadata(self.ptr().as_ptr()) }.cast_mut()).unwrap();
        if unsafe { libcamera_frame_metadata_status(ptr.as_ptr()) } != u32::MAX {
            Some(unsafe { Immutable(FrameMetadataRef::from_ptr(ptr)) })
        } else {
            None
        }
    }

    /// Provides access to framebuffer data by exposing file descriptors, offsets and lengths of the planes.
    fn planes(&self) -> Immutable<FrameBufferPlanesRef<'_>> {
        unsafe {
            Immutable(FrameBufferPlanesRef::from_ptr(
                NonNull::new(libcamera_framebuffer_planes(self.ptr().as_ptr()).cast_mut()).unwrap(),
            ))
        }
    }
}
