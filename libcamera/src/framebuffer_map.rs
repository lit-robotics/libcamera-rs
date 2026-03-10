use std::{collections::HashMap, mem::MaybeUninit};

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
    #[error("Plane {index} has an invalid offset")]
    InvalidOffset { index: usize },
    #[error("mmap failed with {0:?}")]
    MemoryMapError(std::io::Error),
    #[error("mapping was created read-only; write access requested")]
    NotWritable,
}

#[derive(Clone)]
struct MappedPlane {
    fd: i32,
    offset: usize,
    len: usize,
}

/// FrameBuffer wrapper, which exposes internal file descriptors as memory mapped [&[u8]] plane slices.
#[derive(Clone)]
pub struct MemoryMappedFrameBuffer<T: AsFrameBuffer> {
    fb: T,
    writable: bool,
    /// fd -> (mapped_ptr, mapped_len, map_offset)
    mmaps: HashMap<i32, (*mut core::ffi::c_void, usize, usize)>,
    planes: Vec<MappedPlane>,
}

impl<T: AsFrameBuffer> MemoryMappedFrameBuffer<T> {
    /// Memory map framebuffer, which implements [AsFrameBuffer].
    ///
    /// This might fail if framebuffer has invalid plane sizes/offsets or if [libc::mmap] fails itself.
    pub fn new(fb: T) -> Result<Self, MemoryMappedFrameBufferError> {
        Self::with_access(fb, false)
    }

    /// Memory map framebuffer for read/write access. Mapping will be `PROT_READ | PROT_WRITE`.
    pub fn new_writable(fb: T) -> Result<Self, MemoryMappedFrameBufferError> {
        Self::with_access(fb, true)
    }

    fn with_access(fb: T, writable: bool) -> Result<Self, MemoryMappedFrameBufferError> {
        struct MapInfo {
            /// Page-aligned start offset for mapping
            start: usize,
            /// Maximum offset used by data planes
            end: usize,
            /// Total file descriptor size
            total_len: usize,
        }

        let mut planes = Vec::new();
        let mut map_info: HashMap<i32, MapInfo> = HashMap::new();
        let page_size = {
            let ps = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
            if ps > 0 {
                ps as usize
            } else {
                4096
            }
        };

        for (index, plane) in fb.planes().into_iter().enumerate() {
            let fd = plane.fd();
            let offset = plane
                .offset()
                .ok_or(MemoryMappedFrameBufferError::InvalidOffset { index })?;
            let len = plane.len();

            planes.push(MappedPlane { fd, offset, len });

            // Find total FD length if not known yet
            map_info.entry(fd).or_insert_with(|| {
                let mut st = MaybeUninit::<libc::stat>::uninit();
                let ret = unsafe { libc::fstat(fd, st.as_mut_ptr()) };
                let total_len = if ret != 0 {
                    0
                } else {
                    let st = unsafe { st.assume_init() };
                    st.st_size as usize
                };
                MapInfo {
                    start: offset,
                    end: offset,
                    total_len,
                }
            });

            let info = map_info.get_mut(&fd).unwrap();

            // If total_len is 0 (unknown for many DMA-BUFs), skip the bound check and let mmap fail if invalid.
            if info.total_len > 0 && offset + len > info.total_len {
                return Err(MemoryMappedFrameBufferError::PlaneOutOfBounds {
                    index,
                    offset,
                    len,
                    fd_len: info.total_len,
                });
            }

            let aligned_start = offset - (offset % page_size);
            info.start = info.start.min(aligned_start);
            info.end = info.end.max(offset + len);
        }

        let mmaps = map_info
            .iter()
            .map(|(fd, info)| {
                let map_len = info.end.saturating_sub(info.start);
                let addr = unsafe {
                    libc::mmap64(
                        core::ptr::null_mut(),
                        map_len,
                        libc::PROT_READ | if writable { libc::PROT_WRITE } else { 0 },
                        libc::MAP_SHARED,
                        *fd,
                        info.start as _,
                    )
                };

                if addr == libc::MAP_FAILED {
                    Err(MemoryMappedFrameBufferError::MemoryMapError(
                        std::io::Error::last_os_error(),
                    ))
                } else {
                    Ok((*fd, (addr, map_len, info.start)))
                }
            })
            .collect::<Result<HashMap<i32, (*mut core::ffi::c_void, usize, usize)>, MemoryMappedFrameBufferError>>()
            .unwrap();

        Ok(Self {
            fb,
            writable,
            mmaps,
            planes,
        })
    }

    /// Returns data slice for each plane within the framebuffer.
    pub fn data(&self) -> Vec<&[u8]> {
        self.planes
            .iter()
            .map(|plane| {
                let (mmap_ptr, _, map_offset) = self.mmaps[&plane.fd];
                let mmap_ptr: *const u8 = mmap_ptr.cast();
                let offset = plane.offset - map_offset;
                unsafe { core::slice::from_raw_parts(mmap_ptr.add(offset), plane.len) }
            })
            .collect()
    }

    /// Returns mutable data slices for each plane within the framebuffer. Mapping must be writable.
    pub fn data_mut(&mut self) -> Result<Vec<&mut [u8]>, MemoryMappedFrameBufferError> {
        if !self.writable {
            return Err(MemoryMappedFrameBufferError::NotWritable);
        }

        Ok(self
            .planes
            .iter()
            .map(|plane| {
                let (mmap_ptr, _, map_offset) = self.mmaps[&plane.fd];
                let mmap_ptr: *mut u8 = mmap_ptr.cast();
                let offset = plane.offset - map_offset;
                unsafe { core::slice::from_raw_parts_mut(mmap_ptr.add(offset), plane.len) }
            })
            .collect())
    }

    /// Returns true if this mapping was created with write access.
    pub fn is_writable(&self) -> bool {
        self.writable
    }

    /// Returns the mapped length for a given file descriptor, if present.
    pub fn mapped_len(&self, fd: i32) -> Option<usize> {
        self.mmaps.get(&fd).map(|(_, len, _)| *len)
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
        for (_fd, (ptr, size, _map_offset)) in self.mmaps.drain() {
            unsafe {
                libc::munmap(ptr, size);
            }
        }
    }
}
