use super::ReadOnly;
use super::{BorrowMut, BorrowRef};
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::resource::Resource;
use crate::Ecs;
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Unique<'a, T>
where
    T: Resource,
{
    _levels: AtomicBorrowRef<'a>,
    _level: AtomicBorrowRef<'a>,
    borrow: AtomicRef<'a, T>,
}

impl<'a, T> Unique<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'a>,
        level: AtomicBorrowRef<'a>,
        borrow: AtomicRef<'a, T>,
    ) -> Self {
        Unique {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

impl<'a, T> Clone for Unique<'a, T>
where
    T: Resource,
{
    fn clone(&self) -> Self {
        Unique {
            _levels: AtomicBorrowRef::clone(&self._levels),
            _level: AtomicBorrowRef::clone(&self._level),
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'a, T> ReadOnly for Unique<'a, T> where T: Resource {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for Unique<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> AsRef<T> for Unique<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'a, T> BorrowRef<'a> for Unique<'a, T>
where
    T: Resource,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_unique::<T>().unwrap()
    }
}

impl<'a, T> BorrowMut<'a> for Unique<'a, T>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_unique::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowRef<'a> for Option<Unique<'a, T>>
where
    T: Resource,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_unique::<T>()
    }
}

impl<'a, T> BorrowMut<'a> for Option<Unique<'a, T>>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_unique::<T>()
    }
}

pub type TryUnique<'a, T> = Option<Unique<'a, T>>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value

pub struct UniqueMut<'a, T>
where
    T: Resource,
{
    _levels: AtomicBorrowRef<'a>,
    _level: AtomicBorrowRef<'a>,
    borrow: AtomicRefMut<'a, T>,
}

impl<'a, T> UniqueMut<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'a>,
        level: AtomicBorrowRef<'a>,
        borrow: AtomicRefMut<'a, T>,
    ) -> Self {
        UniqueMut {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for UniqueMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> DerefMut for UniqueMut<'a, T>
where
    T: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.borrow.deref_mut()
    }
}

impl<'a, T> AsRef<T> for UniqueMut<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'a, T> AsMut<T> for UniqueMut<'a, T>
where
    T: Resource,
{
    fn as_mut(&mut self) -> &mut T {
        self.borrow.as_mut()
    }
}

impl<'a, T> BorrowMut<'a> for UniqueMut<'a, T>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_unique_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowMut<'a> for Option<UniqueMut<'a, T>>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_unique_mut::<T>()
    }
}

pub type TryUniqueMut<'a, T> = Option<UniqueMut<'a, T>>;
