use crate::bevy::{ReadOnlySystemParam, SystemParam};
use crate::shred::{Resource, ResourceId, SetupDefault, SetupHandler, SystemData};
use crate::World;
use std::collections::HashSet;
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{ResMut, ResRef};

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadRes<'a, T: 'a, F = ()> {
    inner: ResRef<'a, T>,
    phantom: PhantomData<F>,
}

unsafe impl<'a, T, F> ReadOnlySystemParam for ReadRes<'a, T, F> where T: Resource {}

impl<'a, T, F> Deref for ReadRes<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T, F> From<ResRef<'a, T>> for ReadRes<'a, T, F> {
    fn from(inner: ResRef<'a, T>) -> Self {
        ReadRes {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> SystemData<'a> for ReadRes<'a, T, F>
where
    T: Resource,
    F: SetupHandler<T>,
{
    fn setup(world: &mut World) {
        F::setup(world)
    }

    fn fetch(world: &'a World) -> Self {
        ReadRes::<'a, T, F> {
            inner: world.read_resource::<T>(),
            phantom: PhantomData,
        }
    }

    fn reads() -> HashSet<ResourceId> {
        let mut reads = HashSet::new();
        reads.insert(ResourceId::new::<T>());
        reads
    }

    // fn writes() -> Vec<ResourceId> {
    //     vec![]
    // }
}

unsafe impl<'a, T, F> SystemParam for ReadRes<'a, T, F>
where
    T: Resource,
{
    type State = ();
    type Item<'world, 'state> = ReadRes<'world, T, F>;

    fn init_state(_world: &mut World, _system_meta: &mut crate::bevy::SystemMeta) -> Self::State {
        ()
    }

    fn apply(_state: &mut Self::State, _system_meta: &crate::bevy::SystemMeta, _world: &mut World) {
    }

    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &crate::bevy::SystemMeta,
        world: &'world World,
        _change_tick: u32,
    ) -> Self::Item<'world, 'state> {
        world.read_resource::<T>().into()
    }
}

impl<'a, T> Debug for ReadRes<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
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
pub struct WriteRes<'a, T: 'a, F = ()> {
    inner: ResMut<'a, T>,
    phantom: PhantomData<F>,
}

impl<'a, T, F> Deref for WriteRes<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T, F> DerefMut for WriteRes<'a, T, F>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<'a, T, F> From<ResMut<'a, T>> for WriteRes<'a, T, F> {
    fn from(inner: ResMut<'a, T>) -> Self {
        WriteRes {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> SystemData<'a> for WriteRes<'a, T, F>
where
    T: Resource,
    F: SetupHandler<T>,
{
    fn setup(world: &mut World) {
        F::setup(world)
    }

    fn fetch(world: &'a World) -> Self {
        WriteRes::<'a, T, F> {
            inner: world.write_resource::<T>(),
            phantom: PhantomData,
        }
    }

    // fn reads() -> Vec<ResourceId> {
    //     vec![]
    // }

    fn writes() -> HashSet<ResourceId> {
        let mut writes = HashSet::new();
        writes.insert(ResourceId::new::<T>());
        writes
    }
}

impl<'a, T> Debug for WriteRes<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

// ------------------

impl<'a, T, F> SystemData<'a> for Option<ReadRes<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        match world.try_read_resource::<T>() {
            None => None,
            Some(fetch) => Some(ReadRes::<'a, T, F> {
                inner: fetch,
                phantom: PhantomData,
            }),
        }
    }

    fn reads() -> HashSet<ResourceId> {
        let mut reads = HashSet::new();
        reads.insert(ResourceId::new::<T>());
        reads
    }

    // fn writes() -> Vec<ResourceId> {
    //     vec![]
    // }
}

impl<'a, T, F> SystemData<'a> for Option<WriteRes<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        match world.try_write_resource::<T>() {
            None => None,
            Some(fetch) => Some(WriteRes::<'a, T, F> {
                inner: fetch,
                phantom: PhantomData,
            }),
        }
    }

    // fn reads() -> Vec<ResourceId> {
    //     vec![]
    // }

    fn writes() -> HashSet<ResourceId> {
        let mut writes = HashSet::new();
        writes.insert(ResourceId::new::<T>());
        writes
    }
}

/// Allows to optionally fetch a resource in a system immutably.
pub type TryReadRes<'a, T> = Option<ReadRes<'a, T>>;

/// Allows to optionally fetch a resource in a system mutably.
pub type TryWriteRes<'a, T> = Option<WriteRes<'a, T>>;

/// Allows to fetch a resource in a system immutably.
/// **This will add a default value in a `System` setup if one does not exist.**
pub type ReadResSetup<'a, T> = ReadRes<'a, T, SetupDefault>;

/// Allows to fetch a resource in a system mutably.
/// **This will add a default value in a `System` setup if the resource does not exist.**
pub type WriteResSetup<'a, T> = WriteRes<'a, T, SetupDefault>;
