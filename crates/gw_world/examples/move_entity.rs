use gw_ecs::prelude::{Builder, Component, Ecs, Entity};
use serde::{Deserialize, Serialize};

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Component, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Component, Default)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Component)]
struct Invisible;

fn main() {
    let mut ecs = Ecs::empty();

    // create a registry which uses strings as the external type ID
    ecs.register::<Position>();
    ecs.register::<Velocity>();

    let source = ecs.create_world("SOURCE");

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    for i in 0..3 {
        source
            .create_entity()
            .with(Position {
                x: i as f32,
                y: i as f32,
            })
            .with(Velocity::default())
            .id();
    }

    // push a component tuple into the world to create an entity that we will move
    let entity: Entity = source
        .create_entity()
        .with(Position { x: 3.0, y: 4.0 })
        .id();
    // or
    // .. see what happens if the entity has an unregistered component
    // let entity: Entity = source.create_entity().with(Position { x: 0.0, y: 0.0 }).with(Invisible).id();

    drop(source);
    println!("Original Entity = {:?}", entity);

    // CREATE + POPULATE DEST WORLD
    let destination = ecs.create_world("DESTINATION");

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    for i in 10..13 {
        destination
            .create_entity()
            .with(Position {
                x: i as f32,
                y: i as f32,
            })
            .with(Velocity::default())
            .id();

        destination
            .create_entity()
            .with(Position {
                x: (i + 10) as f32,
                y: (i + 10) as f32,
            })
            .id();
    }

    drop(destination);

    // Here is the move logic

    let new_entity = ecs.move_entity(entity, "SOURCE", "DESTINATION");

    {
        let source = ecs.get_world("SOURCE").unwrap();
        if source.entities().is_alive(entity) {
            println!("ERROR - Original entity still there!");
        }
    }

    let destination = ecs.get_world("DESTINATION").unwrap();
    println!("Moved entity({:?}) -> new entity({:?})", entity, new_entity);
    println!(
        "- pos = {:?}",
        destination
            .read_component::<Position>()
            .get(new_entity)
            .unwrap()
    );
}
