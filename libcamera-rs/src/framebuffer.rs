use std::marker::PhantomData;

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
    pub(crate) ptr: *mut libcamera_frame_metadata_planes_t,
}

impl FrameMetadataPlanes {
    pub(crate) unsafe fn from_ptr_mut(ptr: *mut libcamera_frame_metadata_planes_t) -> Self {
        Self { ptr }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_frame_metadata_planes_size(self.ptr) as _ }
    }

    pub fn get(&self, index: usize) -> Option<FrameMetadataPlane> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { libcamera_frame_metadata_planes_data(self.ptr).offset(index as _).read() })
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
        unsafe { libcamera_frame_metadata_planes_destroy(self.ptr) }
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
    pub(crate) ptr: *mut libcamera_frame_metadata_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameMetadataRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_frame_metadata_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    pub fn status(&self) -> FrameMetadataStatus {
        FrameMetadataStatus::try_from(unsafe { libcamera_frame_metadata_status(self.ptr) }).unwrap()
    }

    pub fn sequence(&self) -> u32 {
        unsafe { libcamera_frame_metadata_sequence(self.ptr) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { libcamera_frame_metadata_timestamp(self.ptr) }
    }

    pub fn planes(&self) -> FrameMetadataPlanes {
        unsafe { FrameMetadataPlanes::from_ptr_mut(libcamera_frame_metadata_planes(self.ptr)) }
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
    pub(crate) ptr: *mut libcamera_framebuffer_plane_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferPlaneRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_framebuffer_plane_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    /// File descriptor is valid for the [FrameBufferRef] lifetime.
    pub fn fd(&self) -> i32 {
        unsafe { libcamera_framebuffer_plane_fd(self.ptr) }
    }

    pub fn offset(&self) -> Option<usize> {
        if unsafe { libcamera_framebuffer_plane_offset_valid(self.ptr) } {
            Some(unsafe { libcamera_framebuffer_plane_offset(self.ptr) as _ })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_plane_length(self.ptr) as _ }
    }
}

pub struct FrameBufferPlanesRef<'d> {
    pub(crate) ptr: *mut libcamera_framebuffer_planes_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferPlanesRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_framebuffer_planes_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_planes_size(self.ptr) as _ }
    }

    pub fn get(&self, index: usize) -> Option<Immutable<FrameBufferPlaneRef>> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe {
                FrameBufferPlaneRef::from_ptr(libcamera_framebuffer_planes_data(self.ptr).offset(index as _))
            })
        }
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

pub struct FrameBufferRef<'d> {
    pub(crate) ptr: *mut libcamera_framebuffer_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_framebuffer_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    pub(crate) unsafe fn from_ptr_mut(ptr: *mut libcamera_framebuffer_t) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn metadata(&self) -> Immutable<FrameMetadataRef> {
        unsafe { FrameMetadataRef::from_ptr(libcamera_framebuffer_metadata(self.ptr)) }
    }

    pub fn planes(&self) -> Immutable<FrameBufferPlanesRef> {
        unsafe { FrameBufferPlanesRef::from_ptr(libcamera_framebuffer_planes(self.ptr)) }
    }
}
