use crate::atomic_refcell::{AtomicRef, AtomicRefMut};
use crate::shred::{Resource, ResourceId, SetupDefault, SetupHandler, SystemData};
use crate::World;
use std::collections::HashSet;
use std::{
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