use std::{io, marker::PhantomData, ptr::NonNull};

use libcamera_sys::*;

use crate::{camera::Camera, framebuffer::FrameBufferRef, stream::Stream, utils::Immutable};

pub struct FrameBufferAllocator {
    ptr: NonNull<libcamera_framebuffer_allocator_t>,
}

impl FrameBufferAllocator {
    pub fn new(cam: &Camera) -> Self {
        Self {
            ptr: NonNull::new(unsafe { libcamera_framebuffer_allocator_create(cam.ptr.as_ptr()) }).unwrap(),
        }
    }

    pub fn allocate(&mut self, stream: &Stream) -> io::Result<()> {
        let ret = unsafe { libcamera_framebuffer_allocator_allocate(self.ptr.as_ptr(), stream.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn free(&mut self, stream: &Stream) -> io::Result<()> {
        let ret = unsafe { libcamera_framebuffer_allocator_free(self.ptr.as_ptr(), stream.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn buffers(&self, stream: &Stream) -> Immutable<FrameBufferListRef> {
        unsafe {
            Immutable(FrameBufferListRef::from_ptr(
                NonNull::new(
                    libcamera_framebuffer_allocator_buffers(self.ptr.as_ptr(), stream.ptr.as_ptr()).cast_mut(),
                )
                .unwrap(),
            ))
        }
    }
}

impl Drop for FrameBufferAllocator {
    fn drop(&mut self) {
        unsafe { libcamera_framebuffer_allocator_destroy(self.ptr.as_ptr()) }
    }
}

pub struct FrameBufferListRef<'d> {
    pub(crate) ptr: NonNull<libcamera_framebuffer_list_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferListRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_framebuffer_list_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        unsafe { libcamera_framebuffer_list_size(self.ptr.as_ptr()) as _ }
    }

    pub fn get(&self, index: usize) -> Option<Immutable<FrameBufferRef<'d>>> {
        if self.len() <= index {
            None
        } else {
            Some(Immutable(unsafe {
                FrameBufferRef::from_ptr(
                    NonNull::new(libcamera_framebuffer_list_get(self.ptr.as_ptr(), index as _).cast_mut()).unwrap(),
                )
            }))
        }
    }
}
