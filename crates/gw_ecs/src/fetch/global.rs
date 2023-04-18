use super::ReadOnly;
use super::{Fetch, MaybeBorrowed};
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::resource::Resource;
use crate::Ecs;
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value
#[derive(Debug)]
pub struct Global<'a, T: Resource> {
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

impl<'b, T> MaybeBorrowed for Global<'b, T>
where
    T: Resource,
{
    type Output<'a> = Global<'a, T>;
}

impl<'a, T> Fetch for Global<'a, T>
where
    T: Resource,
{
    fn fetch(ecs: &Ecs) -> Global<'_, T> {
        ecs.get_global::<T>().unwrap()
    }
}

impl<T> Clone for Global<'_, T>
where
    T: Resource,
{
    fn clone(&self) -> Self {
        Global {
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<T> ReadOnly for Global<'_, T> where T: Resource {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<T> Deref for Global<'_, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &'_ T {
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

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<T> MaybeBorrowed for Option<Global<'_, T>>
where
    T: Resource,
{
    type Output<'a> = Option<Global<'a, T>>;
}

impl<'a, T> Fetch for Option<Global<'a, T>>
where
    T: Resource,
{
    fn fetch(ecs: &Ecs) -> Option<Global<'_, T>> {
        ecs.get_global::<T>()
    }
}

pub type TryGlobal<'a, T> = Option<Global<'a, T>>;

// /////////////////////////////////////////////////////////////////////
// /////////////////////////////////////////////////////////////////////

/// Reference to a global value
#[derive(Debug)]
pub struct GlobalMut<'b, T: Resource> {
    borrow: AtomicRefMut<'b, T>,
}

impl<'a, T> GlobalMut<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicRefMut<'a, T>) -> Self {
        GlobalMut { borrow }
    }
}

impl<'b, T> MaybeBorrowed for GlobalMut<'b, T>
where
    T: Resource,
{
    type Output<'a> = GlobalMut<'a, T>;
}

impl<'a, T> Fetch for GlobalMut<'a, T>
where
    T: Resource,
{
    fn fetch(ecs: &Ecs) -> GlobalMut<'_, T> {
        ecs.get_global_mut::<T>().unwrap()
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<T> Deref for GlobalMut<'_, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &'_ T {
        self.borrow.deref()
    }
}

impl<T> DerefMut for GlobalMut<'_, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
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

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<T> MaybeBorrowed for Option<GlobalMut<'_, T>>
where
    T: Resource,
{
    type Output<'a> = Option<GlobalMut<'a, T>>;
}

impl<'a, T> Fetch for Option<GlobalMut<'a, T>>
where
    T: Resource,
{
    fn fetch(ecs: &Ecs) -> Option<GlobalMut<'_, T>> {
        ecs.get_global_mut::<T>()
    }
}

pub type TryGlobalMut<'a, T> = Option<GlobalMut<'a, T>>;
