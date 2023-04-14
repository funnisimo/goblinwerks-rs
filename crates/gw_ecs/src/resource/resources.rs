use crate::refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use crate::resource::{Resource, ResourceTypeId};
use std::collections::{hash_map::Entry, HashMap};
use std::hash::BuildHasherDefault;
use std::marker::PhantomData;

/////////////////////////////////////////////////////

use std::hash::Hasher;

/// A hasher optimized for hashing component type IDs.
#[derive(Default)]
pub struct ComponentTypeIdHasher(u64);

impl Hasher for ComponentTypeIdHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write_u64(&mut self, seed: u64) {
        // This must only be used to hash one value.
        debug_assert_eq!(self.0, 0);
        self.0 = seed;
    }

    fn write(&mut self, _bytes: &[u8]) {
        // This should not be called, only write_u64.
        unimplemented!()
    }
}

/////////////////////////////////////////////////////

pub struct ResourceCell {
    data: AtomicRefCell<Box<dyn Resource>>,
}

impl ResourceCell {
    fn new(resource: Box<dyn Resource>) -> Self {
        Self {
            data: AtomicRefCell::new(resource),
        }
    }

    fn into_inner(self) -> Box<dyn Resource> {
        self.data.into_inner()
    }

    /// # Safety
    /// Types which are !Sync should only be retrieved on the thread which owns the resource
    /// collection.
    pub fn get<T: Resource>(&self) -> AtomicRef<T> {
        let borrow = self.data.borrow();
        AtomicRef::map(borrow, |inner| inner.downcast_ref::<T>().unwrap())
    }

    /// # Safety
    /// Types which are !Send should only be retrieved on the thread which owns the resource
    /// collection.
    pub fn get_mut<T: Resource>(&self) -> AtomicRefMut<T> {
        let borrow = self.data.borrow_mut(); // panics if this is borrowed already

        AtomicRefMut::map(borrow, |inner| inner.downcast_mut::<T>().unwrap())
    }
}

/// A container for resources which performs runtime borrow checking
/// but _does not_ ensure that `!Sync` resources aren't accessed across threads.
#[derive(Default)]
pub struct UnsafeResources {
    pub(crate) map:
        HashMap<ResourceTypeId, ResourceCell, BuildHasherDefault<ComponentTypeIdHasher>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
    pub(crate) fn contains(&self, type_id: &ResourceTypeId) -> bool {
        self.map.contains_key(type_id)
    }

    /// # Safety
    /// Resources which are `!Sync` or `!Send` must be retrieved or inserted only on the main thread.
    pub(crate) unsafe fn entry(
        &mut self,
        type_id: ResourceTypeId,
    ) -> Entry<ResourceTypeId, ResourceCell> {
        self.map.entry(type_id)
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    pub(crate) unsafe fn insert<T: Resource>(&mut self, resource: T) {
        self.map.insert(
            ResourceTypeId::of::<T>(),
            ResourceCell::new(Box::new(resource)),
        );
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    pub(crate) unsafe fn remove(&mut self, type_id: &ResourceTypeId) -> Option<Box<dyn Resource>> {
        self.map.remove(type_id).map(|cell| cell.into_inner())
    }

    pub(crate) fn get(&self, type_id: &ResourceTypeId) -> Option<&ResourceCell> {
        self.map.get(type_id)
    }

    /// # Safety
    /// Resources which are `!Sync` must be retrieved or inserted only on the main thread.
    pub(crate) unsafe fn merge(&mut self, mut other: Self) {
        // Merge resources, retaining our local ones but moving in any non-existant ones
        for resource in other.map.drain() {
            self.map.entry(resource.0).or_insert(resource.1);
        }
    }
}

/// Resources container. Shared resources stored here can be retrieved in systems.
#[derive(Default)]
pub struct Resources {
    pub(crate) internal: UnsafeResources,
    // marker to make `Resources` !Send and !Sync
    _not_send_sync: PhantomData<*const u8>,
}

impl Resources {
    // pub(crate) fn internal(&self) -> &UnsafeResources {
    //     &self.internal
    // }

    pub fn new() -> Self {
        Default::default()
    }

    /// Creates an accessor to resources which are Send and Sync, which itself can be sent
    /// between threads.
    pub fn sync(&mut self) -> SyncResources {
        SyncResources {
            internal: &self.internal,
        }
    }

    /// Returns `true` if type `T` exists in the store. Otherwise, returns `false`.
    pub fn contains<T: Resource>(&self) -> bool {
        self.internal.contains(&ResourceTypeId::of::<T>())
    }

    /// Inserts the instance of `T` into the store. If the type already exists, it will be silently
    /// overwritten. If you would like to retain the instance of the resource that already exists,
    /// call `remove` first to retrieve it.
    pub fn insert<T: Resource>(&mut self, value: T) {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        unsafe {
            self.internal.insert(value);
        }
    }

    /// Removes the type `T` from this store if it exists.
    ///
    /// # Returns
    /// If the type `T` was stored, the inner instance of `T is returned. Otherwise, `None`.
    pub fn remove<T: Resource>(&mut self) -> Option<T> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        unsafe {
            let resource = self
                .internal
                .remove(&ResourceTypeId::of::<T>())?
                .downcast::<T>()
                .ok()?;
            Some(*resource)
        }
    }

    /// Retrieve an immutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    ///
    /// # Panics
    /// Panics if the resource is already borrowed mutably.
    pub fn get<T: Resource>(&self) -> Option<AtomicRef<T>> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = &ResourceTypeId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get::<T>())
    }

    /// Retrieve a mutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    pub fn get_mut<T: Resource>(&self) -> Option<AtomicRefMut<T>> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = &ResourceTypeId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get_mut::<T>())
    }

    /// Attempts to retrieve an immutable reference to `T` from the store. If it does not exist,
    /// the closure `f` is called to construct the object and it is then inserted into the store.
    pub fn get_or_insert_with<T: Resource, F: FnOnce() -> T>(&mut self, f: F) -> AtomicRef<T> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = ResourceTypeId::of::<T>();
        unsafe {
            self.internal
                .entry(type_id)
                .or_insert_with(|| ResourceCell::new(Box::new((f)())))
                .get()
        }
    }

    /// Attempts to retrieve a mutable reference to `T` from the store. If it does not exist,
    /// the closure `f` is called to construct the object and it is then inserted into the store.
    pub fn get_mut_or_insert_with<T: Resource, F: FnOnce() -> T>(
        &mut self,
        f: F,
    ) -> AtomicRefMut<T> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = ResourceTypeId::of::<T>();
        unsafe {
            self.internal
                .entry(type_id)
                .or_insert_with(|| ResourceCell::new(Box::new((f)())))
                .get_mut()
        }
    }

    /// Attempts to retrieve an immutable reference to `T` from the store. If it does not exist,
    /// the provided value is inserted and then a reference to it is returned.
    pub fn get_or_insert<T: Resource>(&mut self, value: T) -> AtomicRef<T> {
        self.get_or_insert_with(|| value)
    }

    /// Attempts to retrieve a mutable reference to `T` from the store. If it does not exist,
    /// the provided value is inserted and then a reference to it is returned.
    pub fn get_mut_or_insert<T: Resource>(&mut self, value: T) -> AtomicRefMut<T> {
        self.get_mut_or_insert_with(|| value)
    }

    /// Attempts to retrieve an immutable reference to `T` from the store. If it does not exist,
    /// the default constructor for `T` is called.
    ///
    /// `T` must implement `Default` for this method.
    pub fn get_or_default<T: Resource + Default>(&mut self) -> AtomicRef<T> {
        self.get_or_insert_with(T::default)
    }

    /// Attempts to retrieve a mutable reference to `T` from the store. If it does not exist,
    /// the default constructor for `T` is called.
    ///
    /// `T` must implement `Default` for this method.
    pub fn get_mut_or_default<T: Resource + Default>(&mut self) -> AtomicRefMut<T> {
        self.get_mut_or_insert_with(T::default)
    }

    /// Performs merging of two resource storages, which occurs during a world merge.
    /// This merge will retain any already-existant resources in the local world, while moving any
    /// new resources from the source world into this one, consuming the resources.
    pub fn merge(&mut self, other: Resources) {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        unsafe {
            self.internal.merge(other.internal);
        }
    }
}

/// A resource collection which is `Send` and `Sync`, but which only allows access to resources
/// which are `Sync`.
pub struct SyncResources<'a> {
    internal: &'a UnsafeResources,
}

impl<'a> SyncResources<'a> {
    /// Retrieve an immutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    ///
    /// # Panics
    /// Panics if the resource is already borrowed mutably.
    pub fn get<T: Resource + Sync>(&self) -> Option<AtomicRef<T>> {
        let type_id = &ResourceTypeId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get::<T>())
    }

    /// Retrieve a mutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    pub fn get_mut<T: Resource + Send>(&self) -> Option<AtomicRefMut<T>> {
        let type_id = &ResourceTypeId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get_mut::<T>())
    }
}
