use crate::atomic_refcell::{AtomicRef, AtomicRefMut};
use crate::globals::{GlobalMut, GlobalRef, Globals};
use crate::shred::MetaTable;
use crate::shred::{Resources, SystemData};
use crate::specs::storage::{AnyStorage, MaskedStorage};
use crate::specs::world::EntityAllocator;
use crate::specs::world::{CreateIter, EntitiesRes};
use crate::specs::{
    Commands, Component, Entities, EntitiesMut, Entity, EntityBuilder, ReadComp, Storage, WriteComp,
};
use crate::Builder;
use crate::{ReadRes, Resource, ResourceId, WriteGlobal, WriteRes};
use atomize::Atom;

// pub use crate::shred::Entry;

pub struct World {
    id: Atom,
    pub(crate) resources: Resources,
    pub(crate) globals: Globals,
}

impl World {
    /// Creates a new World with a new empty Globals.
    ///
    pub fn empty<I: Into<Atom>>(id: I) -> Self {
        Self::new(id, Globals::default())
    }

    /// Creates a new World with the given Globals.
    ///
    pub fn new<I: Into<Atom>>(id: I, globals: Globals) -> Self {
        let mut resources = Resources::empty();
        resources.insert(EntitiesRes::default());
        resources.insert(MetaTable::<dyn AnyStorage>::default());
        resources.insert(Commands::default());

        World {
            id: id.into(),
            resources,
            globals,
        }
    }

    /// Sets the globals on this World.
    /// This is used when adding Worlds into the Ecs.
    pub(crate) fn set_globals(&mut self, globals: Globals) {
        self.globals = globals;
    }

    pub fn id(&self) -> Atom {
        self.id
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
    /// # use gw_ecs::*;
    /// # #[derive(Default)] struct Timer; #[derive(Default)] struct AnotherResource;
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty(0);
    /// world.insert_resource(Timer);
    /// world.insert_resource(AnotherResource);
    /// let system_data: (ReadRes<Timer>, ReadRes<AnotherResource>) = world.fetch();
    /// ```
    ///
    /// # Panics
    ///
    /// * Panics if `T` is already borrowed in an incompatible way.
    pub fn fetch<'a, T>(&'a self) -> T
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
    /// let mut world = World::empty(0);
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
    /// let mut world = World::empty(0);
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
    /// # let mut world = World::empty(0);
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
    /// let mut world = World::empty(0);
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
        f(self.fetch())
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

    pub fn with_global<R, G: Resource, F: FnOnce(&G) -> R>(&self, f: F) -> R {
        let global = self.read_global::<G>();
        f(&global)
    }

    /// Fetch a global value
    pub fn read_global<G: Resource>(&self) -> GlobalRef<G> {
        self.globals.fetch::<G>()
    }

    pub fn read_global_or_insert<G: Resource + Default>(&mut self) -> GlobalRef<G> {
        self.ensure_global::<G>();
        self.globals.fetch::<G>()
    }

    pub fn read_global_or_insert_with<G: Resource, F: FnOnce() -> G>(
        &mut self,
        f: F,
    ) -> GlobalRef<G> {
        self.ensure_global_with(f);
        self.globals.fetch::<G>()
    }

    pub fn with_global_mut<R, G: Resource, F: FnOnce(&mut G) -> R>(&self, f: F) -> R {
        let mut global = self.write_global::<G>();
        f(&mut global)
    }

    /// Fetch a global value as mutable
    pub fn write_global<G: Resource>(&self) -> GlobalMut<G> {
        self.globals.fetch_mut::<G>()
    }

    /// Fetch a global value as mutable
    pub fn write_global_or_insert<G: Resource + Default>(&mut self) -> GlobalMut<G> {
        self.ensure_global::<G>();
        self.globals.fetch_mut::<G>().into()
    }

    /// Fetch a global value as mutable
    pub fn write_global_or_insert_with<G: Resource, F: FnOnce() -> G>(
        &mut self,
        f: F,
    ) -> WriteGlobal<G> {
        self.ensure_global_with(f);
        self.globals.fetch_mut::<G>().into()
    }

    /// Try to fetch a global value
    pub fn try_read_global<G: Resource>(&self) -> Option<GlobalRef<G>> {
        self.globals.try_fetch::<G>().map(|v| v.into())
    }

    /// Try to fetch a global value mutably
    pub fn try_write_global<G: Resource>(&self) -> Option<GlobalMut<G>> {
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
    /// let mut world = World::empty(0);
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
    pub fn ensure_resource<R: Resource + Default>(&mut self) {
        self.resources.ensure(R::default);
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_resource_with<R: Resource, F: FnOnce() -> R>(&mut self, func: F) {
        self.resources.ensure(func);
    }

    pub fn with_resource<R: Resource, F: FnOnce(&R) -> T, T>(&self, f: F) -> T {
        let res = self.read_resource::<R>();
        f(&res)
    }

    pub fn read_resource<R: Resource>(&self) -> AtomicRef<R> {
        match self.resources.get::<R>() {
            None => {
                let name = std::any::type_name::<R>();
                panic!(
                    "Failed to find resource [{name}].  Either ensure it is there or use 'try_read_resource'"
                );
            }
            Some(r) => r,
        }
    }

    pub fn read_resource_or_insert<R: Resource + Default>(&mut self) -> AtomicRef<R> {
        self.ensure_resource::<R>();
        self.resources.get::<R>().unwrap()
    }

    pub fn read_resource_or_insert_with<R: Resource, F: FnOnce() -> R>(
        &mut self,
        f: F,
    ) -> AtomicRef<R> {
        self.ensure_resource_with(f);
        self.resources.get::<R>().unwrap().into()
    }

    pub fn try_read_resource<R: Resource>(&self) -> Option<AtomicRef<R>> {
        self.resources.get::<R>().map(|v| v.into())
    }

    pub fn with_resource_mut<R: Resource, F: FnOnce(&mut R) -> T, T>(&self, f: F) -> T {
        let mut res = self.write_resource::<R>();
        f(&mut res)
    }

    pub fn write_resource<R: Resource>(&self) -> AtomicRefMut<R> {
        match self.resources.get_mut::<R>() {
            None => {
                let name = std::any::type_name::<R>();
                panic!(
                    "Failed to find resource [{name}].  Either ensure it is there or use 'try_write_resource'"
                );
            }
            Some(r) => r,
        }
    }

    pub fn write_resource_or_insert<R: Resource + Default>(&mut self) -> AtomicRefMut<R> {
        self.ensure_resource::<R>();
        self.resources.get_mut::<R>().unwrap().into()
    }

    pub fn write_resource_or_insert_with<R: Resource, F: FnOnce() -> R>(
        &mut self,
        f: F,
    ) -> AtomicRefMut<R> {
        self.ensure_resource_with(f);
        self.resources.get_mut::<R>().unwrap()
    }

    pub fn try_write_resource<R: Resource>(&self) -> Option<AtomicRefMut<R>> {
        self.resources.get_mut::<R>()
    }

    // COMPONENTS

    // pub fn add_resource<T: Resource>(&mut self, res: T) {
    //     self.insert(res);
    // }

    pub fn read_component<T: Component>(&self) -> ReadComp<T> {
        let entities = self.resources.get::<EntitiesRes>().unwrap();
        let data = match self.resources.get::<MaskedStorage<T>>() {
            None => {
                let name = std::any::type_name::<T>();
                panic!(
                    "Failed to find storage for a component [{name}].  Did you forget to register it?"
                );
            }
            Some(data) => data,
        };
        Storage::new(entities, data)
    }

    pub fn with_component<R, C: Component, F: FnOnce(ReadComp<C>) -> R>(&self, f: F) -> R {
        let comp = self.read_component::<C>();
        f(comp)
    }

    pub fn write_component<T: Component>(&self) -> WriteComp<T> {
        let entities = self.resources.get::<EntitiesRes>().unwrap();
        let data = match self.resources.get_mut::<MaskedStorage<T>>() {
            None => {
                let name = std::any::type_name::<T>();
                panic!(
                    "Failed to find storage for a component [{name}].  Did you forget to register it?"
                );
            }
            Some(data) => data,
        };
        Storage::new(entities, data)
    }

    pub fn with_component_mut<R, C: Component, F: FnOnce(WriteComp<C>) -> R>(&self, f: F) -> R {
        let comp = self.write_component::<C>();
        f(comp)
    }

    // REGISTRY

    pub fn register<T: Component>(&mut self)
    where
        T::Storage: Default,
    {
        self.register_with_storage::<T>(Default::default());
    }

    pub(crate) fn register_with_storage<T>(&mut self, storage: T::Storage)
    where
        T: Component,
    {
        self.resources
            .get_or_insert_with(move || MaskedStorage::<T>::new(storage));
        // self.resources
        //     .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
        self.resources
            .get_mut::<MetaTable<dyn AnyStorage>>()
            .unwrap()
            .register(&*self.resources.get::<MaskedStorage<T>>().unwrap());
    }

    pub fn register_components(&mut self, source: &World) {
        let registry = source.read_resource::<MetaTable<dyn AnyStorage>>();

        for item in registry.iter(source) {
            item.register(self);
        }
    }

    // Lazy Update

    pub fn commands(&self) -> ReadRes<Commands> {
        self.resources.get::<Commands>().unwrap().into()
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

    pub fn delete_entity(&mut self, entity: Entity) {
        self.delete_entities(&[entity]);
    }

    pub fn delete_entities(&mut self, delete: &[Entity]) {
        self.delete_components(delete);
        let _ = self.entities_mut().alloc.kill(delete);
    }

    pub fn delete_all(&mut self) {
        use crate::specs::join::Join;

        let entities: Vec<_> = self.entities().join().collect();

        self.delete_entities(&entities);
        // .expect(
        //     "Bug: previously collected entities are not valid \
        //      even though access should be exclusive",
        // );
    }

    pub fn is_alive(&self, e: Entity) -> bool {
        assert!(e.gen().is_alive(), "Generation is dead");

        let alloc: &EntityAllocator = &self.entities().alloc;
        alloc.generation(e.id()) == Some(e.gen())
    }

    pub fn move_entity_to(&mut self, entity: Entity, dest: &mut World) -> Entity {
        let new_entity = {
            let mut builder = dest.create_entity();
            let storages = self.read_resource::<MetaTable<dyn AnyStorage>>();
            for storage in storages.iter_mut(self) {
                storage.try_move_component(entity, &mut builder);
            }
            builder.build()
        };
        let _ = self.delete_entity(entity); // Ignore
        new_entity
    }

    pub fn maintain(&mut self) {
        let lazy = self.resources.get_mut::<Commands>().unwrap().clone();
        lazy.maintain(self);

        let deleted = self.entities_mut().maintain();
        if !deleted.is_empty() {
            self.delete_components(&deleted);
        }
    }

    pub fn delete_components(&mut self, delete: &[Entity]) {
        // self.resources
        //     .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
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

// impl Default for World {
//     fn default() -> Self {
//         World::empty("DEFAULT".into())
//     }
// }

#[cfg(test)]
mod tests {
    use atomize::a;

    use super::*;
    use crate::shred::{ReadRes, WriteRes};
    use crate::shred::{RunNow, System, SystemData};

    #[derive(Default)]
    struct Res;

    #[test]
    fn fetch_aspects() {
        assert!(ReadRes::<Res>::reads().contains(&ResourceId::new::<Res>()));
        assert!(ReadRes::<Res>::writes().is_empty());

        let mut world = World::empty(a!(DEFAULT));
        world.insert_resource(Res);
        <ReadRes<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn fetch_mut_aspects() {
        assert!(WriteRes::<Res>::reads().is_empty());
        assert!(WriteRes::<Res>::writes().contains(&ResourceId::new::<Res>()));

        let mut world = World::empty("DEFAULT");
        world.insert_resource(Res);
        <WriteRes<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn system_data() {
        let mut world = World::empty(a!(MAIN));

        world.insert_resource(5u32);
        let x = *world.fetch::<ReadRes<u32>>();
        assert_eq!(x, 5);
    }

    #[test]
    fn setup() {
        let mut world = World::empty(Atom::from("TEST"));

        world.insert_resource(5u32);
        world.setup::<ReadRes<u32>>();
        let x = *world.fetch::<ReadRes<u32>>();
        assert_eq!(x, 5);

        world.remove_resource::<u32>();
        world.setup::<ReadRes<u32>>();
        let x = *world.fetch::<ReadRes<u32>>();
        assert_eq!(x, 0);
    }

    #[test]
    fn exec() {
        #![allow(clippy::float_cmp)]

        let mut world = World::empty("TEST");

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

        world.exec(|(float, boolean): (ReadRes<f32>, ReadRes<bool>)| {
            assert_eq!(*float, 4.3);
            assert!(*boolean);
        });
    }

    #[test]
    #[should_panic]
    fn exec_panic() {
        let mut world = World::empty(Atom::from(123));

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

        let mut world = World::empty(123);
        assert!(world.try_read_resource::<i32>().is_none());

        let mut sys = Sys;
        RunNow::setup(&mut sys, &mut world);

        sys.run_now(&world);

        assert!(world.try_read_resource::<i32>().is_some());
        assert_eq!(*world.read_resource::<i32>(), 33);
    }
}
