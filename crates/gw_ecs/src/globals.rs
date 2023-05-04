use crate::atomic_refcell::{AtomicBorrowRef, AtomicRef, AtomicRefCell, AtomicRefMut};
use crate::shred::Resources;
use crate::shred::{
    DefaultIfMissing, PanicIfMissing, Resource, ResourceId, SetupHandler, SystemData,
};
use crate::World;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Globals {
    resources: Arc<AtomicRefCell<Resources>>,
}

impl Globals {
    pub fn new() -> Self {
        Globals {
            resources: Arc::new(AtomicRefCell::new(Resources::empty())),
        }
    }

    pub fn empty() -> Self {
        Globals::new()
    }

    /// Returns true if the resource is in the Globals
    pub fn has_value<G: Resource>(&self) -> bool {
        self.resources.borrow().contains::<G>()
    }

    /// Ensures that the resource is in the Globals or enters
    /// the value from the function.
    pub fn ensure_with<G: Resource, F: FnOnce() -> G>(&mut self, func: F) {
        self.resources.borrow_mut().ensure(func);
    }

    /// Inserts a global
    pub fn insert<G: Resource>(&mut self, global: G) {
        self.resources.borrow_mut().insert(global);
    }

    /// Removes a global
    pub fn remove<G: Resource>(&mut self) -> Option<G> {
        self.resources.borrow_mut().remove::<G>()
    }

    pub fn fetch<'b, G: Resource>(&'b self) -> GlobalRef<'b, G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.get::<G>().unwrap();
        GlobalRef::new(borrow, fetch)
    }

    pub(crate) fn try_fetch<G: Resource>(&self) -> Option<GlobalRef<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.get::<G>() {
            None => None,
            Some(fetch) => Some(GlobalRef::new(borrow, fetch)),
        }
    }

    pub fn fetch_mut<G: Resource>(&self) -> GlobalRefMut<G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.get_mut::<G>().unwrap();
        GlobalRefMut::new(borrow, fetch)
    }

    pub fn try_fetch_mut<G: Resource>(&self) -> Option<GlobalRefMut<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.get_mut::<G>() {
            None => None,
            Some(fetch) => Some(GlobalRefMut::new(borrow, fetch)),
        }
    }
}

impl Default for Globals {
    fn default() -> Self {
        Globals::new()
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
pub struct GlobalRef<'a, T: 'a> {
    borrow: AtomicBorrowRef<'a>,
    fetch: AtomicRef<'a, T>,
}

impl<'a, T: 'a> GlobalRef<'a, T> {
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: AtomicRef<'a, T>) -> Self {
        GlobalRef { borrow, fetch }
    }
}

impl<'a, T> Deref for GlobalRef<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> Clone for GlobalRef<'a, T> {
    fn clone(&self) -> Self {
        GlobalRef {
            borrow: AtomicBorrowRef::clone(&self.borrow),
            fetch: AtomicRef::clone(&self.fetch),
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
pub struct GlobalRefMut<'a, T: 'a> {
    #[allow(dead_code)]
    borrow: AtomicBorrowRef<'a>,
    fetch: AtomicRefMut<'a, T>,
}

impl<'a, T: 'a> GlobalRefMut<'a, T> {
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: AtomicRefMut<'a, T>) -> Self {
        GlobalRefMut { borrow, fetch }
    }
}

impl<'a, T> Deref for GlobalRefMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> DerefMut for GlobalRefMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        self.fetch.deref_mut()
    }
}

pub(crate) struct GlobalRes<T>(PhantomData<T>);
// pub(crate) struct GlobalSet;

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadGlobal<'a, T: 'a, F = DefaultIfMissing> {
    fetch: GlobalRef<'a, T>,
    phantom: PhantomData<F>,
}

// impl<'a, T, F> ReadGlobal<'a, T, F>
// where
//     T: Resource,
// {
//     fn new(fetch: GlobalFetch<'a, T>) -> Self {
//         ReadGlobal {
//             fetch,
//             phantom: PhantomData,
//         }
//     }
// }

impl<'a, T, F> Deref for ReadGlobal<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.fetch
    }
}

impl<'a, T, F> From<GlobalRef<'a, T>> for ReadGlobal<'a, T, F> {
    fn from(fetch: GlobalRef<'a, T>) -> Self {
        ReadGlobal {
            fetch,
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
        ReadGlobal::<'a, T, F> {
            fetch: world.read_global::<T>().fetch,
            phantom: PhantomData,
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<Globals>(),
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
    fetch: GlobalRefMut<'a, T>,
    phantom: PhantomData<F>,
}

impl<'a, T, F> Deref for WriteGlobal<'a, T, F>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.fetch
    }
}

impl<'a, T, F> DerefMut for WriteGlobal<'a, T, F>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.fetch
    }
}

impl<'a, T, F> From<GlobalRefMut<'a, T>> for WriteGlobal<'a, T, F> {
    fn from(fetch: GlobalRefMut<'a, T>) -> Self {
        WriteGlobal {
            fetch,
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
        WriteGlobal::<'a, T, F> {
            fetch: world.write_global::<T>().fetch,
            phantom: PhantomData,
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<Globals>()]
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
        match world.try_read_global::<T>() {
            None => None,
            Some(fetch) => Some(ReadGlobal::<'a, T, F> {
                fetch: fetch.fetch,
                phantom: PhantomData,
            }),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<Globals>(),
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
        match world.try_write_global::<T>() {
            None => None,
            Some(fetch) => Some(WriteGlobal::<'a, T, F> {
                fetch: fetch.fetch,
                phantom: PhantomData,
            }),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<Globals>()]
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
