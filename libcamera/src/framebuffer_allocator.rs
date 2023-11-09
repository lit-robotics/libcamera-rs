use std::{
    io,
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use libcamera_sys::*;

use crate::{camera::Camera, framebuffer::AsFrameBuffer, stream::Stream};

/// Buffers are stored inside `libcamera_framebuffer_allocator_t` so we use Arc<FrameBufferAllocatorInstance>
/// to keep the allocator alive as long as there are active buffers.
struct FrameBufferAllocatorInstance {
    ptr: NonNull<libcamera_framebuffer_allocator_t>,
    /// List of streams for which buffers were allocated.
    /// We use this list to free buffers on drop.
    allocated_streams: Vec<NonNull<libcamera_stream_t>>,
}

unsafe impl Send for FrameBufferAllocatorInstance {}

impl Drop for FrameBufferAllocatorInstance {
    fn drop(&mut self) {
        // Free allocated streams
        for stream in self.allocated_streams.drain(..) {
            unsafe {
                libcamera_framebuffer_allocator_free(self.ptr.as_ptr(), stream.as_ptr());
            }
        }

        unsafe { libcamera_framebuffer_allocator_destroy(self.ptr.as_ptr()) }
    }
}

pub struct FrameBufferAllocator {
    inner: Arc<Mutex<FrameBufferAllocatorInstance>>,
}

impl FrameBufferAllocator {
    pub fn new(cam: &Camera<'_>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(FrameBufferAllocatorInstance {
                ptr: NonNull::new(unsafe { libcamera_framebuffer_allocator_create(cam.ptr.as_ptr()) }).unwrap(),
                allocated_streams: Vec::new(),
            })),
        }
    }

    /// Allocate N buffers for a given stream, where N is equal to
    /// [StreamConfigurationRef::get_buffer_count()](crate::stream::StreamConfigurationRef::get_buffer_count).
    pub fn alloc(&mut self, stream: &Stream) -> io::Result<Vec<FrameBuffer>> {
        let mut inner = self.inner.lock().unwrap();

        let ret = unsafe { libcamera_framebuffer_allocator_allocate(inner.ptr.as_ptr(), stream.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            inner.allocated_streams.push(stream.ptr);

            let buffers = unsafe { libcamera_framebuffer_allocator_buffers(inner.ptr.as_ptr(), stream.ptr.as_ptr()) };

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
                        _alloc: self.inner.clone(),
                    }
                })
                .collect())
        }
    }
}

pub struct FrameBuffer {
    ptr: NonNull<libcamera_framebuffer_t>,
    _alloc: Arc<Mutex<FrameBufferAllocatorInstance>>,
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
