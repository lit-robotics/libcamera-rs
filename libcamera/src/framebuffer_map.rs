use std::collections::HashMap;

use thiserror::Error;

use crate::framebuffer::AsFrameBuffer;

#[derive(Debug, Error)]
pub enum MemoryMappedFrameBufferError {
    #[error("Plane {index} with offset {offset} and size {len} exceeds file descriptor size of {fd_len}")]
    PlaneOutOfBounds {
        index: usize,
        offset: usize,
        len: usize,
        fd_len: usize,
    },
    #[error("mmap failed with {0:?}")]
    MemoryMapError(std::io::Error),
}

struct MappedPlane {
    fd: i32,
    offset: usize,
    len: usize,
}

/// FrameBuffer wrapper, which exposes internal file descriptors as memory mapped [&[u8]] plane slices.
pub struct MemoryMappedFrameBuffer<T: AsFrameBuffer> {
    fb: T,
    mmaps: HashMap<i32, (*const core::ffi::c_void, usize)>,
    planes: Vec<MappedPlane>,
}

impl<T: AsFrameBuffer> MemoryMappedFrameBuffer<T> {
    /// Memory map framebuffer, which implements [AsFrameBuffer].
    ///
    /// This might fail if framebuffer has invalid plane sizes/offsets or if [libc::mmap] fails itself.
    pub fn new(fb: T) -> Result<Self, MemoryMappedFrameBufferError> {
        struct MapInfo {
            /// Maximum offset used by data planes
            mapped_len: usize,
            /// Total file descriptor size
            total_len: usize,
        }

        let mut planes = Vec::new();
        let mut map_info: HashMap<i32, MapInfo> = HashMap::new();

        for (index, plane) in fb.planes().into_iter().enumerate() {
            let fd = plane.fd();
            let offset = plane.offset().unwrap();
            let len = plane.len();

            planes.push(MappedPlane { fd, offset, len });

            // Find total FD length if not known yet
            map_info.entry(fd).or_insert_with(|| {
                let total_len = unsafe { libc::lseek64(fd, 0, libc::SEEK_END) } as usize;
                MapInfo {
                    mapped_len: 0,
                    total_len,
                }
            });

            let info = map_info.get_mut(&fd).unwrap();

            if offset + len > info.total_len {
                return Err(MemoryMappedFrameBufferError::PlaneOutOfBounds {
                    index,
                    offset,
                    len,
                    fd_len: info.total_len,
                });
            }

            info.mapped_len = info.mapped_len.max(offset + len);
        }

        let mmaps = map_info
            .iter()
            .map(|(fd, info)| {
                let addr = unsafe {
                    libc::mmap64(
                        core::ptr::null_mut(),
                        info.mapped_len,
                        libc::PROT_READ,
                        libc::MAP_SHARED,
                        *fd,
                        0,
                    )
                };

                if addr == libc::MAP_FAILED {
                    Err(MemoryMappedFrameBufferError::MemoryMapError(
                        std::io::Error::last_os_error(),
                    ))
                } else {
                    Ok((*fd, (addr.cast_const(), info.mapped_len)))
                }
            })
            .collect::<Result<HashMap<i32, (*const core::ffi::c_void, usize)>, MemoryMappedFrameBufferError>>()
            .unwrap();

        Ok(Self { fb, mmaps, planes })
    }

    /// Returns data slice for each plane within the framebuffer.
    pub fn data(&self) -> Vec<&[u8]> {
        self.planes
            .iter()
            .map(|plane| {
                let mmap_ptr: *const u8 = self.mmaps[&plane.fd].0.cast();
                unsafe { core::slice::from_raw_parts(mmap_ptr.add(plane.offset), plane.len) }
            })
            .collect()
    }
}

impl<T: AsFrameBuffer> AsFrameBuffer for MemoryMappedFrameBuffer<T> {
    unsafe fn ptr(&self) -> std::ptr::NonNull<libcamera_sys::libcamera_framebuffer_t> {
        self.fb.ptr()
    }
}

unsafe impl<T: AsFrameBuffer> Send for MemoryMappedFrameBuffer<T> {}

impl<T: AsFrameBuffer> Drop for MemoryMappedFrameBuffer<T> {
    fn drop(&mut self) {
        // Unmap
        for (_fd, (ptr, size)) in self.mmaps.drain() {
            unsafe {
                libc::munmap(ptr.cast_mut(), size);
            }
        }
    }
}
