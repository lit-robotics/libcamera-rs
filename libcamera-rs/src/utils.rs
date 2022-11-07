use std::ops::Deref;

/// A wrapper to make a type immutable.
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
