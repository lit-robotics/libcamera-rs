use std::marker::PhantomData;

use libcamera_sys::libcamera_framebuffer_t;

use crate::utils::Immutable;

pub struct FrameBufferRef<'d> {
    pub(crate) ptr: *mut libcamera_framebuffer_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> FrameBufferRef<'d> {
    pub(crate) unsafe fn from_ptr(ptr: *const libcamera_framebuffer_t) -> Immutable<Self> {
        Immutable(Self {
            ptr: ptr as _,
            _phantom: Default::default(),
        })
    }
}
