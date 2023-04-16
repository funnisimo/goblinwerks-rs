// use super::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
// use std::ops::{Deref, DerefMut};

// /// Shared reference to a component.
// pub struct AtomicRef2<'a, T: ?Sized> {
//     parent: AtomicBorrowRef<'a>,
//     borrow: AtomicRef<'a, T>,
// }

// impl<'a, T> AtomicRef2<'a, T>
// where
//     T: ?Sized,
// {
//     pub(crate) fn new(parent: AtomicBorrowRef<'a>, borrow: AtomicRef<'a, T>) -> Self {
//         AtomicRef2 { parent, borrow }
//     }

//     // /// Destructures Ref so you can use it in a layered ref
//     // pub(crate) fn destructure(self) -> (&'a T, AtomicBorrowRef<'a>, AtomicBorrowRef<'a>) {
//     //     let (t, borrow) = self.borrow.destructure();
//     //     (t, borrow, self.parent)
//     // }

//     /// Makes a new [`Ref`].
//     ///
//     /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
//     #[inline]
//     pub fn map<U, F>(orig: AtomicRef2<'a, T>, f: F) -> AtomicRef2<'a, U>
//     where
//         U: ?Sized,
//         F: FnOnce(&T) -> &U,
//     {
//         AtomicRef2 {
//             parent: orig.parent,
//             borrow: AtomicRef::map(orig.borrow, f),
//         }
//     }
// }

// impl<'a, T> Deref for AtomicRef2<'a, T>
// where
//     T: ?Sized,
// {
//     type Target = T;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self.borrow.deref()
//     }
// }

// impl<'a, T> AsRef<T> for AtomicRef2<'a, T> {
//     #[inline]
//     fn as_ref(&self) -> &T {
//         self.borrow.as_ref()
//     }
// }

// //////////////////////////////////////////////////////

// /// Shared reference to a component.
// pub struct AtomicRef3<'a, T>
// where
//     T: ?Sized,
// {
//     root: AtomicBorrowRef<'a>,
//     parent: AtomicBorrowRef<'a>,
//     borrow: AtomicRef<'a, T>,
// }

// impl<'a, T> AtomicRef3<'a, T>
// where
//     T: ?Sized,
// {
//     pub(crate) fn new(
//         root: AtomicBorrowRef<'a>,
//         parent: AtomicBorrowRef<'a>,
//         borrow: AtomicRef<'a, T>,
//     ) -> Self {
//         AtomicRef3 {
//             root,
//             parent,
//             borrow,
//         }
//     }

//     /// Makes a new [`Ref`].
//     ///
//     /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
//     #[inline]
//     pub fn map<U: ?Sized, F: FnOnce(&T) -> &U>(orig: Self, f: F) -> AtomicRef3<'a, U> {
//         AtomicRef3 {
//             root: orig.root,
//             parent: orig.parent,
//             borrow: AtomicRef::map(orig.borrow, f),
//         }
//     }
// }

// impl<'a, T> Deref for AtomicRef3<'a, T>
// where
//     T: ?Sized,
// {
//     type Target = T;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self.borrow.deref()
//     }
// }

// impl<'a, T> AsRef<T> for AtomicRef3<'a, T>
// where
//     T: ?Sized,
// {
//     #[inline]
//     fn as_ref(&self) -> &T {
//         &self.borrow
//     }
// }

// //////////////////////////////////////////////////////

// /// Shared reference to a component.
// pub struct AtomicRefMut2<'a, T>
// where
//     T: ?Sized,
// {
//     parent: AtomicBorrowRef<'a>,
//     borrow: AtomicRefMut<'a, T>,
// }

// impl<'a, T> AtomicRefMut2<'a, T>
// where
//     T: ?Sized,
// {
//     pub(crate) fn new(parent: AtomicBorrowRef<'a>, borrow: AtomicRefMut<'a, T>) -> Self {
//         AtomicRefMut2 { parent, borrow }
//     }

//     // /// Destructures Ref so you can use it in a layered ref
//     // pub(crate) fn destructure(self) -> (&'a T, AtomicBorrowRefMut<'a>, AtomicBorrowRefMut<'a>) {
//     //     let (t, borrow) = self.borrow.destructure();
//     //     (t, borrow, self.parent)
//     // }

//     /// Makes a new [`Ref`].
//     ///
//     /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
//     #[inline]
//     pub fn map<U, F>(orig: AtomicRefMut2<'a, T>, f: F) -> AtomicRefMut2<'a, U>
//     where
//         U: ?Sized,
//         F: FnOnce(&mut T) -> &mut U,
//     {
//         AtomicRefMut2 {
//             parent: orig.parent,
//             borrow: AtomicRefMut::map(orig.borrow, f),
//         }
//     }
// }

// impl<'a, T> Deref for AtomicRefMut2<'a, T> {
//     type Target = T;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self.borrow.deref()
//     }
// }

// impl<'a, T> DerefMut for AtomicRefMut2<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.borrow.deref_mut()
//     }
// }

// impl<'a, T> AsRef<T> for AtomicRefMut2<'a, T> {
//     #[inline]
//     fn as_ref(&self) -> &T {
//         &self.borrow
//     }
// }

// impl<'a, T> AsMut<T> for AtomicRefMut2<'a, T> {
//     fn as_mut(&mut self) -> &mut T {
//         &mut self.borrow
//     }
// }

// //////////////////////////////////////////////////////

// /// Shared reference to a component.
// pub struct AtomicRefMut3<'a, T>
// where
//     T: ?Sized,
// {
//     root: AtomicBorrowRef<'a>,
//     parent: AtomicBorrowRef<'a>,
//     borrow: AtomicRefMut<'a, T>,
// }

// impl<'a, T> AtomicRefMut3<'a, T>
// where
//     T: ?Sized,
// {
//     pub(crate) fn new(
//         root: AtomicBorrowRef<'a>,
//         parent: AtomicBorrowRef<'a>,
//         borrow: AtomicRefMut<'a, T>,
//     ) -> Self {
//         AtomicRefMut3 {
//             root,
//             parent,
//             borrow,
//         }
//     }

//     /// Makes a new [`Ref`].
//     ///
//     /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
//     #[inline]
//     pub fn map<U: ?Sized, F: FnOnce(&mut T) -> &mut U>(orig: Self, f: F) -> AtomicRefMut3<'a, U> {
//         AtomicRefMut3 {
//             root: orig.root,
//             parent: orig.parent,
//             borrow: AtomicRefMut::map(orig.borrow, f),
//         }
//     }
// }

// impl<'a, T> Deref for AtomicRefMut3<'a, T> {
//     type Target = T;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self.borrow.deref()
//     }
// }

// impl<'a, T> DerefMut for AtomicRefMut3<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.borrow.deref_mut()
//     }
// }

// impl<'a, T> AsRef<T> for AtomicRefMut3<'a, T> {
//     #[inline]
//     fn as_ref(&self) -> &T {
//         &self.borrow
//     }
// }

// impl<'a, T> AsMut<T> for AtomicRefMut3<'a, T> {
//     fn as_mut(&mut self) -> &mut T {
//         &mut self.borrow
//     }
// }
