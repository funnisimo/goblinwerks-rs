use super::{GlobalMut, GlobalRef, ReadNonSendGlobal, WriteNonSendGlobal};
use crate::{
    access::AccessItem,
    // component::ComponentId,
    prelude::World,
    resources::ResourceId,
    system::{ReadOnlySystemParam, Resource, SystemMeta, SystemParam},
};

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for GlobalRef<'a, T> {
    type State = ();
    type Item<'w, 's> = GlobalRef<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.initialize_global::<T>();

        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Global(ResourceId::of::<T>());
        assert!(
            !combined_access.has_write(&item),
            "error[B0002]: ResRef<{}> in system {} conflicts with a previous ResMut<{0}> access. Consider removing the duplicate access.",
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
            .globals
            .try_fetch::<T>(system_meta.last_run_tick, change_tick)
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
unsafe impl<'a, T: Resource + Send + Sync> ReadOnlySystemParam for Option<GlobalRef<'a, T>> {}

// SAFETY: this impl defers to `Res`, which initializes and validates the correct world access.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for Option<GlobalRef<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<GlobalRef<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        GlobalRef::<'a, T>::init_state(world, system_meta)
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
            .globals
            .try_fetch::<T>(system_meta.last_run_tick, change_tick)
    }
}

// SAFETY: Res ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this Res
// conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for GlobalMut<'a, T> {
    type State = ();
    type Item<'w, 's> = GlobalMut<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        // world.initialize_global::<T>();

        // let component_id = world.initialize_resource::<T>();
        let combined_access = &system_meta.component_access_set;
        let item = AccessItem::Global(ResourceId::of::<T>());
        if combined_access.has_write(&item) {
            panic!(
                "error[B0002]: GlobalMut<{}> in system {} conflicts with a previous GlobalMut<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        } else if combined_access.has_read(&item) {
            panic!(
                "error[B0002]: GlobalMut<{}> in system {} conflicts with a previous GlobalRef<{0}> access. Consider removing the duplicate access.",
                std::any::type_name::<T>(), system_meta.name);
        }
        system_meta.component_access_set.add_write(item);

        // let archetype_component_id = world
        //     .get_resource_archetype_component_id(component_id)
        //     .unwrap();
        // system_meta
        //     .archetype_component_access
        //     .add_write(archetype_component_id);
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
            .globals
            .try_fetch_mut::<T>(system_meta.last_run_tick, change_tick)
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
unsafe impl<'a, T: Resource + Send + Sync> SystemParam for Option<GlobalMut<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<GlobalMut<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        GlobalMut::<T>::init_state(world, system_meta)
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
            .globals
            .try_fetch_mut::<T>(system_meta.last_run_tick, change_tick)
    }
}

// NonSend Globals

// SAFETY: Only reads a single World non-send resource
unsafe impl<'w, T: 'static> ReadOnlySystemParam for ReadNonSendGlobal<'w, T> {}

// SAFETY: NonSendComponentId and ArchetypeComponentId access is applied to SystemMeta. If this
// NonSend conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: 'static> SystemParam for ReadNonSendGlobal<'a, T> {
    type State = ();
    type Item<'w, 's> = ReadNonSendGlobal<'w, T>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        system_meta.set_non_send();

        // world.initialize_non_send_global::<T>();
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
            .globals
            .try_fetch::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadNonSendGlobal::new(read))
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
unsafe impl<T: 'static> ReadOnlySystemParam for Option<ReadNonSendGlobal<'_, T>> {}

// SAFETY: this impl defers to `NonSend`, which initializes and validates the correct world access.
unsafe impl<T: 'static> SystemParam for Option<ReadNonSendGlobal<'_, T>> {
    type State = ();
    type Item<'w, 's> = Option<ReadNonSendGlobal<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        ReadNonSendGlobal::<T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .globals
            .try_fetch::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| ReadNonSendGlobal::new(read))
    }
}

// SAFETY: NonSendMut ComponentId and ArchetypeComponentId access is applied to SystemMeta. If this
// NonSendMut conflicts with any prior access, a panic will occur.
unsafe impl<'a, T: 'static> SystemParam for WriteNonSendGlobal<'a, T> {
    type State = ();
    type Item<'w, 's> = WriteNonSendGlobal<'w, T>;

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
            .globals
            .try_fetch_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteNonSendGlobal::new(read))
            .unwrap_or_else(|| {
                panic!(
                    "Global requested by {} does not exist: {}",
                    system_meta.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

// SAFETY: this impl defers to `NonSendMut`, which initializes and validates the correct world access.
unsafe impl<'a, T: 'static> SystemParam for Option<WriteNonSendGlobal<'a, T>> {
    type State = ();
    type Item<'w, 's> = Option<WriteNonSendGlobal<'w, T>>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        WriteNonSendGlobal::<T>::init_state(world, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        &mut _component_id: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w, 's> {
        world
            .globals
            .try_fetch_mut::<T>(system_meta.last_run_tick, change_tick)
            .map(|read| WriteNonSendGlobal::new(read))
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[derive(Default, Debug)]
    struct GlobalA(u32);

    #[derive(Default, Debug)]
    struct GlobalB(u32);

    #[test]
    fn global_basic() {
        let mut world = World::default();

        world.ensure_global::<GlobalA>();
        world.ensure_global::<GlobalB>();

        fn my_system(a: GlobalRef<GlobalA>, mut b: GlobalMut<GlobalB>) {
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

        world.ensure_global::<GlobalA>();
        // world.ensure_global::<GlobalB>();

        fn my_system(a: Option<GlobalRef<GlobalA>>, b: Option<GlobalMut<GlobalB>>) {
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

        fn my_system(a: GlobalRef<GlobalA>, b: Option<GlobalMut<GlobalB>>) {
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

        fn my_system(a: Option<GlobalRef<GlobalA>>, mut b: GlobalMut<GlobalB>) {
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

        world.ensure_global::<GlobalA>();
        world.ensure_resource::<GlobalA>();

        fn my_system(a: GlobalRef<GlobalA>, b: ResRef<GlobalA>) {
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

        world.ensure_global::<GlobalA>();
        world.ensure_resource::<GlobalA>();

        fn my_system(a: GlobalMut<GlobalA>, b: ResMut<GlobalA>) {
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

        world.ensure_global::<GlobalA>();
        world.ensure_resource::<GlobalA>();

        fn my_system(a: GlobalMut<GlobalA>, b: ResRef<GlobalA>) {
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

        world.ensure_global::<GlobalA>();
        world.ensure_resource::<GlobalA>();

        fn my_system(a: GlobalRef<GlobalA>, b: ResMut<GlobalA>) {
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

        world.ensure_global::<GlobalA>();

        fn my_system(a: GlobalMut<GlobalA>, mut b: GlobalMut<GlobalB>) {
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
    fn global_non_send() {
        let mut world = World::default();

        world.ensure_global_non_send::<NonSendA>();

        fn my_write_system(a: WriteNonSendGlobal<NonSendA>) {
            println!("in write system - {:?}", a.0);
        }

        fn my_read_system(a: ReadNonSendGlobal<NonSendA>) {
            println!("in read system - {:?}", a.0);
        }

        let mut schedule = Schedule::default();
        schedule.add_system(my_write_system);
        schedule.add_system(my_read_system);

        schedule.run(&mut world);
    }

    // #[test]
    // fn global_non_send_wrong_fails_compile() {
    //     let mut world = World::default();

    //     world.ensure_global::<NonSendA>();

    //     fn my_write_system(a: GlobalMut<NonSendA>) {
    //         println!("in write system - {:?}", a.0);
    //     }

    //     fn my_read_system(a: GlobalRef<NonSendA>) {
    //         println!("in read system - {:?}", a.0);
    //     }

    //     let mut schedule = Schedule::default();
    //     schedule.add_system(my_write_system);
    //     schedule.add_system(my_read_system);

    //     schedule.run(&mut world);
    // }
}
