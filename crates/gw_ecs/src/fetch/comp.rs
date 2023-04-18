use super::Fetch;
use super::ReadOnly;
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::storage::SparseSet;
use crate::{Component, Ecs};
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Comp<'b, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'b>,
    _level: AtomicBorrowRef<'b>,
    borrow: AtomicRef<'b, SparseSet<T>>,
}

impl<'b, T> Comp<'b, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'b>,
        level: AtomicBorrowRef<'b>,
        borrow: AtomicRef<'b, SparseSet<T>>,
    ) -> Self {
        Comp {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

impl<'b, T> Clone for Comp<'b, T>
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

impl<'b, T> ReadOnly for Comp<'b, T> where T: Component {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'b, T> Deref for Comp<'b, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'b, T> AsRef<SparseSet<T>> for Comp<'b, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

// impl<T> MaybeBorrowed for Comp<'_, T>
// where
//     T: Component,
// {
//     type Output<'a> = Comp<'a, T>;
// }

impl<T> Fetch for Comp<'_, T>
where
    T: Component,
{
    type Output<'a> = Comp<'a, T>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

// impl<T> MaybeBorrowed for Option<Comp<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<Comp<'a, T>>;
// }

impl<T> Fetch for Option<Comp<'_, T>>
where
    T: Component,
{
    type Output<'a> = Option<Comp<'a, T>>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component::<T>()
    }
}

pub type TryComp<'a, T> = Option<Comp<'a, T>>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value

pub struct CompMut<'b, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'b>,
    _level: AtomicBorrowRef<'b>,
    borrow: AtomicRefMut<'b, SparseSet<T>>,
}

impl<'b, T> CompMut<'b, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'b>,
        level: AtomicBorrowRef<'b>,
        borrow: AtomicRefMut<'b, SparseSet<T>>,
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

impl<'b, T> Deref for CompMut<'b, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'b, T> DerefMut for CompMut<'b, T>
where
    T: Component,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.deref_mut()
    }
}

impl<'b, T> AsRef<SparseSet<T>> for CompMut<'b, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

impl<'b, T> AsMut<SparseSet<T>> for CompMut<'b, T>
where
    T: Component,
{
    fn as_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.as_mut()
    }
}

// impl<T> MaybeBorrowed for CompMut<'_, T>
// where
//     T: Component,
// {
//     type Output<'a> = CompMut<'a, T>;
// }

impl<T> Fetch for CompMut<'_, T>
where
    T: Component,
{
    type Output<'a> = CompMut<'a, T>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

// impl<T> MaybeBorrowed for Option<CompMut<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<CompMut<'a, T>>;
// }

impl<T> Fetch for Option<CompMut<'_, T>>
where
    T: Component,
{
    type Output<'a> = Option<CompMut<'a, T>>;

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component_mut::<T>()
    }
}

pub type TryCompMut<'a, T> = Option<CompMut<'a, T>>;
