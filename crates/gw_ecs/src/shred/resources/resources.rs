use super::{Resource, ResourceId};
use crate::atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::collections::{hash_map::Entry, HashMap};
use std::marker::PhantomData;

pub struct ResourceCell {
    pub(crate) data: AtomicRefCell<Box<dyn Resource>>,
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
    map: HashMap<ResourceId, ResourceCell>, // , BuildHasherDefault<ComponentTypeIdHasher>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
    fn contains(&self, type_id: &ResourceId) -> bool {
        self.map.contains_key(type_id)
    }

    // /// # Safety
    // /// Resources which are `!Sync` or `!Send` must be retrieved or inserted only on the main thread.
    unsafe fn entry(&mut self, type_id: ResourceId) -> Entry<ResourceId, ResourceCell> {
        self.map.entry(type_id)
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    unsafe fn ensure<T: Resource, F: FnOnce() -> T>(&mut self, f: F) {
        if !self.contains(&ResourceId::of::<T>()) {
            self.insert(f())
        }
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    unsafe fn insert<T: Resource>(&mut self, resource: T) {
        self.map
            .insert(ResourceId::of::<T>(), ResourceCell::new(Box::new(resource)));
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    unsafe fn insert_by_id<T: Resource>(&mut self, id: ResourceId, resource: T) {
        self.map.insert(id, ResourceCell::new(Box::new(resource)));
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    unsafe fn remove(&mut self, type_id: &ResourceId) -> Option<Box<dyn Resource>> {
        self.map.remove(type_id).map(|cell| cell.into_inner())
    }

    fn get(&self, type_id: &ResourceId) -> Option<&ResourceCell> {
        self.map.get(type_id)
    }

    /// # Safety
    /// Resources which are `!Sync` must be retrieved or inserted only on the main thread.
    unsafe fn merge(&mut self, mut other: Self) {
        // Merge resources, retaining our local ones but moving in any non-existant ones
        for resource in other.map.drain() {
            self.map.entry(resource.0).or_insert(resource.1);
        }
    }
}

/// Resources container. Shared resources stored here can be retrieved in systems.
#[derive(Default)]
pub struct Resources {
    internal: UnsafeResources,
    // marker to make `Resources` !Send and !Sync
    // _not_send_sync: PhantomData<*const u8>,
}

impl Resources {
    pub fn empty() -> Self {
        Default::default()
    }

    pub(crate) fn internal(&self) -> &UnsafeResources {
        &self.internal
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
        self.internal.contains(&ResourceId::of::<T>())
    }

    /// # Safety
    /// Resources which are `!Send` must be retrieved or inserted only on the main thread.
    pub fn ensure<T: Resource, F: FnOnce() -> T>(&mut self, f: F) {
        self.get_or_insert_with(f);
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

    /// Inserts the instance of `T` into the store. If the type already exists, it will be silently
    /// overwritten. If you would like to retain the instance of the resource that already exists,
    /// call `remove` first to retrieve it.
    pub(crate) fn insert_by_id<T: Resource>(&mut self, id: ResourceId, value: T) {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        unsafe {
            self.internal.insert_by_id(id, value);
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
                .remove(&ResourceId::of::<T>())?
                .downcast::<T>()
                .ok()?;
            Some(*resource)
        }
    }

    /// Removes the type `T` from this store if it exists.
    ///
    /// # Returns
    /// If the type `T` was stored, the inner instance of `T is returned. Otherwise, `None`.
    pub(crate) fn remove_by_id<T: Resource>(&mut self, id: ResourceId) -> Option<T> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        unsafe {
            let resource = self.internal.remove(&id)?.downcast::<T>().ok()?;
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
        let type_id = &ResourceId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get::<T>())
    }

    /// Retrieve an immutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    ///
    /// # Panics
    /// Panics if the resource is already borrowed mutably.
    pub(crate) fn get_by_id<T: Resource>(&self, id: ResourceId) -> Option<AtomicRef<T>> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        self.internal.get(&id).map(|x| x.get::<T>())
    }

    /// Retrieve an immutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    ///
    /// # Panics
    /// Panics if the resource is already borrowed mutably.
    pub(crate) fn get_internal(&self, id: ResourceId) -> Option<&ResourceCell> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        self.internal.get(&id)
    }

    /// Retrieve a mutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    pub fn get_mut<T: Resource>(&self) -> Option<AtomicRefMut<T>> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = &ResourceId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get_mut::<T>())
    }

    /// Retrieve a mutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    pub fn get_mut_by_id<T: Resource>(&self, id: ResourceId) -> Option<AtomicRefMut<T>> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        self.internal.get(&id).map(|x| x.get_mut::<T>())
    }

    /// Attempts to retrieve an immutable reference to `T` from the store. If it does not exist,
    /// the closure `f` is called to construct the object and it is then inserted into the store.
    pub fn get_or_insert_with<T: Resource, F: FnOnce() -> T>(&mut self, f: F) -> AtomicRef<T> {
        // safety:
        // this type is !Send and !Sync, and so can only be accessed from the thread which
        // owns the resources collection
        let type_id = ResourceId::of::<T>();
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
        let type_id = ResourceId::of::<T>();
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
        let type_id = &ResourceId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get::<T>())
    }

    /// Retrieve a mutable reference to  `T` from the store if it exists. Otherwise, return `None`.
    pub fn get_mut<T: Resource + Send>(&self) -> Option<AtomicRefMut<T>> {
        let type_id = &ResourceId::of::<T>();
        self.internal.get(&type_id).map(|x| x.get_mut::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::shred::{RunNow, System};

    #[derive(Default)]
    struct Res;

    // #[test]
    // fn fetch_aspects() {
    //     assert_eq!(Read::<Res>::reads(), vec![ResourceId::new::<Res>()]);
    //     assert_eq!(Read::<Res>::writes(), vec![]);

    //     let mut world = World::empty();
    //     world.insert(Res);
    //     <Read<Res> as SystemData>::fetch(&world);
    // }

    // #[test]
    // fn fetch_mut_aspects() {
    //     assert_eq!(Write::<Res>::reads(), vec![]);
    //     assert_eq!(Write::<Res>::writes(), vec![ResourceId::new::<Res>()]);

    //     let mut world = World::empty();
    //     world.insert(Res);
    //     <Write<Res> as SystemData>::fetch(&world);
    // }

    #[test]
    fn fetch_by_id() {
        #![allow(clippy::map_clone)] // False positive

        let mut resources = Resources::empty();

        resources.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(1), 5);
        resources.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(2), 15);
        resources.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(3), 45);

        assert_eq!(
            resources
                .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(2))
                .map(|x| *x),
            Some(15)
        );
        assert_eq!(
            resources
                .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(1))
                .map(|x| *x),
            Some(5)
        );
        assert_eq!(
            resources
                .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(3))
                .map(|x| *x),
            Some(45)
        );
    }

    // #[test]
    // fn system_data() {
    //     let mut resources = World::empty();

    //     resources.insert(5u32);
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 5);
    // }

    // #[test]
    // fn setup() {
    //     let mut resources = World::empty();

    //     resources.insert(5u32);
    //     resources.setup::<Read<u32>>();
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 5);

    //     resources.remove::<u32>();
    //     resources.setup::<Read<u32>>();
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 0);
    // }

    // #[test]
    // fn exec() {
    //     #![allow(clippy::float_cmp)]

    //     let mut resources = World::empty();

    //     resources.exec(|(float, boolean): (Read<f32>, Read<bool>)| {
    //         assert_eq!(*float, 0.0);
    //         assert!(!*boolean);
    //     });

    //     resources.exec(|(mut float, mut boolean): (Write<f32>, Write<bool>)| {
    //         *float = 4.3;
    //         *boolean = true;
    //     });

    //     resources.exec(|(float, boolean): (Read<f32>, ReadExpect<bool>)| {
    //         assert_eq!(*float, 4.3);
    //         assert!(*boolean);
    //     });
    // }

    // #[test]
    // #[should_panic]
    // fn exec_panic() {
    //     let mut resources = World::empty();

    //     resources.exec(|(_float, _boolean): (Write<f32>, Write<bool>)| {
    //         panic!();
    //     });
    // }

    #[test]
    #[should_panic]
    fn invalid_fetch_by_id0() {
        let mut resources = Resources::empty();

        resources.insert(5i32);

        resources.get_by_id::<u32>(ResourceId::new_with_dynamic_id::<i32>(111));
    }

    #[test]
    #[should_panic]
    fn invalid_fetch_by_id1() {
        let mut resources = Resources::empty();

        resources.insert(5i32);

        resources.get_by_id::<i32>(ResourceId::new_with_dynamic_id::<u32>(111));
    }

    #[test]
    fn add() {
        struct Foo;

        let mut resources = Resources::empty();
        resources.insert(Res);

        assert!(resources.contains::<Res>());
        assert!(!resources.contains::<Foo>());
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "already immutably borrowed")]
    fn read_write_fails() {
        let mut resources = Resources::empty();
        resources.insert(Res);

        let read: AtomicRef<Res> = resources.get::<Res>().unwrap();
        let write: AtomicRefMut<Res> = resources.get_mut::<Res>().unwrap();
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn write_read_fails() {
        let mut resources = Resources::empty();
        resources.insert(Res);

        let write: AtomicRefMut<Res> = resources.get_mut::<Res>().unwrap();
        let read: AtomicRef<Res> = resources.get::<Res>().unwrap();
    }

    #[test]
    fn remove_insert() {
        let mut resources = Resources::empty();

        resources.insert(Res);

        assert!(resources.contains::<Res>());

        // println!("{:#?}", resources.hashmap.keys().collect::<Vec<_>>());

        resources.remove::<Res>().unwrap();

        assert!(!resources.contains::<Res>());

        resources.insert(Res);

        assert!(resources.contains::<Res>());
    }

    // #[test]
    // fn default_works() {
    //     struct Sys;

    //     impl<'a> System<'a> for Sys {
    //         type SystemData = Write<'a, i32>;

    //         fn run(&mut self, mut data: Self::SystemData) {
    //             assert_eq!(*data, 0);

    //             *data = 33;
    //         }
    //     }

    //     let mut resources = Resources::empty();
    //     assert!(resources.try_fetch::<i32>().is_none());

    //     let mut sys = Sys;
    //     RunNow::setup(&mut sys, &mut resources);

    //     sys.run_now(&resources);

    //     assert!(resources.try_fetch::<i32>().is_some());
    //     assert_eq!(*resources.fetch::<i32>(), 33);
    // }

    #[test]
    fn simple_read_write_test() {
        struct TestOne {
            value: String,
        }

        struct TestTwo {
            value: String,
        }

        struct NotSync {
            ptr: *const u8,
        }

        let mut resources = Resources::default();
        resources.insert(TestOne {
            value: "one".to_string(),
        });

        resources.insert(TestTwo {
            value: "two".to_string(),
        });

        resources.insert(NotSync {
            ptr: std::ptr::null(),
        });

        assert_eq!(resources.get::<TestOne>().unwrap().value, "one");
        assert_eq!(resources.get::<TestTwo>().unwrap().value, "two");
        assert_eq!(resources.get::<NotSync>().unwrap().ptr, std::ptr::null());

        // test re-ownership
        let owned = resources.remove::<TestTwo>();
        assert_eq!(owned.unwrap().value, "two");
    }
}
