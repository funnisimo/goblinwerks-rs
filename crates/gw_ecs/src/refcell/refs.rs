use super::{AtomicBorrowRef, AtomicBorrowRefMut};
use std::ops::{Deref, DerefMut};

/// Shared reference to a component.
pub struct AtomicRef2<'a, T> {
    inner: &'a T,
    all_borrow: AtomicBorrowRef<'a>,
    borrow: AtomicBorrowRef<'a>,
}

impl<'a, T> AtomicRef2<'a, T> {
    pub(crate) fn new(
        inner: &'a T,
        all_borrow: AtomicBorrowRef<'a>,
        borrow: AtomicBorrowRef<'a>,
    ) -> Self {
        AtomicRef2 {
            inner,
            all_borrow,
            borrow,
        }
    }

    /// Makes a new [`Ref`].
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
    #[inline]
    pub fn map<U, F: FnOnce(&'a T) -> &'a U>(orig: Self, f: F) -> AtomicRef2<'a, U> {
        AtomicRef2 {
            inner: f(orig.inner),
            all_borrow: orig.all_borrow,
            borrow: orig.borrow,
        }
    }
}

impl<'a, T> Deref for AtomicRef2<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> AsRef<T> for AtomicRef2<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner
    }
}

/// Shared reference to a component.
pub struct AtomicRefMut2<'a, T> {
    inner: &'a mut T,
    all_borrow: AtomicBorrowRef<'a>,
    borrow: AtomicBorrowRefMut<'a>,
}

impl<'a, T> AtomicRefMut2<'a, T> {
    pub(crate) fn new(
        inner: &'a mut T,
        all_borrow: AtomicBorrowRef<'a>,
        borrow: AtomicBorrowRefMut<'a>,
    ) -> Self {
        AtomicRefMut2 {
            inner,
            all_borrow,
            borrow,
        }
    }

    /// Makes a new [`Ref`].
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
    #[inline]
    pub fn map<U, F: FnOnce(&'a mut T) -> &'a mut U>(orig: Self, f: F) -> AtomicRefMut2<'a, U> {
        AtomicRefMut2 {
            inner: f(orig.inner),
            all_borrow: orig.all_borrow,
            borrow: orig.borrow,
        }
    }
}

impl<'a, T> Deref for AtomicRefMut2<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for AtomicRefMut2<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> AsRef<T> for AtomicRefMut2<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner
    }
}

impl<'a, T> AsMut<T> for AtomicRefMut2<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner
    }
}
