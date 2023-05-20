use super::{ReadOnlySystemParam, SystemMeta, SystemParam, SystemParamItem};
use crate::{
    world::{FromWorld, WorldId},
    World,
};

pub const MAX_CHANGE_AGE: u32 = u32::MAX;

// TODO: Actually use this in FunctionSystem. We should probably only do this once Systems are constructed using a World reference
// (to avoid the need for unwrapping to retrieve SystemMeta)
/// Holds on to persistent state required to drive [`SystemParam`] for a [`System`].
///
/// This is a very powerful and convenient tool for working with exclusive world access,
/// allowing you to fetch data from the [`World`] as if you were running a [`System`].
///
/// Borrow-checking is handled for you, allowing you to mutably access multiple compatible system parameters at once,
/// and arbitrary system parameters (like [`EventWriter`](crate::event::EventWriter)) can be conveniently fetched.
///
/// For an alternative approach to split mutable access to the world, see [`World::resource_scope`].
///
/// # Warning
///
/// [`SystemState`] values created can be cached to improve performance,
/// and *must* be cached and reused in order for system parameters that rely on local state to work correctly.
/// These include:
/// - [`Added`](crate::query::Added) and [`Changed`](crate::query::Changed) query filters
/// - [`Local`](crate::system::Local) variables that hold state
/// - [`EventReader`](crate::event::EventReader) system parameters, which rely on a [`Local`](crate::system::Local) to track which events have been seen
///
/// # Example
///
/// Basic usage:
/// ```rust
/// use bevy_ecs::prelude::*;
/// use bevy_ecs::{system::SystemState};
/// use bevy_ecs::event::Events;
///
/// struct MyEvent;
/// #[derive(Resource)]
/// struct MyResource(u32);
///
/// #[derive(Component)]
/// struct MyComponent;
///
/// // Work directly on the `World`
/// let mut world = World::new();
/// world.init_resource::<Events<MyEvent>>();
///
/// // Construct a `SystemState` struct, passing in a tuple of `SystemParam`
/// // as if you were writing an ordinary system.
/// let mut system_state: SystemState<(
///     EventWriter<MyEvent>,
///     Option<ResMut<MyResource>>,
///     Query<&MyComponent>,
///     )> = SystemState::new(&mut world);
///
/// // Use system_state.get_mut(&mut world) and unpack your system parameters into variables!
/// // system_state.get(&world) provides read-only versions of your system parameters instead.
/// let (event_writer, maybe_resource, query) = system_state.get_mut(&mut world);
///
/// // If you are using [`Commands`], you can choose when you want to apply them to the world.
/// // You need to manually call `.apply(world)` on the [`SystemState`] to apply them.
/// ```
/// Caching:
/// ```rust
/// use bevy_ecs::prelude::*;
/// use bevy_ecs::{system::SystemState};
/// use bevy_ecs::event::Events;
///
/// struct MyEvent;
/// #[derive(Resource)]
/// struct CachedSystemState {
///    event_state: SystemState<EventReader<'static, 'static, MyEvent>>
/// }
///
/// // Create and store a system state once
/// let mut world = World::new();
/// world.init_resource::<Events<MyEvent>>();
/// let initial_state: SystemState<EventReader<MyEvent>>  = SystemState::new(&mut world);
///
/// // The system state is cached in a resource
/// world.insert_resource(CachedSystemState{event_state: initial_state});
///
/// // Later, fetch the cached system state, saving on overhead
/// world.resource_scope(|world, mut cached_state: Mut<CachedSystemState>| {
///     let mut event_reader = cached_state.event_state.get_mut(world);
///
///     for events in event_reader.iter() {
///         println!("Hello World!");
///     };
/// });
/// ```
pub struct SystemState<Param: SystemParam + 'static> {
    meta: SystemMeta,
    param_state: Param::State,
    world_id: WorldId,
    // archetype_generation: ArchetypeGeneration,
}

impl<Param: SystemParam> SystemState<Param> {
    pub fn new(world: &mut World) -> Self {
        let mut meta = SystemMeta::new::<Param>();
        meta.last_change_tick = world.change_tick().wrapping_sub(MAX_CHANGE_AGE);
        let param_state = Param::init_state(world, &mut meta);
        Self {
            meta,
            param_state,
            world_id: world.id(),
            // archetype_generation: ArchetypeGeneration::initial(),
        }
    }

    #[inline]
    pub fn meta(&self) -> &SystemMeta {
        &self.meta
    }

    /// Retrieve the [`SystemParam`] values. This can only be called when all parameters are read-only.
    #[inline]
    pub fn get<'w, 's>(&'s mut self, world: &'w World) -> SystemParamItem<'w, 's, Param>
    where
        Param: ReadOnlySystemParam,
    {
        self.validate_world(world);
        // self.update_archetypes(world);
        // SAFETY: Param is read-only and doesn't allow mutable access to World. It also matches the World this SystemState was created with.
        unsafe { self.get_unchecked_manual(world) }
    }

    /// Retrieve the mutable [`SystemParam`] values.
    #[inline]
    pub fn get_mut<'w, 's>(&'s mut self, world: &'w mut World) -> SystemParamItem<'w, 's, Param> {
        self.validate_world(world);
        // self.update_archetypes(world);
        // SAFETY: World is uniquely borrowed and matches the World this SystemState was created with.
        unsafe { self.get_unchecked_manual(world) }
    }

    /// Applies all state queued up for [`SystemParam`] values. For example, this will apply commands queued up
    /// by a [`Commands`](`super::Commands`) parameter to the given [`World`].
    /// This function should be called manually after the values returned by [`SystemState::get`] and [`SystemState::get_mut`]
    /// are finished being used.
    pub fn apply(&mut self, world: &mut World) {
        Param::apply(&mut self.param_state, &self.meta, world);
    }

    #[inline]
    pub fn matches_world(&self, world: &World) -> bool {
        self.world_id == world.id()
    }

    /// Asserts that the [`SystemState`] matches the provided [`World`].
    #[inline]
    fn validate_world(&self, world: &World) {
        assert!(self.matches_world(world), "Encountered a mismatched World. A SystemState cannot be used with Worlds other than the one it was created with.");
    }

    // /// Updates the state's internal view of the `world`'s archetypes. If this is not called before fetching the parameters,
    // /// the results may not accurately reflect what is in the `world`.
    // ///
    // /// This is only required if [`SystemState::get_manual`] or [`SystemState::get_manual_mut`] is being called, and it only needs to
    // /// be called if the `world` has been structurally mutated (i.e. added/removed a component or resource). Users using
    // /// [`SystemState::get`] or [`SystemState::get_mut`] do not need to call this as it will be automatically called for them.
    // #[inline]
    // pub fn update_archetypes(&mut self, world: &World) {
    //     let archetypes = world.archetypes();
    //     let new_generation = archetypes.generation();
    //     let old_generation = std::mem::replace(&mut self.archetype_generation, new_generation);
    //     let archetype_index_range = old_generation.value()..new_generation.value();

    //     for archetype_index in archetype_index_range {
    //         Param::new_archetype(
    //             &mut self.param_state,
    //             &archetypes[ArchetypeId::new(archetype_index)],
    //             &mut self.meta,
    //         );
    //     }
    // }

    /// Retrieve the [`SystemParam`] values. This can only be called when all parameters are read-only.
    /// This will not update the state's view of the world's archetypes automatically nor increment the
    /// world's change tick.
    ///
    /// For this to return accurate results, ensure [`SystemState::update_archetypes`] is called before this
    /// function.
    ///
    /// Users should strongly prefer to use [`SystemState::get`] over this function.
    #[inline]
    pub fn get_manual<'w, 's>(&'s mut self, world: &'w World) -> SystemParamItem<'w, 's, Param>
    where
        Param: ReadOnlySystemParam,
    {
        self.validate_world(world);
        let change_tick = world.change_tick(); // read_change_tick
                                               // SAFETY: Param is read-only and doesn't allow mutable access to World. It also matches the World this SystemState was created with.
        unsafe { self.fetch(world, change_tick) }
    }

    /// Retrieve the mutable [`SystemParam`] values.  This will not update the state's view of the world's archetypes
    /// automatically nor increment the world's change tick.
    ///
    /// For this to return accurate results, ensure [`SystemState::update_archetypes`] is called before this
    /// function.
    ///
    /// Users should strongly prefer to use [`SystemState::get_mut`] over this function.
    #[inline]
    pub fn get_manual_mut<'w, 's>(
        &'s mut self,
        world: &'w mut World,
    ) -> SystemParamItem<'w, 's, Param> {
        self.validate_world(world);
        let change_tick = world.change_tick();
        // SAFETY: World is uniquely borrowed and matches the World this SystemState was created with.
        unsafe { self.fetch(world, change_tick) }
    }

    /// Retrieve the [`SystemParam`] values. This will not update archetypes automatically.
    ///
    /// # Safety
    /// This call might access any of the input parameters in a way that violates Rust's mutability rules. Make sure the data
    /// access is safe in the context of global [`World`] access. The passed-in [`World`] _must_ be the [`World`] the [`SystemState`] was
    /// created with.
    #[inline]
    pub unsafe fn get_unchecked_manual<'w, 's>(
        &'s mut self,
        world: &'w World,
    ) -> SystemParamItem<'w, 's, Param> {
        let change_tick = world.change_tick(); // was increment_change_tick, but why?
        self.fetch(world, change_tick)
    }

    /// # Safety
    /// This call might access any of the input parameters in a way that violates Rust's mutability rules. Make sure the data
    /// access is safe in the context of global [`World`] access. The passed-in [`World`] _must_ be the [`World`] the [`SystemState`] was
    /// created with.
    #[inline]
    unsafe fn fetch<'w, 's>(
        &'s mut self,
        world: &'w World,
        change_tick: u32,
    ) -> SystemParamItem<'w, 's, Param> {
        let param = Param::get_param(&mut self.param_state, &self.meta, world, change_tick);
        self.meta.last_change_tick = change_tick;
        param
    }
}

impl<Param: SystemParam> FromWorld for SystemState<Param> {
    fn from_world(world: &mut World) -> Self {
        Self::new(world)
    }
}
