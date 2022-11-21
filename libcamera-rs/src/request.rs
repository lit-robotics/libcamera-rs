use std::{any::Any, collections::HashMap, io, ptr::NonNull};

use libcamera_sys::*;

use crate::{control::ControlListRef, framebuffer::AsFrameBuffer, stream::Stream, utils::Immutable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Complete,
    Cancelled,
}

impl TryFrom<libcamera_request_status_t> for RequestStatus {
    type Error = String;

    fn try_from(value: libcamera_request_status_t) -> Result<Self, Self::Error> {
        match value {
            libcamera_request_status::LIBCAMERA_REQUEST_STATUS_PENDING => Ok(Self::Pending),
            libcamera_request_status::LIBCAMERA_REQUEST_STATUS_COMPLETE => Ok(Self::Complete),
            libcamera_request_status::LIBCAMERA_REQUEST_STATUS_CANCELLED => Ok(Self::Cancelled),
            _ => Err(format!("Unknown libcamera_request_status: {}", value)),
        }
    }
}

pub struct Request {
    pub(crate) ptr: NonNull<libcamera_request_t>,
    buffers: HashMap<Stream, Box<dyn Any + 'static>>,
}

impl Request {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_request_t>) -> Self {
        Self {
            ptr,
            buffers: Default::default(),
        }
    }

    pub fn controls(&self) -> Immutable<ControlListRef> {
        Immutable(unsafe {
            ControlListRef::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap())
        })
    }

    pub fn controls_mut(&mut self) -> ControlListRef {
        unsafe { ControlListRef::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap()) }
    }

    pub fn metadata(&self) -> Immutable<ControlListRef> {
        Immutable(unsafe {
            ControlListRef::from_ptr(NonNull::new(libcamera_request_metadata(self.ptr.as_ptr())).unwrap())
        })
    }

    pub fn add_buffer<T: AsFrameBuffer + Any>(&mut self, stream: &Stream, buffer: T) -> io::Result<()> {
        let ret =
            unsafe { libcamera_request_add_buffer(self.ptr.as_ptr(), stream.ptr.as_ptr(), buffer.ptr().as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            self.buffers.insert(*stream, Box::new(buffer));
            Ok(())
        }
    }

    pub fn buffer<T: 'static>(&self, stream: &Stream) -> Option<&T> {
        self.buffers.get(stream).map(|b| b.downcast_ref()).flatten()
    }

    pub fn buffer_mut<T: 'static>(&mut self, stream: &Stream) -> Option<&mut T> {
        self.buffers.get_mut(stream).map(|b| b.downcast_mut()).flatten()
    }

    pub fn sequence(&self) -> u32 {
        unsafe { libcamera_request_sequence(self.ptr.as_ptr()) }
    }

    pub fn cookie(&self) -> u64 {
        unsafe { libcamera_request_cookie(self.ptr.as_ptr()) }
    }

    pub fn status(&self) -> RequestStatus {
        RequestStatus::try_from(unsafe { libcamera_request_status(self.ptr.as_ptr()) }).unwrap()
    }
}

impl core::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("seq", &self.sequence())
            .field("status", &self.status())
            .field("cookie", &self.cookie())
            .finish()
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { libcamera_request_destroy(self.ptr.as_ptr()) }
    }
}

unsafe impl Send for Request {}
