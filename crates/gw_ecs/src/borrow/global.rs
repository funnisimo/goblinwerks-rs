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

impl<'e, T> BorrowRef<'e> for Global<'e, T>
where
    T: Resource,
{
    type Output = Global<'e, T>;

    fn borrow(ecs: &'e Ecs) -> Self::Output {
        ecs.get_global::<T>().unwrap()
    }
}

impl<'e, T> BorrowMut<'e> for Global<'e, T>
where
    T: Resource,
{
    type Output = Global<'e, T>;

    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_global::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'e, T> BorrowRef<'e> for Option<Global<'e, T>>
where
    T: Resource,
{
    type Output = Option<Global<'e, T>>;

    fn borrow(ecs: &'e Ecs) -> Self::Output {
        ecs.get_global::<T>()
    }
}

impl<'e, T> BorrowMut<'e> for Option<Global<'e, T>>
where
    T: Resource,
{
    type Output = Option<Global<'e, T>>;

    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
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

impl<'e, T> BorrowMut<'e> for GlobalMut<'e, T>
where
    T: Resource,
{
    type Output = GlobalMut<'e, T>;

    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_global_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

impl<'e, T> BorrowMut<'e> for Option<GlobalMut<'e, T>>
where
    T: Resource,
{
    type Output = Option<GlobalMut<'e, T>>;

    fn borrow_mut(ecs: &'e Ecs) -> Self::Output {
        ecs.get_global_mut::<T>()
    }
}

pub type TryGlobalMut<'a, T> = Option<GlobalMut<'a, T>>;
