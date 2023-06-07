use super::{ResMut, ResRef, ResourceId};
use crate::{
    access::AccessItem,
    system::{ReadOnlySystemParam, Resource, SystemMeta, SystemParam},
    world::World,
};
use std::ops::{Deref, DerefMut};
use std::{fmt::Debug, marker::PhantomData};

// RES REF

unsafe impl<'a, T: Resource + Send + Sync> ReadOnlySystemParam for ResRef<'a, T> {}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for ResRef<'a, T> {
    type State = ();
    type Item<'w, 's> = ResRef<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.ensure_resource::<T>();

        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());
        assert!(
            !combined_access.has_write(&item),
            "error[B0002]: Res<{}> in system {} conflicts with a previous ResMut<{0}> access. Consider removing the duplicate access.",
            std::any::type_name::<T>(),
            system_meta.name,
        );
        system_meta.component_access_set.add_read(item);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_read(archetype_component_id);

        // component_id
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get::<T>(system_meta.last_run_tick, change_tick)
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
        // Res {
        //     value: ptr.deref(),
        //     ticks: Ticks {
        //         added: ticks.added.deref(),
        //         changed: ticks.changed.deref(),
        //         last_change_tick: system_meta.last_change_tick,
        //         change_tick,
        //     },
        // }
    }
}

// RES MUT

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for ResMut<'a, T> {
    type State = ();
    type Item<'w, 's> = ResMut<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.initialize_unique::<T>();

        // let component_id = world.initialize_resource::<T>();
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());
        if combined_access.has_write(&item) {
            panic!(
                "error[B0002]: WriteUnique<{}> in system {} conflicts with a previous WriteUnique<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        } else if combined_access.has_read(&item) {
            panic!(
                "error[B0002]: WriteUnique<{}> in system {} conflicts with a previous ReadUnique<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        }
        system_meta.component_access_set.add_write(item);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_write(archetype_component_id);

        // component_ids
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get_mut::<T>(system_meta.last_run_tick, change_tick)
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadUnique<'a, T: Resource + Send + Sync> {
    fetch: ResRef<'a, T>,
}

impl<'a, T> ReadUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    pub(crate) fn new(fetch: ResRef<'a, T>) -> Self {
        ReadUnique { fetch }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }

    /// Returns `true` if the resource was added after the system last ran.
    pub fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    pub fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    #[allow(dead_code)]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for ReadUnique<'w, T>
where
    T: Debug + Resource + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadUnique")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for ReadUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    type Target = ResRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

impl<'a, T> Clone for ReadUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    fn clone(&self) -> Self {
        ReadUnique::new(ResRef::clone(&self.fetch))
    }
}

unsafe impl<'a, T: Resource + Send + Sync> ReadOnlySystemParam for ReadUnique<'a, T> {}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for ReadUnique<'a, T> {
    type State = ();
    type Item<'w, 's> = ReadUnique<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.ensure_resource::<T>();

        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());
        assert!(
            !combined_access.has_write(&item),
            "error[B0002]: Res<{}> in system {} conflicts with a previous ResMut<{0}> access. Consider removing the duplicate access.",
            std::any::type_name::<T>(),
            system_meta.name,
        );
        system_meta.component_access_set.add_read(item);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_read(archetype_component_id);

        // component_id
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadUnique::new(read))
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
        // Res {
        //     value: ptr.deref(),
        //     ticks: Ticks {
        //         added: ticks.added.deref(),
        //         changed: ticks.changed.deref(),
        //         last_change_tick: system_meta.last_change_tick,
        //         change_tick,
        //     },
        // }
    }
}

// SAFETY: Only reads a single World resource
unsafe impl<'a, T: Resource + Send + Sync> ReadOnlySystemParam for Option<ReadUnique<'a, T>> {}

// SAFETY: this impl defers to `Res`, which initializes and validates the correct world access.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for Option<ReadUnique<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<ReadUnique<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        ReadUnique::<'a, T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        // world
        //     .as_unsafe_world_cell_migration_internal()
        //     .get_resource_with_ticks(component_id)
        //     .map(|(ptr, ticks)| Res {
        //         value: ptr.deref(),
        //         ticks: Ticks {
        //             added: ticks.added.deref(),
        //             changed: ticks.changed.deref(),
        //             last_change_tick: system_meta.last_change_tick,
        //             change_tick,
        //         },
        //     })

        world
            .resources
            .get::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadUnique::new(read))
    }
}

impl<'a, T> From<ResRef<'a, T>> for ReadUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    fn from(fetch: ResRef<'a, T>) -> Self {
        ReadUnique { fetch }
    }
}

// WRITE UNIQUE

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use `Option<Write<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct WriteUnique<'a, T: Resource + Send + Sync> {
    fetch: ResMut<'a, T>,
}

impl<'a, T> WriteUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    pub(crate) fn new(fetch: ResMut<'a, T>) -> Self {
        WriteUnique { fetch }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }

    /// Returns `true` if the resource was added after the system last ran.
    pub fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    pub fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    #[allow(dead_code)]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for WriteUnique<'w, T>
where
    T: Debug + Resource + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WriteUnique")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for WriteUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.fetch.deref()
    }
}

impl<'a, T> DerefMut for WriteUnique<'a, T>
where
    T: Resource + Send + Sync,
{
    fn deref_mut(&mut self) -> &mut T {
        self.fetch.deref_mut()
    }
}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for WriteUnique<'a, T> {
    type State = ();
    type Item<'w, 's> = WriteUnique<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.initialize_unique::<T>();

        // let component_id = world.initialize_resource::<T>();
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());
        if combined_access.has_write(&item) {
            panic!(
                "error[B0002]: WriteUnique<{}> in system {} conflicts with a previous WriteUnique<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        } else if combined_access.has_read(&item) {
            panic!(
                "error[B0002]: WriteUnique<{}> in system {} conflicts with a previous ReadUnique<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        }
        system_meta.component_access_set.add_write(item);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_write(archetype_component_id);

        // component_ids
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        // let value = world
        //     .as_unsafe_world_cell_migration_internal()
        //     .get_resource_mut_by_id(component_id)
        //     .unwrap_or_else(|| {
        //         panic!(
        //             "Resource requested by {} does not exist: {}",
        //             system_meta.name,
        //             std::any::type_name::<T>()
        //         )
        //     });
        // ResMut {
        //     value: value.value.deref_mut::<T>(),
        //     ticks: TicksMut {
        //         added: value.ticks.added,
        //         changed: value.ticks.changed,
        //         last_change_tick: system_meta.last_change_tick,
        //         change_tick,
        //     },
        // }

        world
            .resources
            .get_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteUnique::new(read))
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

// SAFETY: this impl defers to `ResMut`, which initializes and validates the correct world access.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for Option<WriteUnique<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<WriteUnique<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        WriteUnique::<T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        // world
        //     .as_unsafe_world_cell_migration_internal()
        //     .get_resource_mut_by_id(component_id)
        //     .map(|value| ResMut {
        //         value: value.value.deref_mut::<T>(),
        //         ticks: TicksMut {
        //             added: value.ticks.added,
        //             changed: value.ticks.changed,
        //             last_change_tick: system_meta.last_change_tick,
        //             change_tick,
        //         },
        //     })

        world
            .resources
            .get_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteUnique::new(read))
    }
}

// NonSend Uniques

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadNonSendUnique<'a, T: 'a> {
    fetch: ResRef<'a, T>,
    phantom: PhantomData<*mut ()>,
}

impl<'a, T> ReadNonSendUnique<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(fetch: ResRef<'a, T>) -> Self {
        ReadNonSendUnique {
            fetch,
            phantom: PhantomData,
        }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }

    /// Returns `true` if the resource was added after the system last ran.
    pub fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    pub fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    #[allow(dead_code)]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for ReadNonSendUnique<'w, T>
where
    T: Debug + Resource,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadNonSendUnique")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for ReadNonSendUnique<'a, T>
where
    T: Resource,
{
    type Target = ResRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

// SAFETY: Only reads a single World non-send resource
unsafe impl<'w, T: 'static> ReadOnlySystemParam for ReadNonSendUnique<'w, T> {}

// SAFETY: NonSendComponentId and ArchetypeComponentId access is applied to SystemMeta. If this
// NonSend conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: 'static> SystemParam for ReadNonSendUnique<'a, T> {
    type State = ();
    type Item<'w, 's> = ReadNonSendUnique<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        system_meta.set_non_send();

        // world.initialize_non_send_unique::<T>();
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());
        assert!(
            !combined_access.has_write(&item),
            "error[B0002]: NonSend<{}> in system {} conflicts with a previous mutable resource access ({0}). Consider removing the duplicate access.",
            std::any::type_name::<T>(),
            system_meta.name,
        );
        system_meta.component_access_set.add_read(item);

        // let archetype_component_id = world
        //     .get_non_send_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_read(archetype_component_id);

        // component_id
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadNonSendUnique::new(read))
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

// SAFETY: Only reads a single World non-send resource
unsafe impl<T: 'static> ReadOnlySystemParam for Option<ReadNonSendUnique<'_, T>> {}

// SAFETY: this impl defers to `NonSend`, which initializes and validates the correct world access.
unsafe impl<T: 'static> SystemParam for Option<ReadNonSendUnique<'_, T>> {
    type State = ();
    type Item<'w, 's> = Option<ReadNonSendUnique<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        ReadNonSendUnique::<T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadNonSendUnique::new(read))
    }
}

// WRITE NON SEND UNIQUE

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct WriteNonSendUnique<'a, T: 'a> {
    fetch: ResMut<'a, T>,
    phantom: PhantomData<*mut ()>,
}

impl<'a, T> WriteNonSendUnique<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(fetch: ResMut<'a, T>) -> Self {
        WriteNonSendUnique {
            fetch,
            phantom: PhantomData,
        }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }

    /// Returns `true` if the resource was added after the system last ran.
    pub fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    pub fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    #[allow(dead_code)]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for WriteNonSendUnique<'w, T>
where
    T: Debug + Resource,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WriteGlobal")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for WriteNonSendUnique<'a, T>
where
    T: Resource,
{
    type Target = ResMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

impl<'a, T> From<ResMut<'a, T>> for WriteNonSendUnique<'a, T>
where
    T: Resource,
{
    fn from(value: ResMut<'a, T>) -> Self {
        WriteNonSendUnique::new(value)
    }
}

// SAFETY: NonSendMut ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this
// NonSendMut conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: 'static> SystemParam for WriteNonSendUnique<'a, T> {
    type State = ();
    type Item<'w, 's> = WriteNonSendUnique<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        system_meta.set_non_send();

        // world.initialize_non_send_resource::<T>();
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<T>());

        if combined_access.has_write(&item) {
            panic!(
                "error[B0002]: NonSendMut<{}> in system {} conflicts with a previous mutable resource access ({0}). Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        } else if combined_access.has_read(&item) {
            panic!(
                "error[B0002]: NonSendMut<{}> in system {} conflicts with a previous immutable resource access ({0}). Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        }
        system_meta.component_access_set.add_write(item);

        // let archetype_component_id = world
        //     .get_non_send_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_write(archetype_component_id);

        // component_id
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteNonSendUnique::new(read))
            .unwrap_or_else(|| {
                panic!(
                    "Unique requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

// SAFETY: this impl defers to `NonSendMut`, which initializes and validates the correct world access.
unsafe impl<'a, T: 'static> SystemParam for Option<WriteNonSendUnique<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<WriteNonSendUnique<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        WriteNonSendUnique::<T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .resources
            .get_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteNonSendUnique::new(read))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        globals::{ReadGlobal, WriteGlobal},
        prelude::*,
    };

    #[derive(Default, Debug)]
    struct UniqueA(u32);

    #[derive(Default, Debug)]
    struct UniqueB(u32);

    #[test]
    fn global_basic() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        world.ensure_resource::<UniqueB>();

        fn my_system(a: ReadUnique<UniqueA>, mut b: WriteUnique<UniqueB>) {
            println!("a = {}", a.0);
            let was = b.0;
            b.0 += 1;
            println!("b = {} -> {}", was, b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_optional_basic() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        // world.ensure_unique::<UniqueB>();

        fn my_system(a: Option<ReadUnique<UniqueA>>, b: Option<WriteUnique<UniqueB>>) {
            match a {
                None => println!("No A"),
                Some(a) => println!("a = {}", a.0),
            }

            match b {
                None => println!("No B"),
                Some(mut b) => {
                    let was = b.0;
                    b.0 += 1;
                    println!("b = {} -> {}", was, b.0);
                }
            }
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn global_not_present_read() {
        let mut world = World::default();

        // world.ensure_unique::<UniqueA>();
        // world.ensure_unique::<UniqueB>();

        fn my_system(a: ReadUnique<UniqueA>, b: Option<WriteUnique<UniqueB>>) {
            println!("a = {}", a.0);

            match b {
                None => println!("No B"),
                Some(mut b) => {
                    let was = b.0;
                    b.0 += 1;
                    println!("b = {} -> {}", was, b.0);
                }
            }
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn global_not_present_write() {
        let mut world = World::default();

        // world.ensure_unique::<UniqueA>();
        // world.ensure_unique::<UniqueB>();

        fn my_system(a: Option<ReadUnique<UniqueA>>, mut b: WriteUnique<UniqueB>) {
            match a {
                None => println!("No A"),
                Some(a) => println!("a = {}", a.0),
            }

            let was = b.0;
            b.0 += 1;
            println!("b = {} -> {}", was, b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_read_with_global_read() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        world.ensure_global::<UniqueA>();

        fn my_system(a: ReadUnique<UniqueA>, b: ReadGlobal<UniqueA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_write_with_global_write() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        world.ensure_global::<UniqueA>();

        fn my_system(a: WriteUnique<UniqueA>, b: WriteGlobal<UniqueA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_write_with_resource_read() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        world.ensure_global::<UniqueA>();

        fn my_system(a: WriteUnique<UniqueA>, b: ReadGlobal<UniqueA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_read_with_resource_write() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();
        world.ensure_global::<UniqueA>();

        fn my_system(a: ReadUnique<UniqueA>, b: WriteGlobal<UniqueA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn unique_double_write() {
        let mut world = World::default();

        world.ensure_resource::<UniqueA>();

        fn my_system(a: WriteUnique<UniqueA>, mut b: WriteUnique<UniqueB>) {
            println!("a = {}", a.0);
            let was = b.0;
            b.0 += 1;
            println!("b = {} -> {}", was, b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    struct NonSendA(*const u8);

    impl Default for NonSendA {
        fn default() -> Self {
            NonSendA(std::ptr::null())
        }
    }

    #[test]
    fn unique_non_send() {
        let mut world = World::default();

        world.ensure_resource_non_send::<NonSendA>();
        assert!(world.has_resource::<NonSendA>());

        fn my_write_system(a: WriteNonSendUnique<NonSendA>) {
            println!("in write system - {:?}", a.0);
        }

        fn my_read_system(a: ReadNonSendUnique<NonSendA>) {
            println!("in read system - {:?}", a.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_write_system);
        schedule.add_system(my_read_system);

        schedule.run(&mut world);
    }

    // #[test]
    // fn unique_non_send_wrong_fails_compile() {
    //     let mut world = World::default();

    //     world.ensure_resource::<NonSendA>();

    //     fn my_write_system(a: WriteUnique<NonSendA>) {
    //         println!("in write system - {:?}", a.0);
    //     }

    //     fn my_read_system(a: ReadUnique<NonSendA>) {
    //         println!("in read system - {:?}", a.0);
    //     }

    //     let mut schedule = Schedule::default();
    //     schedule.add_system(my_write_system);
    //     schedule.add_system(my_read_system);

    //     schedule.run(&mut world);
    // }
}
