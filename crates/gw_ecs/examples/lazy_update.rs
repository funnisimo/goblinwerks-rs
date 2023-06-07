use gw_ecs::prelude::*;

#[derive(Default, Component, Debug)]
struct CompA(u32);

#[derive(Default, Component, Debug)]
struct CompB(u32);

#[derive(Default, Debug)]
struct GlobalA(u32);

#[derive(Default, Debug)]
struct GlobalB(u32);

#[derive(Default, Debug)]
struct ResA(u32);

#[derive(Default, Debug)]
struct ResB(u32);

fn delayed_system_a(mut commands: Commands, entities: Entities) {
    let entity = entities.join().next().unwrap();

    commands.insert_global(GlobalA(32));
    commands.insert_resource(ResA(24));
    commands.insert_component(entity, CompA(14));
    commands.remove_component::<CompB>(entity);
    commands.spawn(CompA(56)).id();
}

fn delayed_system_b(mut commands: Commands, entities: Entities) {
    commands.remove_global::<GlobalA>();
    commands.remove_resource::<ResA>();

    let e2 = entities.join().last().unwrap();
    commands.entity(e2).despawn();
}

fn main() {
    let mut world = World::default();

    world.register::<CompA>();
    world.register::<CompB>();

    let entity = world.create_entity().with(CompA(99)).with(CompB(203)).id();

    assert!(world.try_read_global::<GlobalA>().is_none());
    assert!(world.try_read_resource::<ResA>().is_none());
    assert_eq!(world.read_component::<CompA>().get(entity).unwrap().0, 99);
    assert_eq!(world.read_component::<CompB>().get(entity).unwrap().0, 203);

    // When you exec a system it will flush the deferred actions (commands)
    // No need to maintain
    world.exec(delayed_system_a);

    assert!(world.try_read_global::<GlobalA>().is_some());
    assert!(world.try_read_resource::<ResA>().is_some());
    assert_eq!(world.read_component::<CompA>().get(entity).unwrap().0, 14);
    assert!(world.read_component::<CompB>().get(entity).is_none());

    world.exec(delayed_system_b);

    assert!(world.try_read_global::<GlobalA>().is_none());
    assert!(world.try_read_resource::<ResA>().is_none());

    assert_eq!(world.entities().join().count(), 1);
}
