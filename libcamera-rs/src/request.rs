use std::{any::Any, collections::HashMap, ptr::NonNull};

use libc::EEXIST;
use libcamera_sys::*;

use crate::{control::ControlListRef, framebuffer::AsFrameBuffer, stream::Stream, utils::Immutable};

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("The chosen stream already has a buffer associated with it")]
    BufferAlreadyAdded,
}

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

/// Flags to control the behaviour of [Request::reuse()]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReuseFlag {
    /// Do not reuse buffers.
    Default,
    /// Reuse the buffers that were previously added by [Request::add_buffer()].
    ReuseBuffers,
}

impl From<ReuseFlag> for libcamera_request_reuse_flag_t {
    fn from(value: ReuseFlag) -> Self {
        match value {
            ReuseFlag::Default => libcamera_request_reuse_flag::LIBCAMERA_REQUEST_REUSE_FLAG_DEFAULT,
            ReuseFlag::ReuseBuffers => libcamera_request_reuse_flag::LIBCAMERA_REQUEST_REUSE_FLAG_REUSE_BUFFERS,
        }
    }
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

/// A camera capture request.
///
/// Capture requests are created by [ActiveCamera::create_request()](crate::camera::ActiveCamera::create_request)
/// and scheduled for execution by [ActiveCamera::queue_request()](crate::camera::ActiveCamera::queue_request).
/// Completed requests are returned by request completed callback (see [ActiveCamera::on_request_completed()](crate::camera::ActiveCamera::on_request_completed))
/// and can (should) be reused by calling [ActiveCamera::queue_request()](crate::camera::ActiveCamera::queue_request) again.
pub struct Request<S: RequestState> {
    pub(crate) ptr: NonNull<libcamera_request_t>,
    state: S,
}

impl Request<WithoutBuffers> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_request_t>) -> Self {
        Self {
            ptr,
            state: WithoutBuffers {},
        }
    }
}

impl Request<WithBuffers> {
    /// Returns a reference to the buffer that was attached with [Self::add_buffer()].
    ///
    /// `T` must be equal to the type used in [Self::add_buffer()], otherwise this will return None.
    pub fn buffer<T: 'static>(&self, stream: &Stream) -> Option<&T> {
        self.state.buffers.get(stream).map(|b| b.downcast_ref()).flatten()
    }

    /// Returns a mutable reference to the buffer that was attached with [Self::add_buffer()].
    ///
    /// `T` must be equal to the type used in [Self::add_buffer()], otherwise this will return None.
    pub fn buffer_mut<T: 'static>(&mut self, stream: &Stream) -> Option<&mut T> {
        self.state.buffers.get_mut(stream).map(|b| b.downcast_mut()).flatten()
    }

    /// Reset the request for reuse.
    ///
    /// Reset the status and controls associated with the request, to allow it to be reused and requeued without destruction. This
    /// function shall be called prior to queueing the request to the camera, in lieu of constructing a new request. The same buffers
    /// that were originally added with [Self::add_buffer()] will be recycled. To fully reset even the buffers, see [Self::reset()].
    pub fn reuse(&mut self) {
        let flags = ReuseFlag::ReuseBuffers;
        unsafe { libcamera_request_reuse(self.ptr.as_ptr(), flags.into()) }
    }

    pub fn reset(self) -> Request<WithoutBuffers> {
        let flags = ReuseFlag::Default;
        unsafe { libcamera_request_reuse(self.ptr.as_ptr(), flags.into()) };
        Request {
            ptr: self.ptr,
            state: WithoutBuffers {},
        }
    }
}

impl<S: RequestState> Request<S> {
    /// Attaches a framebuffer to the request.
    ///
    /// Buffers can only be attached once per stream, otherwise an error will be rasied. Use [Self::buffer()] or [Self::buffer_mut()]
    /// to access the buffer once it has been added.
    pub fn add_buffer<T: AsFrameBuffer + Any>(
        self,
        stream: &Stream,
        buffer: T,
    ) -> Result<Request<WithBuffers>, RequestError> {
        match unsafe { libcamera_request_add_buffer(self.ptr.as_ptr(), stream.ptr.as_ptr(), buffer.ptr().as_ptr()) } {
            0 => Ok(()),
            // `libcamera_request_add_buffer` has three failure modes:
            //    -EINVAL - `stream` is null, but we know that our stream is non-null
            //    -EEXIST - either the stream already has an associated buffer, or the buffer
            //              has an associated `fence`, which we don't currently support.
            v if v == -EEXIST => Err(RequestError::BufferAlreadyAdded),
            _ => unreachable!(),
        }?;

        let mut buffers: HashMap<Stream, Box<dyn Any + 'static>> = HashMap::new();
        buffers.insert(*stream, Box::new(buffer));
        let state = WithBuffers { buffers };
        Ok(Request { ptr: self.ptr, state })
    }

    /// Returns an immutable reference of request controls.
    ///
    /// See [controls](crate::controls) for available items.
    pub fn controls(&self) -> Immutable<ControlListRef> {
        Immutable(unsafe {
            ControlListRef::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap())
        })
    }

    /// Returns a mutable reference of request controls.
    ///
    /// See [controls](crate::controls) for available items.
    pub fn controls_mut(&mut self) -> ControlListRef {
        unsafe { ControlListRef::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap()) }
    }

    /// Returns request metadata, which contains information relevant to the request execution (i.e. capture timestamp).
    pub fn metadata(&self) -> Immutable<ControlListRef> {
        Immutable(unsafe {
            ControlListRef::from_ptr(NonNull::new(libcamera_request_metadata(self.ptr.as_ptr())).unwrap())
        })
    }

    // TODO: libcamera sets the sequence number after queuing it to the camera. Does it get zeroed after Request::reuse? If so,
    // it may only make sense if sequence is implemented just for State=WithBuffer
    /// Returns auto-incrementing sequence number of the capture
    pub fn sequence(&self) -> u32 {
        unsafe { libcamera_request_sequence(self.ptr.as_ptr()) }
    }

    /// Returns request identifier that was provided in [ActiveCamera::create_request()](crate::camera::ActiveCamera::create_request).
    ///
    /// Returns zero if cookie was not provided.
    pub fn cookie(&self) -> u64 {
        unsafe { libcamera_request_cookie(self.ptr.as_ptr()) }
    }

    /// Capture request status
    pub fn status(&self) -> RequestStatus {
        RequestStatus::try_from(unsafe { libcamera_request_status(self.ptr.as_ptr()) }).unwrap()
    }
}

impl<S: RequestState> core::fmt::Debug for Request<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("seq", &self.sequence())
            .field("status", &self.status())
            .field("cookie", &self.cookie())
            .finish()
    }
}

impl<S: RequestState> Drop for Request<S> {
    fn drop(&mut self) {
        unsafe { libcamera_request_destroy(self.ptr.as_ptr()) }
    }
}

/// A marker trait for the typestate of a [Request].
pub trait RequestState {}

/// The typestate of a fresh [Request] which has not yet had any buffers added to it. Before a request can
/// be queued for execution by the camera, [Request::add_buffer()] must have been called at least once to move
/// it into the `Request<WithBuffers>` state.
pub struct WithoutBuffers {}

/// The typestate of a [Request] which has a buffer associated to at least one of its configured streams. In this
/// state, it can be queued to the camera for execution.
pub struct WithBuffers {
    buffers: HashMap<Stream, Box<dyn Any + 'static>>,
}

impl RequestState for WithBuffers {}
impl RequestState for WithoutBuffers {}

unsafe impl<S: RequestState> Send for Request<S> {}
