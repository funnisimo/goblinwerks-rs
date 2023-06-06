use std::ops::Deref;
use super::{EntitiesRes, Entity, Generation, Index};
use crate::{
    access::AccessItem,
    prelude::{Join, World},
    resources::{ ResRef, ResourceId},
    system::{ReadOnlySystemParam, SystemMeta, SystemParam},
};
use hibitset::{AtomicBitSet, BitSet, BitSetOr};

/// A wrapper for a read `Entities` resource.
/// Note that this is just `Read<Entities>`, so
/// you can easily use it in your system:
///
/// ```
/// # use specs::prelude::*;
/// # struct Sys;
/// # impl<'a> System<'a> for Sys {
/// type SystemData = (Entities<'a> /* ... */,);
/// # fn run(&mut self, _: Self::SystemData) {}
/// # }
/// ```
///
/// Please note that you should call `World::maintain`
/// after creating / deleting entities with this resource.
///
/// When `.join`ing on `Entities`, you will need to do it like this:
///
/// ```
/// use specs::prelude::*;
///
/// # struct Pos; impl Component for Pos { type Storage = VecStorage<Self>; }
/// # let mut world = World::new(); world.register::<Pos>();
/// # let entities = world.entities(); let positions = world.write_storage::<Pos>();
/// for (e, pos) in (&entities, &positions).join() {
///     // Do something
/// #   let _ = e;
/// #   let _ = pos;
/// }
/// ```
pub struct Entities<'a> {
    data: ResRef<'a, EntitiesRes>,
}

impl<'a> Entities<'a> {
    pub(crate) fn new(data: ResRef<'a, EntitiesRes>) -> Self {
        Entities { data }
    }
}

impl<'a> Clone for Entities<'a> {
    fn clone(&self) -> Self {
        Entities { data: ResRef::clone(&self.data) }
    }
}

impl<'a> Deref for Entities<'a> {
    type Target = EntitiesRes;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

// Join for ResRef<EntitiesRes>
impl<'a> Join for &'a Entities<'a> {
    type Mask = BitSetOr<&'a BitSet, &'a AtomicBitSet>;
    type Item = Entity;
    type Storage = Self;

    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        (
            BitSetOr(&self.data.alloc.alive, &self.data.alloc.raised),
            self,
            self.data.last_system_tick,
            self.data.world_tick,
        )
    }

    unsafe fn get(
        v: &mut Self::Storage,
        idx: Index,
        _last_system_tick: u32,
        _world_tick: u32,
    ) -> Option<Entity> {
        let gen = v
            .data
            .alloc
            .generation(idx)
            .map(|gen| if gen.is_alive() { gen } else { gen.raised() })
            .unwrap_or_else(Generation::one);
        Some(Entity(idx, gen))
    }
}

#[cfg(feature = "parallel")]
unsafe impl<'a> ParJoin for &'a Entities<'a> {}


unsafe impl<'a> ReadOnlySystemParam for Entities<'a> {}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a> SystemParam for Entities<'a> {
    type State = ();
    type Item<'w, 's> = Entities<'w>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Unique(ResourceId::of::<EntitiesRes>());
        assert!(
            !combined_access.has_write(&item),
            "error[B0002]: Entities in system {} conflicts with a previous mutable Entities access. Consider removing the duplicate access.",
            system_meta.name(),
            
        );
        system_meta.component_access_set.add_read(item);

    }

    fn apply(_state: &mut Self::State, _system_meta: &SystemMeta, world: &mut World) {
        // println!("apply Entities changes");
        let tick = world.current_tick();
        let deleted = world.resources.get_mut::<EntitiesRes>(tick, tick).unwrap().merge();

        if !deleted.is_empty() {
            world.delete_components(&deleted);
        }
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _state: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        let data = world
            .resources
            .get::<EntitiesRes>(system_meta.last_run_tick, change_tick)
            .unwrap_or_else(|| {
                panic!(
                    "Resource requested by {} does not exist: Entities",
                    system_meta.name,
                )
            });

            Entities { data }
    }
}
