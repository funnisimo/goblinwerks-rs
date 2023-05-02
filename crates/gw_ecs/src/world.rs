use std::marker::PhantomData;

use crate::globals::Globals;
use crate::shred::{PanicIfMissing, Resources, SystemData};
use crate::specs::error::WrongGeneration;
use crate::specs::storage::{AnyStorage, MaskedStorage};
use crate::specs::world::EntityAllocator;
use crate::specs::world::{CreateIter, EntitiesRes};
use crate::specs::{
    Component, Entities, EntitiesMut, Entity, EntityBuilder, LazyUpdate, ReadComp, Storage,
    WriteComp,
};
use crate::utils::MetaTable;
use crate::{ReadGlobal, ReadRes, Resource, ResourceId, WriteGlobal, WriteRes};

// pub use crate::shred::Entry;

pub struct World {
    pub(crate) resources: Resources,
    pub(crate) globals: Globals,
}

impl World {
    /// Creates a new World with a new empty Globals.
    ///
    pub fn empty() -> Self {
        Self::new(Globals::default())
    }

    /// Creates a new World with the given Globals.
    ///
    pub fn new(globals: Globals) -> Self {
        let mut resources = Resources::empty();
        resources.insert(EntitiesRes::default());
        resources.insert(MetaTable::<dyn AnyStorage>::default());
        resources.insert(LazyUpdate::default());

        World { resources, globals }
    }

    /// Sets the globals on this World.
    /// This is used when adding Worlds into the Ecs.
    pub(crate) fn set_globals(&mut self, globals: Globals) {
        self.globals = globals;
    }

    /// Gets `SystemData` `T` from the `World`. This can be used to retrieve
    /// data just like in [System](crate::System)s.
    ///
    /// This will not setup the system data, i.e. resources fetched here must
    /// exist already.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shred::*;
    /// # #[derive(Default)] struct Timer; #[derive(Default)] struct AnotherResource;
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    /// world.insert(Timer);
    /// world.insert(AnotherResource);
    /// let system_data: (Read<Timer>, Read<AnotherResource>) = world.system_data();
    /// ```
    ///
    /// # Panics
    ///
    /// * Panics if `T` is already borrowed in an incompatible way.
    pub fn system_data<'a, T>(&'a self) -> T
    where
        T: SystemData<'a>,
    {
        SystemData::fetch(self)
    }

    /// Sets up system data `T` for fetching afterwards.
    ///
    /// Most `SystemData` implementations will insert a sensible default value,
    /// by implementing [SystemData::setup]. However, it is not guaranteed to
    /// do that; if there is no sensible default, `setup` might not do anything.
    ///
    /// # Examples
    ///
    /// ```
    /// use shred::{Read, World};
    ///
    /// #[derive(Default)]
    /// struct MyCounter(u32);
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    /// assert!(!world.has_value::<MyCounter>());
    ///
    /// // `Read<MyCounter>` requires a `Default` implementation, and uses
    /// // that to initialize the resource
    /// world.setup::<Read<MyCounter>>();
    /// assert!(world.has_value::<MyCounter>());
    /// ```
    ///
    /// Here's another example, showing the case where no resource gets
    /// initialized:
    ///
    /// ```
    /// use shred::{ReadExpect, World};
    ///
    /// struct MyCounter(u32);
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    ///
    /// world.setup::<ReadExpect<MyCounter>>();
    /// ```
    pub fn setup<'a, T: SystemData<'a>>(&mut self) {
        T::setup(self);
    }

    /// Executes `f` once, right now and with the specified system data.
    ///
    /// This sets up the system data `f` expects, fetches it and then
    /// executes `f`. This is essentially like a one-time
    /// [System](crate::System).
    ///
    /// This is especially useful if you either need a lot of system data or,
    /// with Specs, if you want to build an entity and for that you need to
    /// access resources first - just fetching the resources and building
    /// the entity would cause a double borrow.
    ///
    /// **Calling this method is equivalent to:**
    ///
    /// ```
    /// # use shred::*;
    /// # struct MySystemData; impl MySystemData { fn do_something(&self) {} }
    /// # impl<'a> SystemData<'a> for MySystemData {
    /// #     fn fetch(res: &World) -> Self { MySystemData }
    /// #     fn reads() -> Vec<ResourceId> { vec![] }
    /// #     fn writes() -> Vec<ResourceId> { vec![] }
    /// #     fn setup(res: &mut World) {}
    /// # }
    /// # let mut world = World::empty();
    /// {
    ///     // note the extra scope
    ///     world.setup::<MySystemData>();
    ///     let my_data: MySystemData = world.system_data();
    ///     my_data.do_something();
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use shred::*;
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    ///
    /// #[derive(Default)]
    /// struct MyRes {
    ///     field: i32,
    /// }
    ///
    /// world.exec(|(mut my_res,): (Write<MyRes>,)| {
    ///     assert_eq!(my_res.field, 0);
    ///     my_res.field = 5;
    /// });
    ///
    /// assert_eq!(world.fetch::<MyRes>().field, 5);
    /// ```
    pub fn exec<'a, F, R, T>(&'a mut self, f: F) -> R
    where
        F: FnOnce(T) -> R,
        T: SystemData<'a>,
    {
        self.setup::<T>();
        f(self.system_data())
    }

    // /// Fetches the resource with the specified type `T` or panics if it doesn't
    // /// exist.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the resource doesn't exist.
    // /// Panics if the resource is being accessed mutably.
    // pub fn fetch<T>(&self) -> Fetch<T>
    // where
    //     T: Resource,
    // {
    //     self.try_fetch().unwrap_or_else(|| {
    //         if self.resources.is_empty() {
    //             eprintln!(
    //                 "Note: Could not find a resource (see the following panic);\
    //                  the `World` is completely empty. Did you accidentally create a fresh `World`?"
    //             )
    //         }

    //         fetch_panic!()
    //     })
    // }

    // /// Like `fetch`, but returns an `Option` instead of inserting a default
    // /// value in case the resource does not exist.
    // pub fn try_fetch<T>(&self) -> Option<Fetch<T>>
    // where
    //     T: Resource,
    // {
    //     self.resources.try_fetch::<T>()
    // }

    // /// Like `try_fetch`, but fetches the resource by its `ResourceId` which
    // /// allows using a dynamic ID.
    // ///
    // /// This is usually not what you need; please read the type-level
    // /// documentation of `ResourceId`.
    // ///
    // /// # Panics
    // ///
    // /// This method panics if `id` refers to a different type ID than `T`.
    // pub fn try_fetch_by_id<T>(&self, id: ResourceId) -> Option<Fetch<T>>
    // where
    //     T: Resource,
    // {
    //     self.resources.try_fetch_by_id(id)
    // }

    // /// Fetches the resource with the specified type `T` mutably.
    // ///
    // /// Please see `fetch` for details.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the resource doesn't exist.
    // /// Panics if the resource is already being accessed.
    // pub fn fetch_mut<T>(&self) -> FetchMut<T>
    // where
    //     T: Resource,
    // {
    //     self.resources.try_fetch_mut().unwrap_or_else(|| fetch_panic!())
    // }

    // /// Like `fetch_mut`, but returns an `Option` instead of inserting a default
    // /// value in case the resource does not exist.
    // pub fn try_fetch_mut<T>(&self) -> Option<FetchMut<T>>
    // where
    //     T: Resource,
    // {
    //     self.resources.try_fetch_mut()
    // }

    // /// Like `try_fetch_mut`, but fetches the resource by its `ResourceId` which
    // /// allows using a dynamic ID.
    // ///
    // /// This is usually not what you need; please read the type-level
    // /// documentation of `ResourceId`.
    // ///
    // /// # Panics
    // ///
    // /// This method panics if `id` refers to a different type ID than `T`.
    // pub fn try_fetch_mut_by_id<T>(&self, id: ResourceId) -> Option<FetchMut<T>>
    // where
    //     T: Resource,
    // {
    //     self.resources.try_fetch_mut_by_id(id)
    // }

    // /// Internal function for inserting resources, should only be used if you
    // /// know what you're doing.
    // ///
    // /// This is useful for inserting resources with a custom `ResourceId`.
    // ///
    // /// # Panics
    // ///
    // /// This method panics if `id` refers to a different type ID than `R`.
    // pub fn insert_by_id<R>(&mut self, id: ResourceId, r: R)
    // where
    //     R: Resource,
    // {
    //     self.resources.insert_by_id(id, r)
    // }

    // /// Internal function for removing resources, should only be used if you
    // /// know what you're doing.
    // ///
    // /// This is useful for removing resources with a custom `ResourceId`.
    // ///
    // /// # Panics
    // ///
    // /// This method panics if `id` refers to a different type ID than `R`.
    // pub fn remove_by_id<R>(&mut self, id: ResourceId) -> Option<R>
    // where
    //     R: Resource,
    // {
    //     self.resources.remove_by_id(id)
    // }

    // /// Internal function for fetching resources, should only be used if you
    // /// know what you're doing.
    // pub(crate) fn try_fetch_internal(
    //     &self,
    //     id: ResourceId,
    // ) -> Option<&TrustCell<Box<dyn Resource>>> {
    //     self.resources.try_fetch_internal(id)
    // }

    // /// Retrieves a resource without fetching, which is cheaper, but only
    // /// available with `&mut self`.
    // pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
    //     self.get_mut_raw(ResourceId::new::<T>())
    //         .map(|res| unsafe { res.downcast_mut_unchecked() })
    // }

    // /// Retrieves a resource without fetching, which is cheaper, but only
    // /// available with `&mut self`.
    // pub fn get_mut_raw(&mut self, id: ResourceId) -> Option<&mut dyn Resource> {
    //     self.resources
    //         .get_mut(&id)
    //         .map(TrustCell::get_mut)
    //         .map(Box::as_mut)
    // }

    // GLOBALS

    pub fn has_global<G: Resource>(&self) -> bool {
        self.globals.has_value::<G>()
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_global<G: Resource + Default>(&mut self) {
        self.globals.ensure_with(G::default);
    }

    /// Makes sure there is a value for the given global.
    /// If not found, inserts a default value.
    pub fn ensure_global_with<G: Resource, F: FnOnce() -> G>(&mut self, func: F) {
        self.globals.ensure_with(func);
    }

    /// Inserts a global
    pub fn insert_global<G: Resource>(&mut self, global: G) {
        self.globals.insert(global);
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.globals.remove::<G>()
    }

    /// Fetch a global value
    pub fn read_global<G: Resource>(&self) -> ReadGlobal<G, PanicIfMissing> {
        self.globals.fetch::<G>().into()
    }

    /// Fetch a global value as mutable
    pub fn write_global<G: Resource>(&self) -> WriteGlobal<G, PanicIfMissing> {
        self.globals.fetch_mut::<G>().into()
    }

    /// Try to fetch a global value
    pub fn try_read_global<G: Resource>(&self) -> Option<ReadGlobal<G, ()>> {
        self.globals.try_fetch::<G>().map(|v| v.into())
    }

    /// Try to fetch a global value mutably
    pub fn try_write_global<G: Resource>(&self) -> Option<WriteGlobal<G, ()>> {
        self.globals.try_fetch_mut::<G>().map(|v| v.into())
    }

    //
    // RESOURCES
    //

    /// Inserts a resource into this container. If the resource existed before,
    /// it will be overwritten.
    ///
    /// # Examples
    ///
    /// Every type satisfying `Any + Send + Sync` automatically
    /// implements `Resource`, thus can be added:
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// struct MyRes(i32);
    /// ```
    ///
    /// When you have a resource, simply insert it like this:
    ///
    /// ```rust
    /// # struct MyRes(i32);
    /// use shred::World;
    ///
    /// let mut world = World::empty();
    /// world.insert(MyRes(5));
    /// ```
    pub fn insert_resource<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.resources.insert_by_id(ResourceId::new::<R>(), r);
    }

    /// Removes a resource of type `R` from the `World` and returns its
    /// ownership to the caller. In case there is no such resource in this
    /// `World`, `None` will be returned.
    ///
    /// Use this method with caution; other functions and systems might assume
    /// this resource still exists. Thus, only use this if you're sure no
    /// system will try to access this resource after you removed it (or else
    /// you will get a panic).
    pub fn remove_resource<R>(&mut self) -> Option<R>
    where
        R: Resource,
    {
        self.resources.remove_by_id(ResourceId::new::<R>())
    }

    /// Returns true if the specified resource type `R` exists in `self`.
    pub fn has_resource<R>(&self) -> bool
    where
        R: Resource,
    {
        self.resources.contains::<R>()
    }

    // /// Returns true if the specified resource type exists in `self`.
    // pub fn has_value_raw(&self, id: ResourceId) -> bool {
    //     self.resources.has_value_raw(id)
    // }

    // /// Returns an entry for the resource with type `R`.
    // pub fn entry<R>(&mut self) -> Entry<R>
    // where
    //     R: Resource,
    // {
    //     self.resources.entry::<R>()
    // }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_resource<G: Resource + Default>(&mut self) {
        self.resources.ensure(G::default);
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_resource_with<G: Resource, F: FnOnce() -> G>(&mut self, func: F) {
        self.resources.ensure(func);
    }

    pub fn read_resource<T: Resource>(&self) -> ReadRes<T, PanicIfMissing> {
        self.resources.get::<T>().unwrap().into()
    }

    pub fn try_read_resource<T: Resource>(&self) -> Option<ReadRes<T, ()>> {
        self.resources.get::<T>().map(|v| v.into())
    }

    pub fn write_resource<T: Resource>(&self) -> WriteRes<T, PanicIfMissing> {
        self.resources.get_mut::<T>().unwrap().into()
    }

    pub fn try_write_resource<T: Resource>(&self) -> Option<WriteRes<T, ()>> {
        self.resources.get_mut::<T>().map(|v| v.into())
    }

    // COMPONENTS

    pub fn register<T: Component>(&mut self)
    where
        T::Storage: Default,
    {
        self.register_with_storage::<_, T>(Default::default);
    }

    pub(crate) fn register_with_storage<F, T>(&mut self, storage: F)
    where
        F: FnOnce() -> T::Storage,
        T: Component,
    {
        self.resources
            .get_or_insert_with(move || MaskedStorage::<T>::new(storage()));
        self.resources
            .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
        self.resources
            .get_mut::<MetaTable<dyn AnyStorage>>()
            .unwrap()
            .register(&*self.resources.get::<MaskedStorage<T>>().unwrap());
    }

    // pub fn add_resource<T: Resource>(&mut self, res: T) {
    //     self.insert(res);
    // }

    pub fn read_component<T: Component>(&self) -> ReadComp<T> {
        let entities = self.resources.get::<EntitiesRes>().unwrap();
        let data = self.resources.get::<MaskedStorage<T>>().unwrap();
        Storage::new(entities, data)
    }

    pub fn write_component<T: Component>(&self) -> WriteComp<T> {
        let entities = self.resources.get::<EntitiesRes>().unwrap();
        let data = self.resources.get_mut::<MaskedStorage<T>>().unwrap();
        Storage::new(entities, data)
    }

    // Lazy Update

    pub fn lazy_update(&self) -> ReadRes<LazyUpdate> {
        self.resources.get::<LazyUpdate>().unwrap().into()
    }

    // ENTITIES

    pub fn entities(&self) -> Entities {
        self.resources.get::<EntitiesRes>().unwrap().into()
    }

    pub(crate) fn entities_mut(&self) -> EntitiesMut {
        self.resources.get_mut::<EntitiesRes>().unwrap().into()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        self.create_entity_unchecked()
    }

    pub(crate) fn create_entity_unchecked(&self) -> EntityBuilder {
        let entity = self.entities_mut().alloc.allocate();

        EntityBuilder {
            entity,
            world: self,
            built: false,
        }
    }

    pub fn create_iter(&mut self) -> CreateIter {
        CreateIter(self.entities_mut())
    }

    pub fn delete_entity(&mut self, entity: Entity) -> Result<(), WrongGeneration> {
        self.delete_entities(&[entity])
    }

    pub fn delete_entities(&mut self, delete: &[Entity]) -> Result<(), WrongGeneration> {
        self.delete_components(delete);

        self.entities_mut().alloc.kill(delete)
    }

    pub fn delete_all(&mut self) {
        use crate::specs::join::Join;

        let entities: Vec<_> = self.entities().join().collect();

        self.delete_entities(&entities).expect(
            "Bug: previously collected entities are not valid \
             even though access should be exclusive",
        );
    }

    pub fn is_alive(&self, e: Entity) -> bool {
        assert!(e.gen().is_alive(), "Generation is dead");

        let alloc: &EntityAllocator = &self.entities().alloc;
        alloc.generation(e.id()) == Some(e.gen())
    }

    pub fn maintain(&mut self) {
        let deleted = self.entities_mut().alloc.merge();
        if !deleted.is_empty() {
            self.delete_components(&deleted);
        }

        let lazy = self.resources.get_mut::<LazyUpdate>().unwrap().clone();
        lazy.maintain(self);
    }

    pub fn delete_components(&mut self, delete: &[Entity]) {
        self.resources
            .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
        for storage in self
            .resources
            .get_mut::<MetaTable<dyn AnyStorage>>()
            .unwrap()
            .iter_mut(self)
        {
            storage.drop(delete);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shred::{ReadRes, ReadResExpect, WriteRes};
    use crate::shred::{RunNow, System, SystemData};

    #[derive(Default)]
    struct Res;

    #[test]
    fn fetch_aspects() {
        assert_eq!(ReadRes::<Res>::reads(), vec![ResourceId::new::<Res>()]);
        assert_eq!(ReadRes::<Res>::writes(), vec![]);

        let mut world = World::empty();
        world.insert_resource(Res);
        <ReadRes<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn fetch_mut_aspects() {
        assert_eq!(WriteRes::<Res>::reads(), vec![]);
        assert_eq!(WriteRes::<Res>::writes(), vec![ResourceId::new::<Res>()]);

        let mut world = World::empty();
        world.insert_resource(Res);
        <WriteRes<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn system_data() {
        let mut world = World::empty();

        world.insert_resource(5u32);
        let x = *world.system_data::<ReadRes<u32>>();
        assert_eq!(x, 5);
    }

    #[test]
    fn setup() {
        let mut world = World::empty();

        world.insert_resource(5u32);
        world.setup::<ReadRes<u32>>();
        let x = *world.system_data::<ReadRes<u32>>();
        assert_eq!(x, 5);

        world.remove_resource::<u32>();
        world.setup::<ReadRes<u32>>();
        let x = *world.system_data::<ReadRes<u32>>();
        assert_eq!(x, 0);
    }

    #[test]
    fn exec() {
        #![allow(clippy::float_cmp)]

        let mut world = World::empty();

        world.exec(|(float, boolean): (ReadRes<f32>, ReadRes<bool>)| {
            assert_eq!(*float, 0.0);
            assert!(!*boolean);
        });

        world.exec(
            |(mut float, mut boolean): (WriteRes<f32>, WriteRes<bool>)| {
                *float = 4.3;
                *boolean = true;
            },
        );

        world.exec(|(float, boolean): (ReadRes<f32>, ReadResExpect<bool>)| {
            assert_eq!(*float, 4.3);
            assert!(*boolean);
        });
    }

    #[test]
    #[should_panic]
    fn exec_panic() {
        let mut world = World::empty();

        world.exec(|(_float, _boolean): (WriteRes<f32>, WriteRes<bool>)| {
            panic!();
        });
    }

    #[test]
    fn default_works() {
        struct Sys;

        impl<'a> System<'a> for Sys {
            type SystemData = WriteRes<'a, i32>;

            fn run(&mut self, mut data: Self::SystemData) {
                assert_eq!(*data, 0);

                *data = 33;
            }
        }

        let mut world = World::empty();
        assert!(world.try_read_resource::<i32>().is_none());

        let mut sys = Sys;
        RunNow::setup(&mut sys, &mut world);

        sys.run_now(&world);

        assert!(world.try_read_resource::<i32>().is_some());
        assert_eq!(*world.read_resource::<i32>(), 33);
    }
}
