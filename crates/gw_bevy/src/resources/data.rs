use super::Resource;
use crate::atomic_refcell::{AtomicRef, AtomicRefMut};
use crate::prelude::DetectChanges;
use crate::tick::ComponentTicks;

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
    pub(crate) world_tick: u32,
    pub(crate) last_system_tick: u32,
}

impl<'a, T> ResRef<'a, T>
where
    T: Resource,
{
    pub fn new(
        data: AtomicRef<'a, T>,
        ticks: AtomicRef<'a, ComponentTicks>,
        world_tick: u32,
        last_system_tick: u32,
    ) -> Self {
        ResRef {
            data,
            ticks,
            world_tick,
            last_system_tick,
        }
    }
}

impl<'a, T> DetectChanges for ResRef<'a, T> {
    /// Returns `true` if this value was added after the system last ran.
    fn is_added(&self) -> bool {
        self.ticks.is_added(self.last_system_tick, self.world_tick)
    }

    /// Returns `true` if this value was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.ticks
            .is_changed(self.last_system_tick, self.world_tick)
    }

    /// Returns the change tick recording the time this data was most recently changed.
    ///
    /// Note that components and resources are also marked as changed upon insertion.
    ///
    /// For comparison, the previous change tick of a system can be read using the
    /// [`SystemChangeTick`](crate::system::SystemChangeTick)
    /// [`SystemParam`](crate::system::SystemParam).
    fn last_changed(&self) -> u32 {
        self.ticks.changed.tick
    }
}

// impl<'a, T> Deref for ResRef<'a, T>
// where
//     T: Resource,
// {
//     type Target = T;

//     fn deref(&self) -> &T {
//         self.data.deref()
//     }
// }

impl<'a, T> Clone for ResRef<'a, T> {
    fn clone(&self) -> Self {
        ResRef {
            data: AtomicRef::clone(&self.data),
            ticks: AtomicRef::clone(&self.ticks),
            world_tick: self.world_tick,
            last_system_tick: self.last_system_tick,
        }
    }
}

// impl<'a, T> Debug for ResRef<'a, T>
// where
//     T: Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.data)
//     }
// }

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use `Option<Write<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ResMut<'a, T: 'a> {
    pub(crate) data: AtomicRefMut<'a, T>,
    pub(crate) ticks: AtomicRefMut<'a, ComponentTicks>,
    pub(crate) world_tick: u32,
    pub(crate) last_system_tick: u32,
}

impl<'a, T> ResMut<'a, T>
where
    T: Resource,
{
    pub fn new(
        data: AtomicRefMut<'a, T>,
        ticks: AtomicRefMut<'a, ComponentTicks>,
        world_tick: u32,
        last_system_tick: u32,
    ) -> Self {
        ResMut {
            data,
            ticks,
            world_tick,
            last_system_tick,
        }
    }
}

impl<'a, T> DetectChanges for ResMut<'a, T> {
    /// Returns `true` if this value was added after the system last ran.
    fn is_added(&self) -> bool {
        self.ticks.is_added(self.last_system_tick, self.world_tick)
    }

    /// Returns `true` if this value was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.ticks
            .is_changed(self.last_system_tick, self.world_tick)
    }

    /// Returns the change tick recording the time this data was most recently changed.
    ///
    /// Note that components and resources are also marked as changed upon insertion.
    ///
    /// For comparison, the previous change tick of a system can be read using the
    /// [`SystemChangeTick`](crate::system::SystemChangeTick)
    /// [`SystemParam`](crate::system::SystemParam).
    fn last_changed(&self) -> u32 {
        self.ticks.changed.tick
    }
}

// impl<'a, T> Deref for ResMut<'a, T>
// where
//     T: Resource,
// {
//     type Target = T;

//     fn deref(&self) -> &T {
//         self.data.deref()
//     }
// }

// impl<'a, T> DerefMut for ResMut<'a, T>
// where
//     T: Resource,
// {
//     fn deref_mut(&mut self) -> &mut T {
//         self.ticks.changed.set_changed(self.current_tick);
//         self.data.deref_mut()
//     }
// }

// impl<'a, T> Debug for ResMut<'a, T>
// where
//     T: Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.data)
//     }
// }
