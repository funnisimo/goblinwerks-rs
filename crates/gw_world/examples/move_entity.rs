use gw_app::ecs::{register_component, Deserialize, Entity, Serialize};
use gw_world::level::{move_entity, Level};

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
    let mut source = Level::new("SOURCE");

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    let _entities: &[Entity] = source.world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    ]);

    // push a component tuple into the world to create an entity that we will move
    let entity: Entity = source.world.push((Position { x: 3.0, y: 4.0 },));
    // or
    // .. see what happens if the entity has an unregistered component
    // let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Invisible));

    println!("Original Entity = {:?}", entity);

    // CREATE + POPULATE DEST WORLD
    let mut destination = Level::new("DESTINATION");

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    destination.world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    ]);

    destination.world.extend(vec![
        (Position { x: 0.0, y: 0.0 },),
        (Position { x: 1.0, y: 1.0 },),
        (Position { x: 2.0, y: 2.0 },),
    ]);

    // Here is the move logic

    let new_entity = move_entity(entity, &mut source, &mut destination);

    if let Some(_) = source.world.entry(entity) {
        println!("ERROR - Original entity still there!");
    }

    let entry = destination.world.entry(new_entity).unwrap();
    println!("Moved entity({:?}) -> new entity({:?})", entity, new_entity);
    println!("- pos = {:?}", entry.get_component::<Position>().unwrap());
}
