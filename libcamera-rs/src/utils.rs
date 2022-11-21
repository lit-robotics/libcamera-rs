use std::ops::Deref;

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
