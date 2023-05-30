use super::{ReadGlobal, WriteGlobal};
use crate::{
    component::ComponentId,
    prelude::World,
    system::{ReadOnlySystemParam, Resource, SystemMeta, SystemParam},
};

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource> SystemParam for ReadGlobal<'a, T> {
    type State = ComponentId;
    type Item<'w, 's> = ReadGlobal<'w, T>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        let component_id = world.initialize_global::<T>();

        let combined_access = system_meta.component_access_set.combined_access();
        assert!(
            !combined_access.has_write(component_id),
            "error[B0002]: Res<{}> in system {} conflicts with a previous ResMut<{0}> access. Consider removing the duplicate access.",
            std::any::type_name::<T>(),
            system_meta.name,
        );
        system_meta
            .component_access_set
            .add_unfiltered_read(component_id);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_read(archetype_component_id);

        component_id
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .get_global::<T>()
            .map(|read| ReadGlobal::new(read, system_meta.last_change_tick, change_tick))
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
unsafe impl<'a, T: Resource> ReadOnlySystemParam for Option<ReadGlobal<'a, T>> {}

// SAFETY: this impl defers to `Res`, which initializes and validates the correct world access.
unsafe impl<'a, T: Resource> SystemParam for Option<ReadGlobal<'a, T>> {
    type State = ComponentId;
    type Item<'w, 's> = Option<ReadGlobal<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        ReadGlobal::<'a, T>::init_state(world, system_meta)
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
            .get_global::<T>()
            .map(|read| ReadGlobal::new(read, system_meta.last_change_tick, change_tick))
    }
}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource> SystemParam for WriteGlobal<'a, T> {
    type State = ComponentId;
    type Item<'w, 's> = WriteGlobal<'w, T>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        let component_id = world.initialize_global::<T>();

        // let component_id = world.initialize_resource::<T>();
        let combined_access = system_meta.component_access_set.combined_access();
        if combined_access.has_write(component_id) {
            panic!(
                "error[B0002]: WriteGlobal<{}> in system {} conflicts with a previous WriteGlobal<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        } else if combined_access.has_read(component_id) {
            panic!(
                "error[B0002]: WriteGlobal<{}> in system {} conflicts with a previous ReadGlobal<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        }
        system_meta
            .component_access_set
            .add_unfiltered_write(component_id);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_write(archetype_component_id);

        component_id
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
            .get_global_mut::<T>()
            .map(|read| WriteGlobal::new(read, system_meta.last_change_tick, change_tick))
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
unsafe impl<'a, T: Resource> SystemParam for Option<WriteGlobal<'a, T>> {
    type State = ComponentId;
    type Item<'w, 's> = Option<WriteGlobal<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        WriteGlobal::<T>::init_state(world, system_meta)
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
            .get_global_mut::<T>()
            .map(|read| WriteGlobal::new(read, system_meta.last_change_tick, change_tick))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::prelude::*;

    #[derive(Default, Debug)]
    struct GlobalA(u32);

    #[derive(Default, Debug)]
    struct GlobalB(u32);

    #[test]
    fn global_basic() {
        let mut world = World::default();

        world.init_global::<GlobalA>();
        world.init_global::<GlobalB>();

        fn my_system(a: ReadGlobal<GlobalA>, mut b: WriteGlobal<GlobalB>) {
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

        world.init_global::<GlobalA>();
        // world.ensure_global::<GlobalB>();

        fn my_system(a: Option<ReadGlobal<GlobalA>>, b: Option<WriteGlobal<GlobalB>>) {
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

        // world.ensure_global::<GlobalA>();
        // world.ensure_global::<GlobalB>();

        fn my_system(a: ReadGlobal<GlobalA>, b: Option<WriteGlobal<GlobalB>>) {
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

        // world.ensure_global::<GlobalA>();
        // world.ensure_global::<GlobalB>();

        fn my_system(a: Option<ReadGlobal<GlobalA>>, mut b: WriteGlobal<GlobalB>) {
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
    fn global_read_with_resource_read() {
        let mut world = World::default();

        world.init_global::<GlobalA>();
        world.init_resource::<GlobalA>();

        fn my_system(a: ReadGlobal<GlobalA>, b: Res<GlobalA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    fn global_write_with_resource_write() {
        let mut world = World::default();

        world.init_global::<GlobalA>();
        world.init_resource::<GlobalA>();

        fn my_system(a: WriteGlobal<GlobalA>, b: ResMut<GlobalA>) {
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

        world.init_global::<GlobalA>();
        world.init_resource::<GlobalA>();

        fn my_system(a: WriteGlobal<GlobalA>, b: Res<GlobalA>) {
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

        world.init_global::<GlobalA>();
        world.init_resource::<GlobalA>();

        fn my_system(a: ReadGlobal<GlobalA>, b: ResMut<GlobalA>) {
            println!("a = {}", a.0);
            println!("b = {}", b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn global_double_write() {
        let mut world = World::default();

        world.init_global::<GlobalA>();

        fn my_system(a: WriteGlobal<GlobalA>, mut b: WriteGlobal<GlobalB>) {
            println!("a = {}", a.0);
            let was = b.0;
            b.0 += 1;
            println!("b = {} -> {}", was, b.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_system);

        schedule.run(&mut world);
    }
}
