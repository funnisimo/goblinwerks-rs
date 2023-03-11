use gw_app::ecs::{move_entity, register_component, Deserialize, Entity, Serialize, World};

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Invisible;

fn main() {
    // create a registry which uses strings as the external type ID
    register_component::<Position>("Position");
    register_component::<Velocity>("Velocity");
    register_component::<f32>("f32");
    register_component::<bool>("bool");

    // CREATE + POPULATE SOURCE WORLD
    let mut world = World::default();

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    let _entities: &[Entity] = world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    ]);

    // push a component tuple into the world to create an entity that we will move
    let entity: Entity = world.push((Position { x: 3.0, y: 4.0 },));
    // or
    // .. see what happens if the entity has an unregistered component
    // let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Invisible));

    println!("Original Entity = {:?}", entity);

    // CREATE + POPULATE DEST WORLD
    let mut world_2 = World::default();

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    world_2.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    ]);

    world_2.extend(vec![
        (Position { x: 0.0, y: 0.0 },),
        (Position { x: 1.0, y: 1.0 },),
        (Position { x: 2.0, y: 2.0 },),
    ]);

    // Here is the move logic

    let new_entity = move_entity(entity, &mut world, &mut world_2);

    if let Some(_) = world.entry(entity) {
        println!("ERROR - Original entity still there!");
    }

    let entry = world_2.entry(new_entity).unwrap();
    println!("Moved entity({:?}) -> new entity({:?})", entity, new_entity);
    println!("- pos = {:?}", entry.get_component::<Position>().unwrap());
}
