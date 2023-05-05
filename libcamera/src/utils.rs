use std::{
    ffi::c_int,
    io,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// Provides only an immutable reference to the contained type T.
///
/// Used for FFI types to avoid having separate variants depending on mutability.
pub struct Immutable<T: ?Sized>(pub(crate) T);

impl<T> Immutable<T> {
    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Immutable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Immutable").field(&self.0).finish()
    }
}

/// Trait, which allows type to be used in [UniquePtr]
pub trait UniquePtrTarget: Sized {
    /// Allocates `Self` in the heap and returns pointer.
    ///
    /// # Safety
    ///
    /// Pointer must be deallocated by calling [Self::ptr_drop()] when no longer needed.
    unsafe fn ptr_new() -> *mut Self;
    /// Destroys pointer allocated in `ptr_new()`.
    ///
    /// # Safety
    ///
    /// Pointer must have been created by [Self::ptr_new()] and is no longer aliased.
    unsafe fn ptr_drop(ptr: *mut Self);
}

/// Similar to [Box], but uses custom alloc and drop methods defined in [UniquePtrTarget].
pub struct UniquePtr<T: UniquePtrTarget> {
    ptr: NonNull<T>,
}

impl<T: UniquePtrTarget> UniquePtr<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::new(unsafe { T::ptr_new() }).unwrap(),
        }
    }
}

impl<T: UniquePtrTarget> Default for UniquePtr<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: UniquePtrTarget> Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr.as_ptr() as *const T) }
    }
}

impl<T: UniquePtrTarget> DerefMut for UniquePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr.as_mut() as *mut T) }
    }
}

impl<T: UniquePtrTarget> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        unsafe { T::ptr_drop(self.ptr.as_mut()) }
    }
}

impl<T: UniquePtrTarget + core::fmt::Debug> core::fmt::Debug for UniquePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

#[inline]
pub fn handle_result(ret: c_int) -> io::Result<()> {
    if ret < 0 {
        Err(io::Error::from_raw_os_error(ret))
    } else {
        Ok(())
    }
}
