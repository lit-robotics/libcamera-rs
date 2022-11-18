use std::collections::HashMap;

use thiserror::Error;

use crate::framebuffer::FrameBufferRef;

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

pub struct MemoryMappedFrameBuffer<'d> {
    fb: &'d FrameBufferRef<'d>,
    mmaps: HashMap<i32, &'d [u8]>,
}

impl<'d> MemoryMappedFrameBuffer<'d> {
    pub fn from_framebuffer(fb: &'d FrameBufferRef<'d>) -> Result<Self, MemoryMappedFrameBufferError> {
        struct MapInfo {
            /// Maximum offset used by data planes
            mapped_len: usize,
            /// Total file descriptor size
            total_len: usize,
        }

        let mut map_info: HashMap<i32, MapInfo> = HashMap::new();

        for (index, plane) in fb.planes().into_iter().enumerate() {
            let fd = plane.fd();
            let offset = plane.offset().unwrap();
            let len = plane.len();

            // Find total FD length if not known yet
            if !map_info.contains_key(&fd) {
                let total_len = unsafe { libc::lseek64(fd, 0, libc::SEEK_END) } as usize;
                map_info.insert(
                    fd,
                    MapInfo {
                        mapped_len: 0,
                        total_len,
                    },
                );
            }

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
                    let data = unsafe { core::slice::from_raw_parts(addr as *const u8, info.mapped_len) };
                    Ok((*fd, data))
                }
            })
            .collect::<Result<HashMap<i32, &[u8]>, MemoryMappedFrameBufferError>>()
            .unwrap();

        Ok(Self { fb, mmaps })
    }

    pub fn planes(&self) -> Vec<&[u8]> {
        self.fb
            .planes()
            .into_iter()
            .map(|plane| {
                let offset = plane.offset().unwrap();
                let len = plane.len();
                &self.mmaps[&plane.fd()][offset..offset + len]
            })
            .collect()
    }
}

impl<'d> Drop for MemoryMappedFrameBuffer<'d> {
    fn drop(&mut self) {
        for (_fd, data) in self.mmaps.drain() {
            unsafe {
                libc::munmap(data.as_ptr().cast_mut().cast(), data.len());
            }
        }
    }
}
