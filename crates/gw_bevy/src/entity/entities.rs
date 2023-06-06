use super::{EntitiesRes, Entity, Generation, Index};
use crate::{
    prelude::Join,
    resources::{ReadUnique, ResRef},
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
pub type Entities<'a> = ReadUnique<'a, EntitiesRes>;

// Join for ResRef<EntitiesRes>
impl<'a> Join for &'a Entities<'a> {
    type Mask = BitSetOr<&'a BitSet, &'a AtomicBitSet>;
    type Item = Entity;
    type Storage = Self;

    unsafe fn open(self) -> (Self::Mask, Self, u32, u32) {
        (
            BitSetOr(&self.alloc.alive, &self.alloc.raised),
            self,
            self.last_system_tick,
            self.world_tick,
        )
    }

    unsafe fn get(
        v: &mut &'a Entities<'a>,
        idx: Index,
        _last_system_tick: u32,
        _world_tick: u32,
    ) -> Option<Entity> {
        let gen = v
            .alloc
            .generation(idx)
            .map(|gen| if gen.is_alive() { gen } else { gen.raised() })
            .unwrap_or_else(Generation::one);
        Some(Entity(idx, gen))
    }
}

#[cfg(feature = "parallel")]
unsafe impl<'a> ParJoin for &'a Entities<'a> {}

impl<'a> Join for &'a ResRef<'a, EntitiesRes> {
    type Mask = BitSetOr<&'a BitSet, &'a AtomicBitSet>;
    type Item = Entity;
    type Storage = Self;

    unsafe fn open(self) -> (Self::Mask, Self, u32, u32) {
        (
            BitSetOr(&self.alloc.alive, &self.alloc.raised),
            self,
            self.last_system_tick,
            self.world_tick,
        )
    }

    unsafe fn get(
        v: &mut &'a ResRef<EntitiesRes>,
        idx: Index,
        _last_system_tick: u32,
        _world_tick: u32,
    ) -> Option<Entity> {
        let gen = v
            .alloc
            .generation(idx)
            .map(|gen| if gen.is_alive() { gen } else { gen.raised() })
            .unwrap_or_else(Generation::one);
        Some(Entity(idx, gen))
    }
}

#[cfg(feature = "parallel")]
unsafe impl<'a> ParJoin for &'a ResRef<'a, EntitiesRes> {}

// unsafe impl<'a, T: Resource + Send + Sync> ReadOnlySystemParam for ReadUnique<'a, T> {}

// // SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// // conflicts with any prior access, a panic will occur.
// unsafe impl<'a, T: Resource + Send + Sync> SystemParam for ReadUnique<'a, T> {
//     type State = ();
//     type Item<'w, 's> = ReadUnique<'w, T>;

//     fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
//         // world.ensure_resource::<T>();

//         let combined_access = &system_meta.component_access_set;
//         let item = AccessItem::Unique(ResourceId::of::<T>());
//         assert!(
//             !combined_access.has_write(&item),
//             "error[B0002]: Res<{}> in system {} conflicts with a previous ResMut<{0}> access. Consider removing the duplicate access.",
//             std::any::type_name::<T>(),
//             system_meta.name,
//         );
//         system_meta.component_access_set.add_read(item);

//         // let archetype_component_id = world
//         //     .get_resource_archetype_component_id(component_id)
//         //     .unwrap();
//         // system_meta
//         //     .archetype_component_access
//         //     .add_read(archetype_component_id);

//         // component_id
//     }

//     #[inline]
//     unsafe fn get_param<'w, 's>(
//         &mut _component_id: &'s mut Self::State,
//         system_meta: &SystemMeta,
//         world: &'w World,
//         change_tick: u32,
//     ) -> Self::Item<'w, 's> {
//         world
//             .resources
//             .get::<T>(system_meta.last_run_tick, change_tick)
//             .map(|read| ReadUnique::new(read))
//             .unwrap_or_else(|| {
//                 panic!(
//                     "Resource requested by {} does not exist: {}",
//                     system_meta.name,
//                     std::any::type_name::<T>()
//                 )
//             })
//         // Res {
//         //     value: ptr.deref(),
//         //     ticks: Ticks {
//         //         added: ticks.added.deref(),
//         //         changed: ticks.changed.deref(),
//         //         last_change_tick: system_meta.last_change_tick,
//         //         change_tick,
//         //     },
//         // }
//     }
// }
