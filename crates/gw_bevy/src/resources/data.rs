use super::Resource;
use crate::atomic_refcell::{AtomicRef, AtomicRefMut};
use crate::component::ComponentTicks;
use crate::component::Tick;
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
    pub(crate) ticks: AtomicRef<'a, ComponentTicks>,
}

impl<'a, T> ResRef<'a, T>
where
    T: Resource,
{
    pub fn new(data: AtomicRef<'a, T>, ticks: AtomicRef<'a, ComponentTicks>) -> Self {
        ResRef { data, ticks }
    }

    pub fn inserted_tick(&self) -> Tick {
        self.ticks.added
    }
    pub fn updated_tick(&self) -> Tick {
        self.ticks.changed
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
    pub(crate) ticks: AtomicRefMut<'a, ComponentTicks>,
    current_tick: u32,
}

impl<'a, T> ResMut<'a, T>
where
    T: Resource,
{
    pub fn new(
        data: AtomicRefMut<'a, T>,
        ticks: AtomicRefMut<'a, ComponentTicks>,
        current_tick: u32,
    ) -> Self {
        ResMut {
            data,
            ticks,
            current_tick,
        }
    }

    pub fn inserted_tick(&self) -> Tick {
        self.ticks.added
    }
    pub fn updated_tick(&self) -> Tick {
        self.ticks.changed
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
        self.ticks.changed.set_changed(self.current_tick);
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
