use std::{io, marker::PhantomData};

use libcamera_sys::*;

use crate::{camera::Camera, framebuffer::FrameBufferRef, stream::StreamRef, utils::Immutable};

pub struct FrameBufferAllocator {
    ptr: *mut libcamera_framebuffer_allocator_t,
}

impl FrameBufferAllocator {
    pub fn new(cam: &Camera) -> Self {
        Self {
            ptr: unsafe { libcamera_framebuffer_allocator_create(libcamera_camera_copy(cam.ptr)) },
        }
    }

    pub fn allocate(&mut self, stream: &StreamRef) -> io::Result<()> {
        let ret = unsafe { libcamera_framebuffer_allocator_allocate(self.ptr, stream.ptr) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn free(&mut self, stream: &StreamRef) -> io::Result<()> {
        let ret = unsafe { libcamera_framebuffer_allocator_free(self.ptr, stream.ptr) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn buffers(&self, stream: &StreamRef) -> Immutable<FrameBufferListRef> {
        unsafe { FrameBufferListRef::from_ptr(libcamera_framebuffer_allocator_buffers(self.ptr, stream.ptr)) }
    }
}

impl Drop for FrameBufferAllocator {
    fn drop(&mut self) {
        unsafe { libcamera_framebuffer_allocator_destroy(self.ptr) }
    }
}

pub struct FrameBufferListRef<'d> {
    pub(crate) ptr: *mut libcamera_framebuffer_list_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_framebuffer_list_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_list_size(self.ptr) as _ }
    }

    pub fn get(&self, index: usize) -> Option<Immutable<FrameBufferRef<'d>>> {
        if self.len() <= index {
            None
        } else {
            Some(unsafe { FrameBufferRef::from_ptr(libcamera_framebuffer_list_get(self.ptr, index as _)) })
        }
    }
}
