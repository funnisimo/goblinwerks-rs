use gw_ecs::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct UniqueA(u32);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct GlobalA(u32);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct GlobalB(u32);

fn global_system(
    global_a: GlobalRef<GlobalA>,
    global_b: GlobalRef<GlobalB>,
    unique_a: ResRef<UniqueA>,
) {
    println!(
        "System = A:{:?}, B:{:?} + U:{:?}",
        global_a.0, global_b.0, unique_a.0
    );
}

fn main() {
    let mut ecs = Ecs::default();

    assert!(ecs.try_read_global::<GlobalA>().is_none());
    assert!(!ecs.has_global::<GlobalA>());

    ecs.insert_global(GlobalA(32));
    ecs.insert_global(GlobalB(64));

    let world = ecs.current_world_mut();
    world.insert_resource(UniqueA(1));

    assert!(world.has_global::<GlobalA>());
    assert_eq!(world.read_global::<GlobalA>().0, 32);

    assert!(world.try_read_global::<GlobalA>().is_some());

    let mut schedule = Schedule::new();
    schedule.add_system(global_system);

    schedule.run(world);

    let mut world = World::empty("TACO");
    world.insert_resource(UniqueA(2));

    ecs.insert_world(world); // Sets the globals
    ecs.set_current_world("TACO").unwrap();

    schedule.run(ecs.current_world_mut());

    let world = ecs.current_world();
    assert!(world.has_global::<GlobalA>());
    assert_eq!(world.read_global::<GlobalA>().0, 32);
}
