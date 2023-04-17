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

impl<'e, T> BorrowRef<'e> for Unique<'e, T>
where
    T: Resource,
{
    type Output = Unique<'e, T>;

    fn borrow(ecs: &'e Ecs) -> Self::Output {
        let (levels, root) = ecs.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<T>().unwrap();
        Unique::new(root, parent, borrow)

        // ecs.get_unique::<T>().unwrap()
    }
}

impl<'e, T> BorrowMut<'e> for Unique<'e, T>
where
    T: Resource,
{
    type Output = Unique<'e, T>;
    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_unique::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'e, T> BorrowRef<'e> for Option<Unique<'e, T>>
where
    T: Resource,
{
    type Output = Option<Unique<'e, T>>;

    fn borrow(ecs: &'e Ecs) -> Self::Output {
        ecs.get_unique::<T>()
    }
}

impl<'e, T> BorrowMut<'e> for Option<Unique<'e, T>>
where
    T: Resource,
{
    type Output = Option<Unique<'e, T>>;

    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
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

impl<'e, T> BorrowMut<'e> for UniqueMut<'e, T>
where
    T: Resource,
{
    type Output = UniqueMut<'e, T>;
    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_unique_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'e, T> BorrowMut<'e> for Option<UniqueMut<'e, T>>
where
    T: Resource,
{
    type Output = Option<UniqueMut<'e, T>>;
    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_unique_mut::<T>()
    }
}

pub type TryUniqueMut<'a, T> = Option<UniqueMut<'a, T>>;
