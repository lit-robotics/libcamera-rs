use std::io;

use libcamera_sys::*;

use crate::{control::ControlListRef, framebuffer::FrameBufferRef, stream::StreamRef, utils::Immutable};

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
    pub(crate) ptr: *mut libcamera_request_t,
}

impl Request {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_request_t) -> Self {
        Self { ptr }
    }

    pub fn controls(&self) -> Immutable<ControlListRef> {
        unsafe { ControlListRef::from_ptr(libcamera_request_controls(self.ptr)) }
    }

    pub fn controls_mut(&mut self) -> ControlListRef {
        unsafe { ControlListRef::from_ptr_mut(libcamera_request_controls(self.ptr)) }
    }

    pub fn metadata(&self) -> Immutable<ControlListRef> {
        unsafe { ControlListRef::from_ptr(libcamera_request_metadata(self.ptr)) }
    }

    pub fn add_buffer(&mut self, stream: &StreamRef, buffer: &FrameBufferRef) -> io::Result<()> {
        let ret = unsafe { libcamera_request_add_buffer(self.ptr, stream.ptr, buffer.ptr) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn status(&self) -> RequestStatus {
        RequestStatus::try_from(unsafe { libcamera_request_status(self.ptr) }).unwrap()
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { libcamera_request_destroy(self.ptr) }
    }
}
