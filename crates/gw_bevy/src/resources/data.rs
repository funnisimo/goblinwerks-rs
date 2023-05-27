use super::Resource;
use super::{ChangeTicks, Ticks};
use crate::atomic_refcell::{AtomicRef, AtomicRefMut};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ResRef<'a, T: 'a> {
    pub(crate) data: AtomicRef<'a, T>,
    pub(crate) ticks: AtomicRef<'a, ChangeTicks>,
}

impl<'a, T> ResRef<'a, T>
where
    T: Resource,
{
    pub fn new(data: AtomicRef<'a, T>, ticks: AtomicRef<'a, ChangeTicks>) -> Self {
        ResRef { data, ticks }
    }

    pub fn inserted(&self) -> Ticks {
        self.ticks.inserted
    }
    pub fn updated(&self) -> Ticks {
        self.ticks.updated
    }
}

impl<'a, T> Deref for ResRef<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.data.deref()
    }
}

impl<'a, T> Clone for ResRef<'a, T> {
    fn clone(&self) -> Self {
        ResRef {
            data: AtomicRef::clone(&self.data),
            ticks: AtomicRef::clone(&self.ticks),
        }
    }
}

impl<'a, T> Debug for ResRef<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use `Option<Write<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ResMut<'a, T: 'a> {
    data: AtomicRefMut<'a, T>,
    ticks: AtomicRefMut<'a, ChangeTicks>,
    current: Ticks,
}

impl<'a, T> ResMut<'a, T>
where
    T: Resource,
{
    pub fn new(
        data: AtomicRefMut<'a, T>,
        ticks: AtomicRefMut<'a, ChangeTicks>,
        current: Ticks,
    ) -> Self {
        ResMut {
            data,
            ticks,
            current,
        }
    }

    pub fn inserted(&self) -> Ticks {
        self.ticks.inserted
    }
    pub fn updated(&self) -> Ticks {
        self.ticks.updated
    }
}

impl<'a, T> Deref for ResMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.data.deref()
    }
}

impl<'a, T> DerefMut for ResMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        self.ticks.updated = self.current;
        self.data.deref_mut()
    }
}

impl<'a, T> Debug for ResMut<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
