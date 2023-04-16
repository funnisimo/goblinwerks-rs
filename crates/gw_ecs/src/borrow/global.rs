use super::ReadOnly;
use super::{BorrowMut, BorrowRef};
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::resource::Resource;
use crate::Ecs;
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value
#[derive(Debug)]
pub struct Global<'a, T>
where
    T: Resource,
{
    borrow: AtomicRef<'a, T>,
}

impl<'a, T> Global<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicRef<'a, T>) -> Self {
        Global { borrow }
    }

    pub(crate) fn destructure(self) -> (&'a T, AtomicBorrowRef<'a>) {
        self.borrow.destructure()
    }
}

impl<'a, T> Clone for Global<'a, T>
where
    T: Resource,
{
    fn clone(&self) -> Self {
        Global {
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'a, T> ReadOnly for Global<'a, T> where T: Resource {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for Global<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> AsRef<T> for Global<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'a, T> BorrowRef<'a> for Global<'a, T>
where
    T: Resource,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_global::<T>().unwrap()
    }
}

impl<'a, T> BorrowMut<'a> for Global<'a, T>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_global::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowRef<'a> for Option<Global<'a, T>>
where
    T: Resource,
{
    fn borrow(ecs: &'a Ecs) -> Self {
        ecs.get_global::<T>()
    }
}

impl<'a, T> BorrowMut<'a> for Option<Global<'a, T>>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_global::<T>()
    }
}

pub type TryGlobal<'a, T> = Option<Global<'a, T>>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value
#[derive(Debug)]
pub struct GlobalMut<'a, T>
where
    T: Resource,
{
    borrow: AtomicRefMut<'a, T>,
}

impl<'a, T> GlobalMut<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicRefMut<'a, T>) -> Self {
        GlobalMut { borrow }
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for GlobalMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> DerefMut for GlobalMut<'a, T>
where
    T: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.borrow.deref_mut()
    }
}

impl<'a, T> AsRef<T> for GlobalMut<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'a, T> AsMut<T> for GlobalMut<'a, T>
where
    T: Resource,
{
    fn as_mut(&mut self) -> &mut T {
        self.borrow.as_mut()
    }
}

impl<'a, T> BorrowMut<'a> for GlobalMut<'a, T>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_global_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'a, T> BorrowMut<'a> for Option<GlobalMut<'a, T>>
where
    T: Resource,
{
    fn borrow_mut(ecs: &'a Ecs) -> Self {
        ecs.get_global_mut::<T>()
    }
}

pub type TryGlobalMut<'a, T> = Option<GlobalMut<'a, T>>;
