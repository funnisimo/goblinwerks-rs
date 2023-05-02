use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::World;
use crate::{
    shred::{
        DefaultIfMissing, Fetch, FetchMut, PanicIfMissing, Resource, ResourceId, SetupHandler,
        SystemData,
    },
    world::UnsafeWorld,
};

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadRes<'a, T: 'a, F = DefaultIfMissing> {
    inner: Fetch<'a, T>,
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

impl<'a, T, F> From<Fetch<'a, T>> for ReadRes<'a, T, F> {
    fn from(inner: Fetch<'a, T>) -> Self {
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

    fn fetch(world: &UnsafeWorld<'a>) -> Self {
        ReadRes::<'a, T, F> {
            inner: world.read_resource::<T>().inner,
            phantom: PhantomData,
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
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
pub struct WriteRes<'a, T: 'a, F = DefaultIfMissing> {
    inner: FetchMut<'a, T>,
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

impl<'a, T, F> From<FetchMut<'a, T>> for WriteRes<'a, T, F> {
    fn from(inner: FetchMut<'a, T>) -> Self {
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

    fn fetch(world: &UnsafeWorld<'a>) -> Self {
        WriteRes::<'a, T, F> {
            inner: world.write_resource::<T>().inner,
            phantom: PhantomData,
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }
}

// ------------------

impl<'a, T, F> SystemData<'a> for Option<ReadRes<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &UnsafeWorld<'a>) -> Self {
        match world.try_read_resource::<T>() {
            None => None,
            Some(fetch) => Some(ReadRes::<'a, T, F> {
                inner: fetch.inner,
                phantom: PhantomData,
            }),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

impl<'a, T, F> SystemData<'a> for Option<WriteRes<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &UnsafeWorld<'a>) -> Self {
        match world.try_write_resource::<T>() {
            None => None,
            Some(fetch) => Some(WriteRes::<'a, T, F> {
                inner: fetch.inner,
                phantom: PhantomData,
            }),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }
}

/// Allows to fetch a resource in a system immutably.
/// **This will panic if the resource does not exist.**
/// Usage of `Read` or `Option<Read>` is therefore recommended.
pub type ReadResExpect<'a, T> = ReadRes<'a, T, PanicIfMissing>;

/// Allows to fetch a resource in a system mutably.
/// **This will panic if the resource does not exist.**
/// Usage of `Write` or `Option<Write>` is therefore recommended.
pub type WriteResExpect<'a, T> = WriteRes<'a, T, PanicIfMissing>;
