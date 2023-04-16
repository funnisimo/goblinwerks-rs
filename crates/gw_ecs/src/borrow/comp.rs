use super::ReadOnly;
use super::{BorrowMut, BorrowRef};
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::storage::SparseSet;
use crate::{Component, Ecs};
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Comp<'a, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'a>,
    _level: AtomicBorrowRef<'a>,
    borrow: AtomicRef<'a, SparseSet<T>>,
}

impl<'a, T> Comp<'a, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'a>,
        level: AtomicBorrowRef<'a>,
        borrow: AtomicRef<'a, SparseSet<T>>,
    ) -> Self {
        Comp {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

impl<'a, T> Clone for Comp<'a, T>
where
    T: Component,
{
    fn clone(&self) -> Self {
        Comp {
            _levels: AtomicBorrowRef::clone(&self._levels),
            _level: AtomicBorrowRef::clone(&self._level),
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'a, T> ReadOnly for Comp<'a, T> where T: Component {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for Comp<'a, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'a, T> AsRef<SparseSet<T>> for Comp<'a, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

impl<'a, T> BorrowRef<'a> for Comp<'a, T>
where
    T: Component,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_component::<T>().unwrap()
    }
}

impl<'a, T> BorrowMut<'a> for Comp<'a, T>
where
    T: Component,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_component::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowRef<'a> for Option<Comp<'a, T>>
where
    T: Component,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_component::<T>()
    }
}

impl<'a, T> BorrowMut<'a> for Option<Comp<'a, T>>
where
    T: Component,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_component::<T>()
    }
}

pub type TryComp<'a, T> = Option<Comp<'a, T>>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value

pub struct CompMut<'a, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'a>,
    _level: AtomicBorrowRef<'a>,
    borrow: AtomicRefMut<'a, SparseSet<T>>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'a>,
        level: AtomicBorrowRef<'a>,
        borrow: AtomicRefMut<'a, SparseSet<T>>,
    ) -> Self {
        CompMut {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for CompMut<'a, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'a, T> DerefMut for CompMut<'a, T>
where
    T: Component,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.deref_mut()
    }
}

impl<'a, T> AsRef<SparseSet<T>> for CompMut<'a, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

impl<'a, T> AsMut<SparseSet<T>> for CompMut<'a, T>
where
    T: Component,
{
    fn as_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.as_mut()
    }
}

impl<'a, T> BorrowMut<'a> for CompMut<'a, T>
where
    T: Component,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_component_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowMut<'a> for Option<CompMut<'a, T>>
where
    T: Component,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_component_mut::<T>()
    }
}

pub type TryCompMut<'a, T> = Option<CompMut<'a, T>>;
