use gw_ecs::Builder; // For create_entity
use gw_ecs::{Component, DenseVecStorage};
use gw_ecs::{Ecs, Entity}; // For Component derive

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
    let mut ecs = Ecs::empty();

    println!("Ecs is empty = {}", ecs.is_empty());

    // register components that will be in every world...
    ecs.register::<Position>();
    ecs.register::<Velocity>();

    let entity = {
        let mut world = ecs.create_world("MAIN");

        println!("world 1 = {}", world.id());

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

        let positions = world.read_component::<Position>();
        println!("- pos = {:?}", positions.get(entity).unwrap());

        entity
    };

    {
        // CREATE + POPULATE DEST WORLD
        let world2 = ecs.create_world("SECOND");

        println!("world 2 = {}", world2.id());

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
    }

    // Here is the move logic
    let new_entity = ecs.move_entity(entity, "MAIN", "SECOND");

    ecs.set_current_world("SECOND").unwrap();

    {
        println!("Moved entity({:?}) -> new entity({:?})", entity, new_entity);
        let positions = ecs.read_component::<Position>();
        println!("- pos = {:?}", positions.get(new_entity).unwrap());

        let positions = ecs.get_world("MAIN").unwrap().read_component::<Position>();
        println!(
            "- original pos exists = {:?}",
            positions.get(entity).is_some()
        );
    }

    // Move doesn't require LazyUpdate because it has the world mutably locked
    // TODO - LazyUpdate - move_entity(entity, to_id)

    ecs.maintain();

    println!(":: MAINTAIN ::");

    let positions = ecs.read_component::<Position>();
    println!("- pos = {:?}", positions.get(new_entity).unwrap());

    let positions = ecs.get_world("MAIN").unwrap().read_component::<Position>();
    println!(
        "- original pos exists = {:?}",
        positions.get(entity).is_some()
    );
}
