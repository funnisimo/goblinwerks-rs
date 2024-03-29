//! Types for declaring and storing [`Component`]s.

use crate::change_detection::MAX_CHANGE_AGE;
use bevy_ptr::UnsafeCellDeref;
pub use gw_ecs_macros::Component;
use std::cell::UnsafeCell;

// pub struct TableStorage;
// pub struct SparseStorage;

// pub trait ComponentStorage: sealed::Sealed {
//     // because the trait is sealed, those items are private API.
//     const STORAGE_TYPE: StorageType;
// }

// impl ComponentStorage for TableStorage {
//     const STORAGE_TYPE: StorageType = StorageType::Table;
// }
// impl ComponentStorage for SparseStorage {
//     const STORAGE_TYPE: StorageType = StorageType::SparseSet;
// }

// mod sealed {
//     pub trait Sealed {}
//     impl Sealed for super::TableStorage {}
//     impl Sealed for super::SparseStorage {}
// }

// /// The storage used for a specific component type.
// ///
// /// # Examples
// /// The [`StorageType`] for a component is configured via the derive attribute
// ///
// /// ```
// /// # use gw_ecs::{prelude::*, component::*};
// /// #[derive(Component)]
// /// #[component(storage = "SparseSet")]
// /// struct A;
// /// ```
// #[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
// pub enum StorageType {
//     /// Provides fast and cache-friendly iteration, but slower addition and removal of components.
//     /// This is the default storage type.
//     #[default]
//     Table,
//     /// Provides fast addition and removal of components, but slower iteration.
//     SparseSet,
// }

// #[derive(Debug)]
// pub struct ComponentInfo {
//     id: ComponentId,
//     descriptor: ComponentDescriptor,
// }

// impl ComponentInfo {
//     #[inline]
//     pub fn id(&self) -> ComponentId {
//         self.id
//     }

//     #[inline]
//     pub fn name(&self) -> &str {
//         &self.descriptor.name
//     }

//     #[inline]
//     pub fn type_id(&self) -> Option<TypeId> {
//         self.descriptor.type_id
//     }

//     #[inline]
//     pub fn layout(&self) -> Layout {
//         self.descriptor.layout
//     }

//     #[inline]
//     /// Get the function which should be called to clean up values of
//     /// the underlying component type. This maps to the
//     /// [`Drop`] implementation for 'normal' Rust components
//     ///
//     /// Returns `None` if values of the underlying component type don't
//     /// need to be dropped, e.g. as reported by [`needs_drop`].
//     pub fn drop(&self) -> Option<unsafe fn(OwningPtr<'_>)> {
//         self.descriptor.drop
//     }

//     #[inline]
//     pub fn storage_type(&self) -> StorageType {
//         self.descriptor.storage_type
//     }

//     #[inline]
//     pub fn is_send_and_sync(&self) -> bool {
//         self.descriptor.is_send_and_sync
//     }

//     /// Create a new [`ComponentInfo`].
//     pub(crate) fn new(id: ComponentId, descriptor: ComponentDescriptor) -> Self {
//         ComponentInfo { id, descriptor }
//     }
// }

// /// A semi-opaque value which uniquely identifies the type of a [`Component`] within a
// /// [`World`](crate::world::World).
// ///
// /// Each time a new `Component` type is registered within a `World` using
// /// [`World::init_component`](crate::world::World::init_component) or
// /// [`World::init_component_with_descriptor`](crate::world::World::init_component_with_descriptor),
// /// a corresponding `ComponentId` is created to track it.
// ///
// /// While the distinction between `ComponentId` and [`TypeId`] may seem superficial, breaking them
// /// into two separate but related concepts allows components to exist outside of Rust's type system.
// /// Each Rust type registered as a `Component` will have a corresponding `ComponentId`, but additional
// /// `ComponentId`s may exist in a `World` to track components which cannot be
// /// represented as Rust types for scripting or other advanced use-cases.
// ///
// /// A `ComponentId` is tightly coupled to its parent `World`. Attempting to use a `ComponentId` from
// /// one `World` to access the metadata of a `Component` in a different `World` is undefined behaviour
// /// and must not be attempted.
// #[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
// pub struct ComponentId(usize);

// impl ComponentId {
//     #[inline]
//     pub const fn new(index: usize) -> ComponentId {
//         ComponentId(index)
//     }

//     #[inline]
//     pub fn index(self) -> usize {
//         self.0
//     }
// }

// impl SparseSetIndex for ComponentId {
//     #[inline]
//     fn sparse_set_index(&self) -> usize {
//         self.index()
//     }

//     fn get_sparse_set_index(value: usize) -> Self {
//         Self(value)
//     }
// }

// pub struct ComponentDescriptor {
//     name: Cow<'static, str>,
//     // SAFETY: This must remain private. It must match the statically known StorageType of the
//     // associated rust component type if one exists.
//     storage_type: StorageType,
//     // SAFETY: This must remain private. It must only be set to "true" if this component is
//     // actually Send + Sync
//     is_send_and_sync: bool,
//     type_id: Option<TypeId>,
//     layout: Layout,
//     // SAFETY: this function must be safe to call with pointers pointing to items of the type
//     // this descriptor describes.
//     // None if the underlying type doesn't need to be dropped
//     drop: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
// }

// // We need to ignore the `drop` field in our `Debug` impl
// impl std::fmt::Debug for ComponentDescriptor {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("ComponentDescriptor")
//             .field("name", &self.name)
//             .field("storage_type", &self.storage_type)
//             .field("is_send_and_sync", &self.is_send_and_sync)
//             .field("type_id", &self.type_id)
//             .field("layout", &self.layout)
//             .finish()
//     }
// }

// impl ComponentDescriptor {
//     // SAFETY: The pointer points to a valid value of type `T` and it is safe to drop this value.
//     unsafe fn drop_ptr<T>(x: OwningPtr<'_>) {
//         x.drop_as::<T>();
//     }

//     /// Create a new `ComponentDescriptor` for the type `T`.
//     pub fn new<T: Component>() -> Self {
//         Self {
//             name: Cow::Borrowed(std::any::type_name::<T>()),
//             storage_type: StorageType::Table, // T::Storage::STORAGE_TYPE,
//             is_send_and_sync: true,
//             type_id: Some(TypeId::of::<T>()),
//             layout: Layout::new::<T>(),
//             drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
//         }
//     }

//     /// Create a new `ComponentDescriptor`.
//     ///
//     /// # Safety
//     /// - the `drop` fn must be usable on a pointer with a value of the layout `layout`
//     /// - the component type must be safe to access from any thread (Send + Sync in rust terms)
//     pub unsafe fn new_with_layout(
//         name: impl Into<Cow<'static, str>>,
//         storage_type: StorageType,
//         layout: Layout,
//         drop: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
//     ) -> Self {
//         Self {
//             name: name.into(),
//             storage_type,
//             is_send_and_sync: true,
//             type_id: None,
//             layout,
//             drop,
//         }
//     }

//     /// Create a new `ComponentDescriptor` for a resource.
//     ///
//     /// The [`StorageType`] for resources is always [`TableStorage`].
//     pub fn new_resource<T: Resource>() -> Self {
//         Self {
//             name: Cow::Borrowed(std::any::type_name::<T>()),
//             // PERF: `SparseStorage` may actually be a more
//             // reasonable choice as `storage_type` for resources.
//             storage_type: StorageType::Table,
//             is_send_and_sync: true,
//             type_id: Some(TypeId::of::<T>()),
//             layout: Layout::new::<T>(),
//             drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
//         }
//     }

//     fn new_non_send<T: Any>(storage_type: StorageType) -> Self {
//         Self {
//             name: Cow::Borrowed(std::any::type_name::<T>()),
//             storage_type,
//             is_send_and_sync: false,
//             type_id: Some(TypeId::of::<T>()),
//             layout: Layout::new::<T>(),
//             drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
//         }
//     }

//     /// Create a new `ComponentDescriptor` for a resource.
//     ///
//     /// The [`StorageType`] for globals is always [`TableStorage`].
//     pub fn new_global<T: Resource>() -> Self {
//         Self {
//             name: Cow::Borrowed(std::any::type_name::<T>()),
//             // PERF: `SparseStorage` may actually be a more
//             // reasonable choice as `storage_type` for globals.
//             storage_type: StorageType::Table,
//             is_send_and_sync: true,
//             type_id: Some(TypeId::of::<T>()),
//             layout: Layout::new::<T>(),
//             drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
//         }
//     }

//     #[inline]
//     pub fn storage_type(&self) -> StorageType {
//         self.storage_type
//     }

//     #[inline]
//     pub fn type_id(&self) -> Option<TypeId> {
//         self.type_id
//     }

//     #[inline]
//     pub fn name(&self) -> &str {
//         self.name.as_ref()
//     }
// }

// #[derive(Debug, Default)]
// pub struct Components {
//     components: Vec<ComponentInfo>,
//     indices: TypeIdMap<usize>,
//     resource_indices: TypeIdMap<usize>,
//     global_indices: TypeIdMap<usize>,
// }

// impl Components {
//     #[inline]
//     pub fn init_component<T: Component>(&mut self, storages: &mut Storages) -> ComponentId {
//         let type_id = TypeId::of::<T>();

//         let Components {
//             indices,
//             components,
//             ..
//         } = self;
//         let index = indices.entry(type_id).or_insert_with(|| {
//             Components::init_component_inner(components, storages, ComponentDescriptor::new::<T>())
//         });
//         ComponentId(*index)
//     }

//     pub fn init_component_with_descriptor(
//         &mut self,
//         storages: &mut Storages,
//         descriptor: ComponentDescriptor,
//     ) -> ComponentId {
//         let index = Components::init_component_inner(&mut self.components, storages, descriptor);
//         ComponentId(index)
//     }

//     #[inline]
//     fn init_component_inner(
//         components: &mut Vec<ComponentInfo>,
//         storages: &mut Storages,
//         descriptor: ComponentDescriptor,
//     ) -> usize {
//         let index = components.len();
//         let info = ComponentInfo::new(ComponentId(index), descriptor);
//         if info.descriptor.storage_type == StorageType::SparseSet {
//             storages.sparse_sets.get_or_insert(&info);
//         }
//         components.push(info);
//         index
//     }

//     #[inline]
//     pub fn len(&self) -> usize {
//         self.components.len()
//     }

//     #[inline]
//     pub fn is_empty(&self) -> bool {
//         self.components.len() == 0
//     }

//     #[inline]
//     pub fn get_info(&self, id: ComponentId) -> Option<&ComponentInfo> {
//         self.components.get(id.0)
//     }

//     #[inline]
//     pub fn get_name(&self, id: ComponentId) -> Option<&str> {
//         self.get_info(id).map(|descriptor| descriptor.name())
//     }

//     /// # Safety
//     ///
//     /// `id` must be a valid [`ComponentId`]
//     #[inline]
//     pub unsafe fn get_info_unchecked(&self, id: ComponentId) -> &ComponentInfo {
//         debug_assert!(id.index() < self.components.len());
//         self.components.get_unchecked(id.0)
//     }

//     /// Type-erased equivalent of [`Components::component_id`].
//     #[inline]
//     pub fn get_id(&self, type_id: TypeId) -> Option<ComponentId> {
//         self.indices.get(&type_id).map(|index| ComponentId(*index))
//     }

//     /// Returns the [`ComponentId`] of the given [`Component`] type `T`.
//     ///
//     /// The returned `ComponentId` is specific to the `Components` instance
//     /// it was retrieved from and should not be used with another `Components`
//     /// instance.
//     ///
//     /// Returns [`None`] if the `Component` type has not
//     /// yet been initialized using [`Components::init_component`].
//     ///
//     /// ```rust
//     /// use gw_ecs::prelude::*;
//     ///
//     /// let mut world = World::new();
//     ///
//     /// #[derive(Component)]
//     /// struct ComponentA;
//     ///
//     /// let component_a_id = world.init_component::<ComponentA>();
//     ///
//     /// assert_eq!(component_a_id, world.components().component_id::<ComponentA>().unwrap())
//     /// ```
//     #[inline]
//     pub fn component_id<T: Component>(&self) -> Option<ComponentId> {
//         self.get_id(TypeId::of::<T>())
//     }

//     /// Type-erased equivalent of [`Components::resource_id`].
//     #[inline]
//     pub fn get_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
//         self.resource_indices
//             .get(&type_id)
//             .map(|index| ComponentId(*index))
//     }

//     /// Returns the [`ComponentId`] of the given [`Resource`] type `T`.
//     ///
//     /// The returned `ComponentId` is specific to the `Components` instance
//     /// it was retrieved from and should not be used with another `Components`
//     /// instance.
//     ///
//     /// Returns [`None`] if the `Resource` type has not
//     /// yet been initialized using [`Components::init_resource`].
//     ///
//     /// ```rust
//     /// use gw_ecs::prelude::*;
//     ///
//     /// let mut world = World::new();
//     ///
//     /// #[derive(Resource, Default)]
//     /// struct ResourceA;
//     ///
//     /// let resource_a_id = world.init_resource::<ResourceA>();
//     ///
//     /// assert_eq!(resource_a_id, world.components().resource_id::<ResourceA>().unwrap())
//     /// ```
//     #[inline]
//     pub fn resource_id<T: Resource>(&self) -> Option<ComponentId> {
//         self.get_resource_id(TypeId::of::<T>())
//     }

//     #[inline]
//     pub fn init_resource<T: Resource>(&mut self) -> ComponentId {
//         // SAFETY: The [`ComponentDescriptor`] matches the [`TypeId`]
//         unsafe {
//             self.get_or_insert_resource_with(TypeId::of::<T>(), || {
//                 ComponentDescriptor::new_resource::<T>()
//             })
//         }
//     }

//     #[inline]
//     pub fn init_non_send<T: Any>(&mut self) -> ComponentId {
//         // SAFETY: The [`ComponentDescriptor`] matches the [`TypeId`]
//         unsafe {
//             self.get_or_insert_resource_with(TypeId::of::<T>(), || {
//                 ComponentDescriptor::new_non_send::<T>(StorageType::default())
//             })
//         }
//     }

//     /// # Safety
//     ///
//     /// The [`ComponentDescriptor`] must match the [`TypeId`]
//     #[inline]
//     unsafe fn get_or_insert_resource_with(
//         &mut self,
//         type_id: TypeId,
//         func: impl FnOnce() -> ComponentDescriptor,
//     ) -> ComponentId {
//         let components = &mut self.components;
//         let index = self.resource_indices.entry(type_id).or_insert_with(|| {
//             let descriptor = func();
//             let index = components.len();
//             components.push(ComponentInfo::new(ComponentId(index), descriptor));
//             index
//         });

//         ComponentId(*index)
//     }

//     /// Type-erased equivalent of [`Components::resource_id`].
//     #[inline]
//     pub fn get_global_id(&self, type_id: TypeId) -> Option<ComponentId> {
//         self.global_indices
//             .get(&type_id)
//             .map(|index| ComponentId(*index))
//     }

//     /// Returns the [`ComponentId`] of the given [`Resource`] type `T`.
//     ///
//     /// The returned `ComponentId` is specific to the `Components` instance
//     /// it was retrieved from and should not be used with another `Components`
//     /// instance.
//     ///
//     /// Returns [`None`] if the `Resource` type has not
//     /// yet been initialized using [`Components::init_global`].
//     ///
//     /// ```rust
//     /// use gw_ecs::prelude::*;
//     ///
//     /// let mut world = World::new();
//     ///
//     /// #[derive(Resource, Default)]
//     /// struct ResourceA;
//     ///
//     /// let global_a_id = world.init_global::<ResourceA>();
//     ///
//     /// assert_eq!(global_a_id, world.components().global_id::<ResourceA>().unwrap())
//     /// ```
//     #[inline]
//     pub fn global_id<T: Resource>(&self) -> Option<ComponentId> {
//         self.get_global_id(TypeId::of::<T>())
//     }

//     #[inline]
//     pub fn init_global<T: Resource>(&mut self) -> ComponentId {
//         // SAFETY: The [`ComponentDescriptor`] matches the [`TypeId`]
//         unsafe {
//             self.get_or_insert_global_with(TypeId::of::<T>(), || {
//                 ComponentDescriptor::new_global::<T>()
//             })
//         }
//     }

//     #[inline]
//     pub fn init_non_send_global<T: Any>(&mut self) -> ComponentId {
//         // SAFETY: The [`ComponentDescriptor`] matches the [`TypeId`]
//         unsafe {
//             self.get_or_insert_global_with(TypeId::of::<T>(), || {
//                 ComponentDescriptor::new_non_send::<T>(StorageType::default())
//             })
//         }
//     }

//     /// # Safety
//     ///
//     /// The [`ComponentDescriptor`] must match the [`TypeId`]
//     #[inline]
//     unsafe fn get_or_insert_global_with(
//         &mut self,
//         type_id: TypeId,
//         func: impl FnOnce() -> ComponentDescriptor,
//     ) -> ComponentId {
//         let components = &mut self.components;
//         let index = self.global_indices.entry(type_id).or_insert_with(|| {
//             let descriptor = func();
//             let index = components.len();
//             components.push(ComponentInfo::new(ComponentId(index), descriptor));
//             index
//         });

//         ComponentId(*index)
//     }

//     pub fn iter(&self) -> impl Iterator<Item = &ComponentInfo> + '_ {
//         self.components.iter()
//     }
// }

// pub static mut CURRENT_TICK: AtomicU32 = AtomicU32::new(0);

// pub fn current_tick() -> u32 {
//     unsafe { CURRENT_TICK.load(Ordering::Relaxed) }
// }

/// Used to track changes in state between system runs, e.g. components being added or accessed mutably.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Tick {
    pub(crate) tick: u32,
}

impl Tick {
    // pub fn current() -> Self {
    //     Self {
    //         tick: unsafe { CURRENT_TICK.load(Ordering::Relaxed) },
    //     }
    // }

    pub const fn new(tick: u32) -> Self {
        Self { tick }
    }

    #[inline]
    /// Returns `true` if this `Tick` occurred since the system's `last_change_tick`.
    ///
    /// `change_tick` is the current tick of the system, used as a reference to help deal with wraparound.
    pub fn is_newer_than(&self, last_change_tick: u32, change_tick: u32) -> bool {
        // This works even with wraparound because the world tick (`change_tick`) is always "newer" than
        // `last_change_tick` and `self.tick`, and we scan periodically to clamp `ComponentTicks` values
        // so they never get older than `u32::MAX` (the difference would overflow).
        //
        // The clamp here ensures determinism (since scans could differ between app runs).
        let ticks_since_insert = change_tick.wrapping_sub(self.tick).min(MAX_CHANGE_AGE);
        let ticks_since_system = change_tick
            .wrapping_sub(last_change_tick)
            .min(MAX_CHANGE_AGE);

        ticks_since_system > ticks_since_insert
    }

    #[allow(dead_code)]
    pub(crate) fn check_tick(&mut self, change_tick: u32) {
        let age = change_tick.wrapping_sub(self.tick);
        // This comparison assumes that `age` has not overflowed `u32::MAX` before, which will be true
        // so long as this check always runs before that can happen.
        if age > MAX_CHANGE_AGE {
            self.tick = change_tick.wrapping_sub(MAX_CHANGE_AGE);
        }
    }

    /// Manually sets the change tick.
    ///
    /// This is normally done automatically via the [`DerefMut`](std::ops::DerefMut) implementation
    /// on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.
    /// However, components and resources that make use of interior mutability might require manual updates.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use gw_ecs::{world::World, component::ComponentTicks};
    /// let world: World = unimplemented!();
    /// let component_ticks: ComponentTicks = unimplemented!();
    ///
    /// component_ticks.set_changed(world.read_change_tick());
    /// ```
    #[inline]
    pub fn set_changed(&mut self, change_tick: u32) {
        self.tick = change_tick;
    }
}

impl PartialEq<u32> for Tick {
    // Required method
    fn eq(&self, other: &u32) -> bool {
        self.tick == *other
    }
}

/// Wrapper around [`Tick`]s for a single component
#[derive(Copy, Clone, Debug)]
pub struct TickCells<'a> {
    pub added: &'a UnsafeCell<Tick>,
    pub changed: &'a UnsafeCell<Tick>,
}

impl<'a> TickCells<'a> {
    /// # Safety
    /// All cells contained within must uphold the safety invariants of [`UnsafeCellDeref::read`].
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn read(&self) -> ComponentTicks {
        ComponentTicks {
            added: self.added.read(),
            changed: self.changed.read(),
        }
    }
}

/// Records when a component was added and when it was last mutably dereferenced (or added).
#[derive(Copy, Clone, Debug, Default)]
pub struct ComponentTicks {
    pub(crate) added: Tick,
    pub(crate) changed: Tick,
}

impl ComponentTicks {
    #[inline]
    /// Returns `true` if the component was added after the system last ran.
    pub fn is_added(&self, last_change_tick: u32, change_tick: u32) -> bool {
        self.added.is_newer_than(last_change_tick, change_tick)
    }

    #[inline]
    /// Returns `true` if the component was added or mutably dereferenced after the system last ran.
    pub fn is_changed(&self, last_change_tick: u32, change_tick: u32) -> bool {
        self.changed.is_newer_than(last_change_tick, change_tick)
    }

    pub(crate) fn new(change_tick: u32) -> Self {
        Self {
            added: Tick::new(change_tick),
            changed: Tick::new(change_tick),
        }
    }

    /// Manually sets the change tick.
    ///
    /// This is normally done automatically via the [`DerefMut`](std::ops::DerefMut) implementation
    /// on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.
    /// However, components and resources that make use of interior mutability might require manual updates.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use gw_ecs::{world::World, component::ComponentTicks};
    /// let world: World = unimplemented!();
    /// let component_ticks: ComponentTicks = unimplemented!();
    ///
    /// component_ticks.set_changed(world.read_change_tick());
    /// ```
    #[inline]
    pub fn set_changed(&mut self, change_tick: u32) {
        self.changed.set_changed(change_tick);
    }

    pub(crate) fn check_ticks(&mut self, world_tick: u32) {
        self.added.check_tick(world_tick);
        self.changed.check_tick(world_tick);
    }
}

// /// Initialize and fetch a [`ComponentId`] for a specific type.
// ///
// /// # Example
// /// ```rust
// /// # use gw_ecs::{system::Local, component::{Component, ComponentId, ComponentIdFor}};
// /// #[derive(Component)]
// /// struct Player;
// /// fn my_system(component_id: Local<ComponentIdFor<Player>>) {
// ///     let component_id: ComponentId = component_id.into();
// ///     // ...
// /// }
// /// ```
// pub struct ComponentIdFor<T: Component> {
//     component_id: ComponentId,
//     phantom: PhantomData<T>,
// }

// impl<T: Component> FromWorld for ComponentIdFor<T> {
//     fn from_world(world: &mut World) -> Self {
//         Self {
//             component_id: world.init_component::<T>(),
//             phantom: PhantomData,
//         }
//     }
// }

// impl<T: Component> std::ops::Deref for ComponentIdFor<T> {
//     type Target = ComponentId;
//     fn deref(&self) -> &Self::Target {
//         &self.component_id
//     }
// }

// impl<T: Component> From<ComponentIdFor<T>> for ComponentId {
//     fn from(to_component_id: ComponentIdFor<T>) -> ComponentId {
//         *to_component_id
//     }
// }

// impl<'s, T: Component> From<Local<'s, ComponentIdFor<T>>> for ComponentId {
//     fn from(to_component_id: Local<ComponentIdFor<T>>) -> ComponentId {
//         **to_component_id
//     }
// }
