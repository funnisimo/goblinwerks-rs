use gw_ecs::{
    Builder, Component, DenseVecStorage, Entities, LazyUpdate, ReadRes, SystemData, World,
};

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

fn main() {
    let mut world = World::empty(1);

    world.register::<CompA>();
    world.register::<CompB>();

    let entity = world
        .create_entity()
        .with(CompA(99))
        .with(CompB(203))
        .build();

    world.maintain();

    let e2 = {
        let (commands, entities) = <(ReadRes<LazyUpdate>, Entities)>::fetch(&world);

        commands.insert_global(GlobalA(32));
        commands.insert_resource(ResA(24));
        commands.insert_component(entity, CompA(14));
        commands.remove_component::<CompB>(entity);
        commands.create_entity(&entities).with(CompA(56)).build()
    };

    assert!(world.try_read_global::<GlobalA>().is_none());
    assert!(world.try_read_resource::<ResA>().is_none());
    assert_eq!(world.read_component::<CompA>().get(entity).unwrap().0, 99);
    assert_eq!(world.read_component::<CompB>().get(entity).unwrap().0, 203);

    world.maintain();

    assert!(world.try_read_global::<GlobalA>().is_some());
    assert!(world.try_read_resource::<ResA>().is_some());
    assert_eq!(world.read_component::<CompA>().get(entity).unwrap().0, 14);
    assert!(world.read_component::<CompB>().get(entity).is_none());

    {
        let (commands,) = <(ReadRes<LazyUpdate>,)>::fetch(&world);
        commands.remove_global::<GlobalA>();
        commands.remove_resource::<ResA>();
        commands.delete_entity(e2);
    }

    assert!(world.try_read_global::<GlobalA>().is_some());
    assert!(world.try_read_resource::<ResA>().is_some());
    assert_eq!(world.read_component::<CompA>().get(entity).unwrap().0, 14);
    assert!(world.read_component::<CompB>().get(entity).is_none());
    assert!(world.entities().is_alive(e2));

    world.maintain();

    assert!(world.try_read_global::<GlobalA>().is_none());
    assert!(world.try_read_resource::<ResA>().is_none());
    assert!(!world.entities().is_alive(e2));
}
