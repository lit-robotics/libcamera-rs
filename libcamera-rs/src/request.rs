#![allow(clippy::manual_strip)]

use std::{any::Any, collections::HashMap, io, ptr::NonNull};

use bitflags::bitflags;
use libcamera_sys::*;

use crate::{control::ControlList, framebuffer::AsFrameBuffer, stream::Stream};

/// Status of [Request]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    /// Request is ready to be executed by [ActiveCamera::queue_request()](crate::camera::ActiveCamera::queue_request)
    Pending,
    /// Request was executed successfully
    Complete,
    /// Request was cancelled, most likely due to call to [ActiveCamera::stop()](crate::camera::ActiveCamera::stop)
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

bitflags! {
    /// Flags to control the behaviour of [Request::reuse()].
    pub struct ReuseFlag: u32 {
        /// Reuse the buffers that were previously added by [Request::add_buffer()].
        const REUSE_BUFFERS = 1 << 0;
    }
}

/// A camera capture request.
///
/// Capture requests are created by [ActiveCamera::create_request()](crate::camera::ActiveCamera::create_request)
/// and scheduled for execution by [ActiveCamera::queue_request()](crate::camera::ActiveCamera::queue_request).
/// Completed requests are returned by request completed callback (see
/// [ActiveCamera::on_request_completed()](crate::camera::ActiveCamera::on_request_completed)) and can (should) be
/// reused by calling [ActiveCamera::queue_request()](crate::camera::ActiveCamera::queue_request) again.
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

    /// Returns an immutable reference of request controls.
    ///
    /// See [controls](crate::controls) for available items.
    pub fn controls(&self) -> &ControlList {
        unsafe { ControlList::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap()) }
    }

    /// Returns a mutable reference of request controls.
    ///
    /// See [controls](crate::controls) for available items.
    pub fn controls_mut(&mut self) -> &mut ControlList {
        unsafe { ControlList::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap()) }
    }

    /// Returns request metadata, which contains information relevant to the request execution (i.e. capture timestamp).
    ///
    /// See [controls](crate::controls) for available items.
    pub fn metadata(&self) -> &ControlList {
        unsafe { ControlList::from_ptr(NonNull::new(libcamera_request_metadata(self.ptr.as_ptr())).unwrap()) }
    }

    /// Attaches framebuffer to the request.
    ///
    /// Buffers can only be attached once. To access framebuffer after executing request use [Self::buffer()] or
    /// [Self::buffer_mut()].
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

    /// Returns a reference to the buffer that was attached with [Self::add_buffer()].
    ///
    /// `T` must be equal to the type used in [Self::add_buffer()], otherwise this will return None.
    pub fn buffer<T: 'static>(&self, stream: &Stream) -> Option<&T> {
        self.buffers.get(stream).and_then(|b| b.downcast_ref())
    }

    /// Returns a mutable reference to the buffer that was attached with [Self::add_buffer()].
    ///
    /// `T` must be equal to the type used in [Self::add_buffer()], otherwise this will return None.
    pub fn buffer_mut<T: 'static>(&mut self, stream: &Stream) -> Option<&mut T> {
        self.buffers.get_mut(stream).and_then(|b| b.downcast_mut())
    }

    /// Returns auto-incrementing sequence number of the capture
    pub fn sequence(&self) -> u32 {
        unsafe { libcamera_request_sequence(self.ptr.as_ptr()) }
    }

    /// Returns request identifier that was provided in
    /// [ActiveCamera::create_request()](crate::camera::ActiveCamera::create_request).
    ///
    /// Returns zero if cookie was not provided.
    pub fn cookie(&self) -> u64 {
        unsafe { libcamera_request_cookie(self.ptr.as_ptr()) }
    }

    /// Capture request status
    pub fn status(&self) -> RequestStatus {
        RequestStatus::try_from(unsafe { libcamera_request_status(self.ptr.as_ptr()) }).unwrap()
    }

    /// Reset the request for reuse.
    ///
    /// Reset the status and controls associated with the request, to allow it to be reused and requeued without
    /// destruction. This function shall be called prior to queueing the request to the camera, in lieu of
    /// constructing a new request. The application can reuse the buffers that were previously added to the request
    /// via [Self::add_buffer()] by setting flags to [ReuseFlag::REUSE_BUFFERS].
    pub fn reuse(&mut self, flags: ReuseFlag) {
        unsafe { libcamera_request_reuse(self.ptr.as_ptr(), flags.bits()) }
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
