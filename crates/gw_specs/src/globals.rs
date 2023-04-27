use crate::atomic_refcell::AtomicBorrowRef;
use crate::shred::cell::{Ref, TrustCell};
use crate::shred::{Fetch, FetchMut, Resource, World};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Globals {
    world: Arc<TrustCell<World>>,
}

impl Globals {
    pub fn new() -> Self {
        Globals {
            world: Arc::new(TrustCell::new(World::empty())),
        } // Using shred world, not specs world (no components)
    }

    pub fn has_value<G: Resource>(&self) -> bool {
        self.world.borrow().has_value::<G>()
    }

    /// Inserts a global
    pub fn insert<G: Resource>(&self, global: G) {
        self.world.borrow_mut().insert(global)
    }

    /// Removes a global
    pub fn remove<G: Resource>(&self) -> Option<G> {
        self.world.borrow_mut().remove()
    }

    pub fn fetch<'b, G: Resource>(&'b self) -> GlobalFetch<'b, G> {
        self.try_fetch().unwrap()
    }

    pub fn try_fetch<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        let (world, borrow) = self.world.borrow().destructure();
        match world.try_fetch::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetch::new(borrow, fetch)),
        }
    }

    pub fn fetch_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        self.try_fetch_mut().unwrap()
    }

    pub fn try_fetch_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        let (world, borrow) = self.world.borrow().destructure();
        match world.try_fetch_mut::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetchMut::new(borrow, fetch)),
        }
    }
}

impl Clone for Globals {
    fn clone(&self) -> Self {
        Globals {
            world: Arc::clone(&self.world),
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
    fn new(borrow: AtomicBorrowRef<'a>, fetch: Fetch<'a, T>) -> Self {
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
    fn new(borrow: AtomicBorrowRef<'a>, fetch: FetchMut<'a, T>) -> Self {
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
