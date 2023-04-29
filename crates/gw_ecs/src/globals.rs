use crate::atomic_refcell::{AtomicBorrowRef, AtomicRefCell};
use crate::shred::World as Resources;
use crate::shred::{
    DefaultIfMissing, Fetch, FetchMut, PanicIfMissing, Resource, ResourceId, SetupHandler,
    SystemData,
};
use crate::World;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Globals {
    resources: Arc<AtomicRefCell<Resources>>,
}

impl Globals {
    pub fn new(resources: Arc<AtomicRefCell<Resources>>) -> Self {
        Globals { resources } // Using shred world, not specs world (no components)
    }

    pub fn has_value<G: Resource>(&self) -> bool {
        self.resources.borrow().has_value::<G>()
    }

    /// Inserts a global
    pub fn insert<G: Resource>(&mut self, global: G) {
        self.resources.borrow_mut().insert(global);
    }

    /// Removes a global
    pub fn remove<G: Resource>(&mut self) -> Option<G> {
        self.resources.borrow_mut().remove::<G>()
    }

    pub fn fetch<'b, G: Resource>(&'b self) -> GlobalFetch<'b, G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.fetch::<G>();
        GlobalFetch::new(borrow, fetch)
    }

    pub fn try_fetch<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.try_fetch::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetch::new(borrow, fetch)),
        }
    }

    pub fn fetch_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.fetch_mut::<G>();
        GlobalFetchMut::new(borrow, fetch)
    }

    pub fn try_fetch_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.try_fetch_mut::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetchMut::new(borrow, fetch)),
        }
    }
}

impl Default for Globals {
    fn default() -> Self {
        Globals {
            resources: Arc::new(AtomicRefCell::new(Resources::empty())),
        }
    }
}

impl Clone for Globals {
    fn clone(&self) -> Self {
        Globals {
            resources: Arc::clone(&self.resources),
        }
    }
}

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<GlobalFetch<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct GlobalFetch<'a, T: 'a> {
    borrow: AtomicBorrowRef<'a>,
    fetch: Fetch<'a, T>,
}

impl<'a, T: 'a> GlobalFetch<'a, T> {
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: Fetch<'a, T>) -> Self {
        GlobalFetch { borrow, fetch }
    }
}

impl<'a, T> Deref for GlobalFetch<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> Clone for GlobalFetch<'a, T> {
    fn clone(&self) -> Self {
        GlobalFetch {
            borrow: AtomicBorrowRef::clone(&self.borrow),
            fetch: self.fetch.clone(),
        }
    }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use
/// `Option<GlobalFetchMut<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct GlobalFetchMut<'a, T: 'a> {
    #[allow(dead_code)]
    borrow: AtomicBorrowRef<'a>,
    fetch: FetchMut<'a, T>,
}

impl<'a, T: 'a> GlobalFetchMut<'a, T> {
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: FetchMut<'a, T>) -> Self {
        GlobalFetchMut { borrow, fetch }
    }
}

impl<'a, T> Deref for GlobalFetchMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> DerefMut for GlobalFetchMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        self.fetch.deref_mut()
    }
}

pub(crate) struct GlobalRes<T>(PhantomData<T>);
pub(crate) struct GlobalSet;

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadGlobal<'a, T: 'a, F = DefaultIfMissing> {
    inner: GlobalFetch<'a, T>,
    phantom: PhantomData<F>,
}

impl<'a, T, F> Deref for ReadGlobal<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T, F> From<GlobalFetch<'a, T>> for ReadGlobal<'a, T, F> {
    fn from(inner: GlobalFetch<'a, T>) -> Self {
        ReadGlobal {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> SystemData<'a> for ReadGlobal<'a, T, F>
where
    T: Resource,
    F: SetupHandler<T>,
{
    fn setup(world: &mut World) {
        F::setup(world)
    }

    fn fetch(world: &'a World) -> Self {
        world.fetch_global::<T>().into()
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<GlobalSet>(),
            ResourceId::new::<GlobalRes<T>>(),
        ]
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
pub struct WriteGlobal<'a, T: 'a, F = DefaultIfMissing> {
    inner: GlobalFetchMut<'a, T>,
    phantom: PhantomData<F>,
}

impl<'a, T, F> Deref for WriteGlobal<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T, F> DerefMut for WriteGlobal<'a, T, F>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<'a, T, F> From<GlobalFetchMut<'a, T>> for WriteGlobal<'a, T, F> {
    fn from(inner: GlobalFetchMut<'a, T>) -> Self {
        WriteGlobal {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> SystemData<'a> for WriteGlobal<'a, T, F>
where
    T: Resource,
    F: SetupHandler<T>,
{
    fn setup(world: &mut World) {
        F::setup(world)
    }

    fn fetch(world: &'a World) -> Self {
        world.fetch_global_mut::<T>().into()
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<GlobalSet>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<GlobalRes<T>>()]
    }
}

// ------------------

impl<'a, T, F> SystemData<'a> for Option<ReadGlobal<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        world.try_fetch_global().map(Into::into)
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<GlobalSet>(),
            ResourceId::new::<GlobalRes<T>>(),
        ]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

impl<'a, T, F> SystemData<'a> for Option<WriteGlobal<'a, T, F>>
where
    T: Resource,
{
    fn setup(_: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        world.try_fetch_global_mut().map(Into::into)
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<GlobalSet>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<GlobalRes<T>>()]
    }
}

/// Allows to fetch a resource in a system immutably.
/// **This will panic if the resource does not exist.**
/// Usage of `Read` or `Option<Read>` is therefore recommended.
pub type ReadGlobalExpect<'a, T> = ReadGlobal<'a, T, PanicIfMissing>;

/// Allows to fetch a resource in a system mutably.
/// **This will panic if the resource does not exist.**
/// Usage of `Write` or `Option<Write>` is therefore recommended.
pub type WriteGlobalExpect<'a, T> = WriteGlobal<'a, T, PanicIfMissing>;

/////////////////////////////////////////////////////////////

// pub trait WorldGlobals {
//     fn fetch_global<G: Resource>(&self) -> GlobalFetch<G> {
//         self.try_fetch_global::<G>().unwrap()
//     }

//     fn try_fetch_global<G: Resource>(&self) -> Option<GlobalFetch<G>>;

//     fn fetch_global_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
//         self.try_fetch_global_mut::<G>().unwrap()
//     }

//     fn try_fetch_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>>;
// }

// impl WorldGlobals for World {
//     fn try_fetch_global<G: Resource>(&self) -> Option<GlobalFetch<G>> {
//         let (globals, borrow) = self.fetch::<Globals>().destructure();
//         match globals.try_fetch::<G>() {
//             None => None,
//             Some(fetch) => Some(GlobalFetch::new(borrow, fetch)),
//         }
//     }

//     fn try_fetch_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
//         let (globals, borrow) = self.fetch::<Globals>().destructure();
//         match globals.try_fetch_mut::<G>() {
//             None => None,
//             Some(fetch) => Some(GlobalFetchMut::new(borrow, fetch)),
//         }
//     }
// }
