#![allow(clippy::manual_strip)]

use std::{any::Any, collections::HashMap, io, marker::PhantomData, ptr::NonNull};

use bitflags::bitflags;
use libcamera_sys::*;

use crate::{control::ControlList, fence::Fence, framebuffer::AsFrameBuffer, stream::Stream};

/// Non-owning view of a libcamera request.
pub struct RequestRef<'d> {
    pub(crate) ptr: NonNull<libcamera_request_t>,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> RequestRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: NonNull<libcamera_request_t>) -> Self {
        Self {
            ptr,
            _phantom: Default::default(),
        }
    }

    pub fn controls(&self) -> &ControlList {
        unsafe { ControlList::from_ptr(NonNull::new(libcamera_request_controls(self.ptr.as_ptr())).unwrap()) }
    }

    pub fn metadata(&self) -> &ControlList {
        unsafe {
            ControlList::from_ptr(NonNull::new(libcamera_request_metadata(self.ptr.as_ptr()).cast_mut()).unwrap())
        }
    }

    pub fn find_buffer(&self, stream: &Stream) -> Option<*mut libcamera_framebuffer_t> {
        let ptr = unsafe { libcamera_request_find_buffer(self.ptr.as_ptr(), stream.ptr.as_ptr()) };
        NonNull::new(ptr).map(|p| p.as_ptr())
    }

    pub fn has_pending_buffers(&self) -> bool {
        unsafe { libcamera_request_has_pending_buffers(self.ptr.as_ptr()) }
    }

    pub fn to_string_repr(&self) -> String {
        unsafe {
            let ptr = libcamera_request_to_string(self.ptr.as_ptr());
            if ptr.is_null() {
                return String::new();
            }
            let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr.cast());
            s
        }
    }

    /// Iterate over buffers attached to this request as (Stream, framebuffer pointer).
    pub fn buffers_iter(&self) -> RequestBufferMapIter<'_> {
        RequestBufferMapIter::new(self.ptr)
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
}

unsafe impl Send for RequestRef<'_> {}

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
            _ => Err(format!("Unknown libcamera_request_status: {value}")),
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
        unsafe {
            ControlList::from_ptr(NonNull::new(libcamera_request_metadata(self.ptr.as_ptr()).cast_mut()).unwrap())
        }
    }

    /// Attaches framebuffer to the request.
    ///
    /// Buffers can only be attached once. To access framebuffer after executing request use [Self::buffer()] or
    /// [Self::buffer_mut()].
    pub fn add_buffer<T: AsFrameBuffer + Any>(&mut self, stream: &Stream, buffer: T) -> io::Result<()> {
        let ret =
            unsafe { libcamera_request_add_buffer(self.ptr.as_ptr(), stream.ptr.as_ptr(), buffer.ptr().as_ptr()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(-ret))
        } else {
            self.buffers.insert(*stream, Box::new(buffer));
            Ok(())
        }
    }

    /// Attaches framebuffer to the request with an optional acquire fence (fd is consumed).
    pub fn add_buffer_with_fence<T: AsFrameBuffer + Any>(
        &mut self,
        stream: &Stream,
        buffer: T,
        fence: Option<Fence>,
    ) -> io::Result<()> {
        let fence_ptr = fence.map(Fence::into_raw).unwrap_or(std::ptr::null_mut());

        let ret = unsafe {
            libcamera_request_add_buffer_with_fence(
                self.ptr.as_ptr(),
                stream.ptr.as_ptr(),
                buffer.ptr().as_ptr(),
                fence_ptr,
            )
        };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(-ret))
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

    pub(crate) fn stream_for_buffer_ptr(&self, fb_ptr: *mut libcamera_framebuffer_t) -> Option<Stream> {
        self.buffers_iter()
            .find_map(|(s, ptr)| if ptr == fb_ptr { Some(s) } else { None })
    }

    /// Returns the buffer attached to a stream (raw pointer).
    pub fn find_buffer(&self, stream: &Stream) -> Option<*mut libcamera_framebuffer_t> {
        let ptr = unsafe { libcamera_request_find_buffer(self.ptr.as_ptr(), stream.ptr.as_ptr()) };
        NonNull::new(ptr).map(|p| p.as_ptr())
    }

    pub fn has_pending_buffers(&self) -> bool {
        unsafe { libcamera_request_has_pending_buffers(self.ptr.as_ptr()) }
    }

    pub fn to_string_repr(&self) -> String {
        unsafe {
            let ptr = libcamera_request_to_string(self.ptr.as_ptr());
            if ptr.is_null() {
                return String::new();
            }
            let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr.cast());
            s
        }
    }

    /// Iterate over buffers attached to this request as (Stream, framebuffer pointer).
    pub fn buffers_iter(&self) -> RequestBufferMapIter<'_> {
        RequestBufferMapIter::new(self.ptr)
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
        // Mirror libcamera behaviour: unless REUSE_BUFFERS is set, drop our buffer map so callbacks
        // and buffer() lookups can't return stale handles.
        if !flags.contains(ReuseFlag::REUSE_BUFFERS) {
            self.buffers.clear();
        }
    }
}

impl core::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { libcamera_request_destroy(self.ptr.as_ptr()) }
    }
}

unsafe impl Send for Request {}

pub struct RequestBufferMapIter<'d> {
    iter: NonNull<libcamera_request_buffer_map_iter_t>,
    _phantom: core::marker::PhantomData<&'d libcamera_request_buffer_map_t>,
}

impl<'d> RequestBufferMapIter<'d> {
    pub fn new(req_ptr: NonNull<libcamera_request_t>) -> Self {
        let map = unsafe { libcamera_request_buffers(req_ptr.as_ptr()) };
        let iter = NonNull::new(unsafe { libcamera_request_buffer_map_iter(map.cast_mut()) }).unwrap();
        Self {
            iter,
            _phantom: Default::default(),
        }
    }
}

impl<'d> Iterator for RequestBufferMapIter<'d> {
    type Item = (Stream, *mut libcamera_framebuffer_t);

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { libcamera_request_buffer_map_iter_end(self.iter.as_ptr()) } {
            return None;
        }
        let stream = unsafe {
            Stream::from_ptr(
                NonNull::new(libcamera_request_buffer_map_iter_stream(self.iter.as_ptr()) as *mut _).unwrap(),
            )
        };
        let buffer = unsafe { libcamera_request_buffer_map_iter_buffer(self.iter.as_ptr()) };
        unsafe { libcamera_request_buffer_map_iter_next(self.iter.as_ptr()) };
        Some((stream, buffer))
    }
}

impl Drop for RequestBufferMapIter<'_> {
    fn drop(&mut self) {
        unsafe { libcamera_request_buffer_map_iter_destroy(self.iter.as_ptr()) }
    }
}
