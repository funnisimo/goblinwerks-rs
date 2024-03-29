use crate::components::{Component, ReadComp, WriteComp};
use crate::components::{ComponentSet, Components};
use crate::entity::EntityAllocator;
use crate::entity::{Builder, Entities};
use crate::entity::{EntitiesRes, Entity, EntityBuilder};
use crate::event::{AllEvents, Event, Events, ManualEventReader};
use crate::globals::{GlobalMut, GlobalRef, Globals};
use crate::resources::Resources;
use crate::resources::{ResMut, ResRef};
use crate::resources::{Resource, ResourceId};
use crate::schedule::{Schedule, ScheduleLabel, Schedules};
use crate::storage::{MaskedStorage, Storage};
use crate::system::{IntoSystem, System};
use atomize::Atom;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "trace")]
use tracing;
use tracing::Level;

mod fetch;
pub use fetch::*;

// pub use crate::shred::Entry;

pub type WorldId = Atom;

/// Creates an instance of the type this trait is implemented for
/// using data from the supplied [World].
///
/// This can be helpful for complex initialization or context-aware defaults.
pub trait FromWorld {
    /// Creates `Self` using data from the given [World]
    fn from_world(world: &mut World) -> Self;
}

impl<T: Default> FromWorld for T {
    fn from_world(_world: &mut World) -> Self {
        T::default()
    }
}

pub struct World {
    id: WorldId,
    pub(crate) current_tick: AtomicU32,
    pub(crate) last_maintain_tick: u32,
    pub(crate) resources: Resources,
    pub(crate) globals: Globals,
}

impl World {
    /// Creates a new World with a new empty Globals.
    ///
    pub fn empty<I: Into<WorldId>>(id: I) -> Self {
        Self::new(id, Globals::default())
    }

    /// Creates a new World with the given Globals.
    ///
    pub fn new<I: Into<WorldId>>(id: I, globals: Globals) -> Self {
        let mut resources = Resources::empty();
        resources.insert(EntitiesRes::default(), 0);
        resources.insert(Components::default(), 0);
        resources.insert(AllEvents::default(), 0);

        let mut globals = globals;
        globals.ensure_with(|| Schedules::default(), 0);

        World {
            id: id.into(),
            current_tick: AtomicU32::new(1),
            last_maintain_tick: 0,
            resources,
            globals,
        }
    }

    /// Sets the globals on this World.
    /// This is used when adding Worlds into the Ecs.
    pub(crate) fn set_globals(&mut self, globals: Globals) {
        self.globals = globals;
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    // /// Gets `SystemData` `T` from the `World`. This can be used to retrieve
    // /// data just like in [System](crate::System)s.
    // ///
    // /// This will not setup the system data, i.e. resources fetched here must
    // /// exist already.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # use gw_ecs::*;
    // /// # #[derive(Default)] struct Timer; #[derive(Default)] struct AnotherResource;
    // ///
    // /// // NOTE: If you use Specs, use `World::new` instead.
    // /// let mut world = World::empty(0);
    // /// world.insert_resource(Timer);
    // /// world.insert_resource(AnotherResource);
    // /// let system_data: (ReadRes<Timer>, ReadRes<AnotherResource>) = world.fetch();
    // /// ```
    // ///
    // /// # Panics
    // ///
    // /// * Panics if `T` is already borrowed in an incompatible way.
    // pub fn fetch<'a, T>(&'a self) -> T
    // where
    //     T: SystemParam<'a>,
    // {
    //     SystemParam::get_param(self)
    // }

    // /// Sets up system data `T` for fetching afterwards.
    // ///
    // /// Most `SystemData` implementations will insert a sensible default value,
    // /// by implementing [SystemData::setup]. However, it is not guaranteed to
    // /// do that; if there is no sensible default, `setup` might not do anything.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use shred::{Read, World};
    // ///
    // /// #[derive(Default)]
    // /// struct MyCounter(u32);
    // ///
    // /// // NOTE: If you use Specs, use `World::new` instead.
    // /// let mut world = World::empty(0);
    // /// assert!(!world.has_value::<MyCounter>());
    // ///
    // /// // `Read<MyCounter>` requires a `Default` implementation, and uses
    // /// // that to initialize the resource
    // /// world.setup::<Read<MyCounter>>();
    // /// assert!(world.has_value::<MyCounter>());
    // /// ```
    // ///
    // /// Here's another example, showing the case where no resource gets
    // /// initialized:
    // ///
    // /// ```
    // /// use shred::{ReadExpect, World};
    // ///
    // /// struct MyCounter(u32);
    // ///
    // /// // NOTE: If you use Specs, use `World::new` instead.
    // /// let mut world = World::empty(0);
    // ///
    // /// world.setup::<ReadExpect<MyCounter>>();
    // /// ```
    // pub fn setup<'a, T: SystemData<'a>>(&mut self) {
    //     T::setup(self);
    // }

    // /// Executes `f` once, right now and with the specified system data.
    // ///
    // /// This sets up the system data `f` expects, fetches it and then
    // /// executes `f`. This is essentially like a one-time
    // /// [System](crate::System).
    // ///
    // /// This is especially useful if you either need a lot of system data or,
    // /// with Specs, if you want to build an entity and for that you need to
    // /// access resources first - just fetching the resources and building
    // /// the entity would cause a double borrow.
    // ///
    // /// **Calling this method is equivalent to:**
    // ///
    // /// ```
    // /// # use shred::*;
    // /// # struct MySystemData; impl MySystemData { fn do_something(&self) {} }
    // /// # impl<'a> SystemData<'a> for MySystemData {
    // /// #     fn fetch(res: &World) -> Self { MySystemData }
    // /// #     fn reads() -> Vec<ResourceId> { vec![] }
    // /// #     fn writes() -> Vec<ResourceId> { vec![] }
    // /// #     fn setup(res: &mut World) {}
    // /// # }
    // /// # let mut world = World::empty(0);
    // /// {
    // ///     // note the extra scope
    // ///     world.setup::<MySystemData>();
    // ///     let my_data: MySystemData = world.system_data();
    // ///     my_data.do_something();
    // /// }
    // /// ```
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # use shred::*;
    // /// // NOTE: If you use Specs, use `World::new` instead.
    // /// let mut world = World::empty(0);
    // ///
    // /// #[derive(Default)]
    // /// struct MyRes {
    // ///     field: i32,
    // /// }
    // ///
    // /// world.exec(|(mut my_res,): (Write<MyRes>,)| {
    // ///     assert_eq!(my_res.field, 0);
    // ///     my_res.field = 5;
    // /// });
    // ///
    // /// assert_eq!(world.fetch::<MyRes>().field, 5);
    // /// ```
    // pub fn exec<'a, F, R, T>(&'a mut self, f: F) -> R
    // where
    //     F: FnOnce(T) -> R,
    //     T: SystemData<'a>,
    // {
    //     self.setup::<T>();
    //     f(self.fetch())
    // }

    /// Fetches the resource with the specified type `T` or panics if it doesn't
    /// exist.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is being accessed mutably.
    pub fn fetch<T: Fetch>(&self) -> <T as Fetch>::Item<'_> {
        T::fetch(self)
    }

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
        self.globals.contains::<G>()
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_global<G: Resource + Send + Sync + FromWorld>(&mut self) -> bool {
        if self.globals.contains::<G>() {
            return false;
        }

        let g = G::from_world(self);
        self.globals.insert(g, self.current_tick());
        true
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_global_non_send<G: Resource + FromWorld>(&mut self) -> bool {
        if self.globals.contains::<G>() {
            return false;
        }

        let g = G::from_world(self);
        self.globals.insert_non_send(g, self.current_tick());
        true
    }

    /// Makes sure there is a value for the given global.
    /// If not found, inserts a default value.
    pub fn ensure_global_with<G: Resource + Send + Sync, F: FnOnce() -> G>(
        &mut self,
        func: F,
    ) -> bool {
        self.globals.ensure_with(func, self.current_tick())
    }

    /// Inserts a global
    pub fn insert_global<G: Resource + Send + Sync>(&mut self, global: G) {
        self.globals.insert(global, self.current_tick());
    }

    pub fn insert_global_non_send<G: Resource>(&mut self, global: G) {
        self.globals.insert_non_send(global, self.current_tick());
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.globals.remove::<G>(self.current_tick())
    }

    pub fn with_global<R, G: Resource, F: FnOnce(&G) -> R>(&self, f: F) -> R {
        let global = self.read_global::<G>();
        f(&global)
    }

    /// Fetch a global value
    pub fn read_global<G: Resource>(&self) -> GlobalRef<G> {
        self.globals
            .fetch::<G>(self.last_maintain_tick, self.current_tick())
    }

    pub fn read_global_or_insert<G: Resource + Send + Sync + FromWorld>(&mut self) -> GlobalRef<G> {
        self.ensure_global::<G>();
        self.globals
            .fetch::<G>(self.last_maintain_tick, self.current_tick())
    }

    pub fn read_global_or_insert_with<G: Resource + Send + Sync, F: FnOnce() -> G>(
        &mut self,
        f: F,
    ) -> GlobalRef<G> {
        self.ensure_global_with(f);
        self.globals
            .fetch::<G>(self.last_maintain_tick, self.current_tick())
    }

    pub fn with_global_mut<R, G: Resource, F: FnOnce(&mut G) -> R>(&self, f: F) -> R {
        let mut global = self.write_global::<G>();
        f(&mut global)
    }

    /// Fetch a global value as mutable
    pub fn write_global<G: Resource>(&self) -> GlobalMut<G> {
        self.globals
            .fetch_mut::<G>(self.last_maintain_tick, self.current_tick())
    }

    /// Fetch a global value as mutable
    pub fn write_global_or_insert<G: Resource + Send + Sync + FromWorld>(
        &mut self,
    ) -> GlobalMut<G> {
        self.ensure_global::<G>();
        self.globals
            .fetch_mut::<G>(self.last_maintain_tick, self.current_tick())
            .into()
    }

    /// Fetch a global value as mutable
    pub fn write_global_or_insert_with<G: Resource + Send + Sync, F: FnOnce() -> G>(
        &mut self,
        f: F,
    ) -> GlobalMut<G> {
        self.ensure_global_with(f);
        self.globals
            .fetch_mut::<G>(self.last_maintain_tick, self.current_tick())
    }

    /// Try to fetch a global value
    pub fn try_read_global<G: Resource>(&self) -> Option<GlobalRef<G>> {
        self.globals
            .try_fetch::<G>(self.last_maintain_tick, self.current_tick())
            .map(|v| v.into())
    }

    /// Try to fetch a global value mutably
    pub fn try_write_global<G: Resource>(&self) -> Option<GlobalMut<G>> {
        self.globals
            .try_fetch_mut::<G>(self.last_maintain_tick, self.current_tick())
            .map(|v| v.into())
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
        R: Resource + Send + Sync,
    {
        self.resources.insert(r, self.current_tick());
    }

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
    pub fn insert_resource_non_send<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.resources.insert_non_send(r, self.current_tick());
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
        self.resources
            .remove_by_id(ResourceId::new::<R>(), self.current_tick())
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
    pub fn ensure_resource<R: Resource + Send + Sync + FromWorld>(&mut self) -> bool {
        if self.resources.contains::<R>() {
            return false;
        }
        let val = R::from_world(self);
        self.resources.insert(val, self.current_tick());
        true
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_resource_non_send<R: Resource + FromWorld>(&mut self) -> bool {
        if self.resources.contains::<R>() {
            return false;
        }
        let val = R::from_world(self);
        self.resources.insert_non_send(val, self.current_tick());
        true
    }

    /// Makes sure there is a value for the given resource.
    /// If not found, inserts a default value.
    pub fn ensure_resource_with<R: Resource + Send + Sync, F: FnOnce() -> R>(
        &mut self,
        func: F,
    ) -> bool {
        self.resources.ensure_with(func, self.current_tick())
    }

    pub fn with_resource<R: Resource, F: FnOnce(&R) -> T, T>(&self, f: F) -> T {
        let res = self.read_resource::<R>();
        f(&res)
    }

    pub fn read_resource<'world, R: Resource>(&'world self) -> ResRef<'world, R> {
        match self
            .resources
            .get::<R>(self.last_maintain_tick, self.current_tick())
        {
            None => {
                let name = std::any::type_name::<R>();
                panic!(
                    "Failed to find resource [{name}].  Either ensure it is there or use 'try_read_resource'"
                );
            }
            Some(r) => r,
        }
    }

    pub fn read_resource_or_insert<R: Resource + Send + Sync + FromWorld>(&mut self) -> ResRef<R> {
        self.ensure_resource::<R>();
        self.resources
            .get::<R>(self.last_maintain_tick, self.current_tick())
            .unwrap()
    }

    pub fn read_resource_or_insert_with<R: Resource + Send + Sync, F: FnOnce() -> R>(
        &mut self,
        f: F,
    ) -> ResRef<R> {
        self.ensure_resource_with(f);
        self.resources
            .get::<R>(self.last_maintain_tick, self.current_tick())
            .unwrap()
            .into()
    }

    pub fn try_read_resource<R: Resource>(&self) -> Option<ResRef<R>> {
        self.resources
            .get::<R>(self.last_maintain_tick, self.current_tick())
            .map(|v| v.into())
    }

    pub fn with_resource_mut<R: Resource, F: FnOnce(&mut R) -> T, T>(&self, f: F) -> T {
        let mut res = self.write_resource::<R>();
        f(&mut res)
    }

    pub fn write_resource<R: Resource>(&self) -> ResMut<R> {
        match self
            .resources
            .get_mut::<R>(self.last_maintain_tick, self.current_tick())
        {
            None => {
                let name = std::any::type_name::<R>();
                panic!(
                    "Failed to find resource [{name}].  Either ensure it is there or use 'try_write_resource'"
                );
            }
            Some(r) => r,
        }
    }

    pub fn write_resource_or_insert<R: Resource + Send + Sync + FromWorld>(&mut self) -> ResMut<R> {
        self.ensure_resource::<R>();
        self.resources
            .get_mut::<R>(self.last_maintain_tick, self.current_tick())
            .unwrap()
            .into()
    }

    pub fn write_resource_or_insert_with<R: Resource + Send + Sync, F: FnOnce() -> R>(
        &mut self,
        f: F,
    ) -> ResMut<R> {
        self.ensure_resource_with(f);
        self.resources
            .get_mut::<R>(self.last_maintain_tick, self.current_tick())
            .unwrap()
    }

    pub fn try_write_resource<R: Resource>(&self) -> Option<ResMut<R>> {
        self.resources
            .get_mut::<R>(self.last_maintain_tick, self.current_tick())
    }

    // COMPONENTS

    // pub fn add_resource<T: Resource>(&mut self, res: T) {
    //     self.insert(res);
    // }

    pub fn read_component<T: Component>(&self) -> ReadComp<T> {
        let entities = self
            .resources
            .get::<EntitiesRes>(self.last_maintain_tick, self.current_tick())
            .unwrap();
        let data = match self
            .resources
            .get::<MaskedStorage<T>>(self.last_maintain_tick, self.current_tick())
        {
            None => {
                let name = std::any::type_name::<T>();
                panic!(
                    "Failed to find storage for a component [{name}].  Did you forget to register it?"
                );
            }
            Some(data) => data,
        };
        Storage::new(entities, data, self.last_maintain_tick, self.current_tick())
    }

    pub fn with_component<R, C: Component, F: FnOnce(ReadComp<C>) -> R>(&self, f: F) -> R {
        let comp = self.read_component::<C>();
        f(comp)
    }

    pub fn write_component<T: Component>(&self) -> WriteComp<T> {
        let entities = self
            .resources
            .get::<EntitiesRes>(self.last_maintain_tick, self.current_tick())
            .unwrap();
        let data = match self
            .resources
            .get_mut::<MaskedStorage<T>>(self.last_maintain_tick, self.current_tick())
        {
            None => {
                let name = std::any::type_name::<T>();
                panic!(
                    "Failed to find storage for a component [{name}].  Did you forget to register it?"
                );
            }
            Some(data) => data,
        };
        Storage::new(entities, data, self.last_maintain_tick, self.current_tick())
    }

    pub fn with_component_mut<R, C: Component, F: FnOnce(WriteComp<C>) -> R>(&self, f: F) -> R {
        let comp = self.write_component::<C>();
        f(comp)
    }

    // pub fn insert_component<C: Component>(
    //     &mut self,
    //     entity: Entity,
    //     component: C,
    // ) -> InsertResult<C> {
    //     self.write_component::<C>().insert(entity, component)
    // }

    // pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Option<C> {
    //     match self.write_component::<C>().remove(entity) {
    //         None => None,
    //         Some(old) => {
    //             self.send_event(DeleteComp::<C>::new(entity));
    //             Some(old)
    //         }
    //     }
    // }

    // REGISTRY

    pub fn register<T: Component>(&mut self) -> bool
    where
        T::Storage: Default,
    {
        #[cfg(feature = "trace")]
        tracing::event!(
            Level::TRACE,
            "register : {component}",
            component = ResourceId::of::<T>().name()
        );

        self.register_with_storage::<T>(Default::default())
    }

    pub(crate) fn register_with_storage<T>(&mut self, storage: T::Storage) -> bool
    where
        T: Component,
    {
        if self.resources.ensure_with(
            move || MaskedStorage::<T>::new(storage),
            self.current_tick(),
        ) {
            // self.resources
            //     .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
            self.resources
                .get_mut::<Components>(self.last_maintain_tick, self.current_tick())
                .unwrap()
                .register(
                    &*self
                        .resources
                        .get::<MaskedStorage<T>>(self.last_maintain_tick, self.current_tick())
                        .unwrap(),
                );

            // self.register_event::<DeleteComp<T>>();
            return true;
        }
        false
    }

    pub fn register_components_from(&mut self, source: &World) {
        let registry = source.read_resource::<Components>();

        for item in registry.iter(source) {
            item.register(self);
        }
    }

    pub fn components(&self) -> ResRef<Components> {
        self.read_resource::<Components>()
    }

    pub(crate) fn components_mut(&self) -> ResMut<Components> {
        self.write_resource::<Components>()
    }

    // Events

    pub fn register_event<T: Event>(&mut self) -> bool {
        if self.has_resource::<Events<T>>() {
            return false;
        }

        let events = Events::<T>::default();
        self.write_resource::<AllEvents>().register(&events);
        self.insert_resource(events);

        #[cfg(feature = "trace")]
        tracing::event!(
            Level::TRACE,
            "register event : {component}",
            component = ResourceId::of::<T>().name()
        );
        true
    }

    pub fn register_events_from(&mut self, source: &World) {
        let registry = source.read_resource::<AllEvents>();

        for item in registry.iter(source) {
            item.register(self);
        }
    }

    pub fn send_event<T: Event>(&self, event: T) {
        self.write_resource::<Events<T>>().send(event);
    }

    pub fn event_reader<T: Event>(&self) -> ManualEventReader<T> {
        self.read_events::<T>().get_reader_current()
    }

    pub fn read_events<T: Event>(&self) -> ResRef<Events<T>> {
        self.read_resource::<Events<T>>()
    }

    pub fn write_events<T: Event>(&self) -> ResMut<Events<T>> {
        self.write_resource::<Events<T>>()
    }

    // pub fn removed_components(&self) -> ResRef<RemovedComponentEvents> {
    //     self.read_resource::<RemovedComponentEvents>()
    // }

    // pub fn removed_components_mut(&self) -> ResMut<RemovedComponentEvents> {
    //     self.write_resource::<RemovedComponentEvents>()
    // }

    // // Lazy Update

    // pub fn commands(&self) -> ResRef<Commands> {
    //     self.resources
    //         .get::<Commands>(self.last_maintain_tick, self.change_tick())
    //         .unwrap()
    // }

    // ENTITIES

    pub fn entities(&self) -> Entities {
        let entities = self
            .resources
            .get::<EntitiesRes>(self.last_maintain_tick, self.current_tick())
            .unwrap();

        Entities::new(entities)
    }

    fn entities_mut(&self) -> ResMut<EntitiesRes> {
        self.resources
            .get_mut::<EntitiesRes>(self.last_maintain_tick, self.current_tick())
            .unwrap()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entities_mut().create();
        EntityBuilder::new(self, entity)
    }

    pub fn spawn<C: ComponentSet>(&mut self, set: C) -> Entity {
        self.create_entity().spawn(set).id()
    }

    // pub fn create_iter(&mut self) -> CreateIter {
    //     CreateIter(self.entities_mut())
    // }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.delete_entities(&[entity]);
    }

    pub fn delete_entities(&mut self, delete: &[Entity]) {
        self.delete_components(delete);
        let _ = self.entities_mut().alloc.kill(delete);
    }

    pub fn delete_all(&mut self) {
        use crate::join::Join;

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
            let storages = self.read_resource::<Components>();
            for storage in storages.iter_mut(self) {
                storage.try_move_component(entity, self.current_tick(), &mut builder);
            }
            builder.id()
        };
        let _ = self.delete_entity(entity); // Ignore
        new_entity
    }

    pub fn current_tick(&self) -> u32 {
        self.current_tick.load(Ordering::Relaxed)
    }

    // TODO - Remove This
    pub(crate) fn increment_current_tick(&self) -> u32 {
        self.current_tick.fetch_add(1, Ordering::Relaxed)
    }

    // pub fn clear_trackers(&mut self) {
    //     self.maintain();
    // }

    pub fn maintain(&mut self) {
        // println!("WORLD MAINTAIN");
        // All maintain changes are in new tick so that they can be detected by change trackers
        self.last_maintain_tick = self.increment_current_tick();

        {
            let meta = self.components_mut();
            for comp in meta.iter_mut(self) {
                comp.maintain(self.current_tick());
            }
        }

        {
            let meta = self.write_resource::<AllEvents>();
            for ev in meta.iter_mut(self) {
                ev.maintain(self.current_tick());
            }
        }

        self.globals.maintain(self.current_tick());
        self.resources.maintain(self.current_tick());
    }

    pub fn delete_components(&mut self, delete: &[Entity]) {
        // self.resources
        //     .get_or_insert_with(MetaTable::<dyn AnyStorage>::default);
        for storage in self
            .resources
            .get_mut::<Components>(self.last_maintain_tick, self.current_tick())
            .unwrap()
            .iter_mut(self)
        {
            storage.drop(delete, self.current_tick());
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::empty("DEFAULT")
    }
}

// Schedule-related methods
impl World {
    /// Runs the [`Schedule`] associated with the `label` a single time.
    ///
    /// The [`Schedule`] is fetched from the
    pub fn add_schedule(&mut self, schedule: Schedule, label: impl ScheduleLabel) {
        let mut schedules = self.write_global::<Schedules>();
        schedules.insert(label, schedule);
    }

    /// Runs the [`Schedule`] associated with the `label` a single time.
    ///
    /// The [`Schedule`] is fetched from the [`Schedules`] resource of the world by its label,
    /// and system state is cached.
    ///
    /// For simple testing use cases, call [`Schedule::run(&mut world)`](Schedule::run) instead.
    ///
    /// # Panics
    ///
    /// Panics if the requested schedule does not exist, or the [`Schedules`] resource was not added.
    pub fn run_schedule(&mut self, label: impl ScheduleLabel) {
        self.run_schedule_ref(&label);
    }

    /// Runs the [`Schedule`] associated with the `label` a single time.
    ///
    /// Unlike the `run_schedule` method, this method takes the label by reference, which can save a clone.
    ///
    /// The [`Schedule`] is fetched from the [`Schedules`] resource of the world by its label,
    /// and system state is cached.
    ///
    /// For simple testing use cases, call [`Schedule::run(&mut world)`](Schedule::run) instead.
    ///
    /// # Panics
    ///
    /// Panics if the requested schedule does not exist, or the [`Schedules`] resource was not added.
    pub fn run_schedule_ref(&mut self, label: &dyn ScheduleLabel) {
        let (extracted_label, mut schedule) = self
            .write_global::<Schedules>()
            .remove_entry(label)
            .unwrap_or_else(|| panic!("The schedule with the label {label:?} was not found."));

        // TODO: move this span to Schedule::run
        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!("schedule", name = ?extracted_label).entered();
        schedule.run(self);
        self.write_global::<Schedules>()
            .insert(extracted_label, schedule);
    }

    /// Runs the given system
    pub fn exec<R, Marker, S: IntoSystem<(), R, Marker>>(&mut self, sys: S) -> R {
        let mut system = IntoSystem::into_system(sys);
        system.initialize(self);
        let r = system.run((), self);
        system.apply_buffers(self);
        r
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[derive(Default, Debug)]
    struct ResA(u32);

    #[test]
    fn basic_exec() {
        let mut world = World::default();

        world.insert_resource(ResA::default());

        assert_eq!(world.read_resource::<ResA>().0, 0);

        let res = world.exec(|mut a: ResMut<ResA>| {
            a.0 += 1;
            a.0
        });

        assert_eq!(res, 1);
        assert_eq!(world.read_resource::<ResA>().0, 1);
    }
}
