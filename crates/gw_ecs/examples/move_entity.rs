use gw_ecs::Builder; // For create_entity
use gw_ecs::{Component, DenseVecStorage};
use gw_ecs::{Entity, World}; // For Component derive

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
struct Invisible;

fn main() {
    // CREATE + POPULATE SOURCE WORLD
    let mut world = World::default();

    {
        let entities = world.entities();
        println!("world 1 entities = {:?}", *entities);
    }

    // create a registry which uses strings as the external type ID
    world.register::<Position>();
    world.register::<Velocity>();

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    world
        .create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 1.0, y: 1.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 2.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .build();

    // push a component tuple into the world to create an entity that we will move
    let entity: Entity = world
        .create_entity()
        .with(Position { x: 3.0, y: 4.0 })
        .build();
    // or
    // .. see what happens if the entity has an unregistered component
    // let entity: Entity = world.create_entity().with(Position { x: 0.0, y: 0.0 }).with(Invisible).build();

    println!("Original Entity = {:?}", entity);

    // CREATE + POPULATE DEST WORLD
    let mut world2 = World::default();
    world2.register_components(&world);

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    for i in 0..4 {
        world2
            .create_entity()
            .with(Position {
                x: i as f32,
                y: i as f32,
            })
            .with(Velocity { dx: 0., dy: 0. })
            .build();
        world2
            .create_entity()
            .with(Position {
                x: i as f32 + 10.0,
                y: i as f32 + 10.0,
            })
            .build();
    }

    // Here is the move logic
    let new_entity = world.move_entity_to(entity, &mut world2);

    println!("Moved entity({:?}) -> new entity({:?})", entity, new_entity);
    let positions = world2.read_component::<Position>();
    println!("- pos = {:?}", positions.get(new_entity).unwrap());

    let positions = world.read_component::<Position>();
    println!("- original pos = {:?}", positions.get(entity).is_none());
}
