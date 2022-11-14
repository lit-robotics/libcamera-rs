use libcamera_sys::*;

pub struct Request {
    pub(crate) ptr: *mut libcamera_request_t,
}

impl Request {
    pub(crate) unsafe fn from_ptr(ptr: *mut libcamera_request_t) -> Self {
        Self { ptr }
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { libcamera_request_destroy(self.ptr) }
    }
}
