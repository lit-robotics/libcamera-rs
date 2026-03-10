use std::{
    collections::HashMap,
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
    /// Active allocations per stream pointer.
    streams: HashMap<usize, StreamAllocState>,
}

#[derive(Debug)]
struct StreamAllocState {
    count: usize,
    free_requested: bool,
}

unsafe impl Send for FrameBufferAllocatorInstance {}

impl Drop for FrameBufferAllocatorInstance {
    fn drop(&mut self) {
        // Free any remaining streams.
        for (stream_ptr, _) in self.streams.drain() {
            unsafe { libcamera_framebuffer_allocator_free(self.ptr.as_ptr(), stream_ptr as *mut _) };
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
                streams: HashMap::new(),
            })),
        }
    }

    /// Allocate N buffers for a given stream, where N is equal to
    /// [StreamConfigurationRef::get_buffer_count()](crate::stream::StreamConfigurationRef::get_buffer_count).
    pub fn alloc(&mut self, stream: &Stream) -> io::Result<Vec<FrameBuffer>> {
        let mut inner = self.inner.lock().unwrap();
        let key = stream.ptr.as_ptr() as usize;
        if inner.streams.contains_key(&key) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "buffers already allocated for this stream",
            ));
        }

        let ret = unsafe { libcamera_framebuffer_allocator_allocate(inner.ptr.as_ptr(), stream.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(-ret))
        } else {
            let buffers = unsafe { libcamera_framebuffer_allocator_buffers(inner.ptr.as_ptr(), stream.ptr.as_ptr()) };

            let len = unsafe { libcamera_framebuffer_list_size(buffers) };

            inner.streams.insert(
                key,
                StreamAllocState {
                    count: len,
                    free_requested: false,
                },
            );

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
                        stream_key: key,
                        _alloc: self.inner.clone(),
                    }
                })
                .collect())
        }
    }

    /// Free buffers for a stream.
    pub fn free(&mut self, stream: &Stream) -> io::Result<()> {
        let mut inner = self.inner.lock().unwrap();
        let key = stream.ptr.as_ptr() as usize;
        let state = inner
            .streams
            .get_mut(&key)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no buffers allocated for stream"))?;

        state.free_requested = true;
        if state.count == 0 {
            let ret = unsafe { libcamera_framebuffer_allocator_free(inner.ptr.as_ptr(), stream.ptr.as_ptr()) };
            if ret < 0 {
                return Err(io::Error::from_raw_os_error(-ret));
            }
            inner.streams.remove(&key);
        }
        Ok(())
    }

    /// Returns true if any buffers are allocated.
    pub fn allocated(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        unsafe { libcamera_framebuffer_allocator_allocated(inner.ptr.as_ptr()) }
    }
}

#[derive(Clone)]
pub struct FrameBuffer {
    ptr: NonNull<libcamera_framebuffer_t>,
    stream_key: usize,
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

impl FrameBuffer {
    /// Retrieve the user cookie associated with this buffer.
    pub fn cookie(&self) -> u64 {
        unsafe { libcamera_framebuffer_cookie(self.ptr.as_ptr()) }
    }

    /// Set a user cookie for this buffer.
    pub fn set_cookie(&self, cookie: u64) {
        unsafe { libcamera_framebuffer_set_cookie(self.ptr.as_ptr(), cookie) }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        let mut inner = self._alloc.lock().unwrap();
        if let Some(state) = inner.streams.get_mut(&self.stream_key) {
            if state.count > 0 {
                state.count -= 1;
            }
            if state.count == 0 && state.free_requested {
                unsafe {
                    let _ = libcamera_framebuffer_allocator_free(inner.ptr.as_ptr(), self.stream_key as *mut _);
                }
                inner.streams.remove(&self.stream_key);
            }
        }
    }
}
