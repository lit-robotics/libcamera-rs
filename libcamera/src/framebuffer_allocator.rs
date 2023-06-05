use std::{io, ptr::NonNull, sync::Arc};

use libcamera_sys::*;

use crate::{camera::Camera, framebuffer::AsFrameBuffer, stream::Stream};

/// Buffers are stored inside `libcamera_framebuffer_allocator_t` so we use Arc<FrameBufferAllocatorInstance>
/// to keep the allocator alive as long as there are active buffers.
struct FrameBufferAllocatorInstance {
    ptr: NonNull<libcamera_framebuffer_allocator_t>,
}

impl Drop for FrameBufferAllocatorInstance {
    fn drop(&mut self) {
        unsafe { libcamera_framebuffer_allocator_destroy(self.ptr.as_ptr()) }
    }
}

pub struct FrameBufferAllocator {
    inner: Arc<FrameBufferAllocatorInstance>,
}

impl FrameBufferAllocator {
    pub fn new(cam: &Camera<'_>) -> Self {
        Self {
            inner: Arc::new(FrameBufferAllocatorInstance {
                ptr: NonNull::new(unsafe { libcamera_framebuffer_allocator_create(cam.ptr.as_ptr()) }).unwrap(),
            }),
        }
    }

    /// Allocate N buffers for a given stream, where N is equal to
    /// [StreamConfigurationRef::get_buffer_count()](crate::stream::StreamConfigurationRef::get_buffer_count).
    pub fn alloc(&mut self, stream: &Stream) -> io::Result<Vec<FrameBuffer>> {
        let ret = unsafe { libcamera_framebuffer_allocator_allocate(self.inner.ptr.as_ptr(), stream.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            let buffers =
                unsafe { libcamera_framebuffer_allocator_buffers(self.inner.ptr.as_ptr(), stream.ptr.as_ptr()) };
            let len = unsafe { libcamera_framebuffer_list_size(buffers) };
            Ok((0..len)
                .map(|i| unsafe { libcamera_framebuffer_list_get(buffers, i) })
                .map(|ptr| NonNull::new(ptr.cast_mut()).unwrap())
                .map(|ptr| {
                    // This is very very unsafe.
                    // Setting first field of metadata (status) to u32::MAX, which is used as an indication that
                    // metadata is unavailable. Otherwise all metadata fields are uninitialized and
                    // there is no way to detect availability.
                    unsafe {
                        libcamera_framebuffer_metadata(ptr.as_ptr())
                            .cast_mut()
                            .cast::<u32>()
                            .write(u32::MAX)
                    };

                    FrameBuffer {
                        ptr,
                        alloc: self.inner.clone(),
                        stream_ptr: stream.ptr,
                    }
                })
                .collect())
        }
    }
}

pub struct FrameBuffer {
    ptr: NonNull<libcamera_framebuffer_t>,
    alloc: Arc<FrameBufferAllocatorInstance>,
    /// Only used as an ID for deallocating framebuffer, may be invalid otherwise
    stream_ptr: NonNull<libcamera_stream_t>,
}

impl core::fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameBuffer")
            .field("metadata", &self.metadata())
            .field("planes", &self.planes())
            .finish()
    }
}

unsafe impl Send for FrameBuffer {}

impl AsFrameBuffer for FrameBuffer {
    unsafe fn ptr(&self) -> NonNull<libcamera_framebuffer_t> {
        self.ptr
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            libcamera_framebuffer_allocator_free(self.alloc.ptr.as_ptr(), self.stream_ptr.as_ptr());
        }
    }
}
