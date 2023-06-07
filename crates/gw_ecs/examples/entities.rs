use gw_ecs::prelude::*;

fn fun_with_entities(entities: Entities) {
    println!("Create some entities");
    entities.create();
    let b = entities.create();
    entities.create();
    let d = entities.create();

    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }

    println!("----");
    println!("Delete 2 entities");

    let _ = entities.delete(b);
    let _ = entities.delete(d);

    println!(" - everybody is still marked alive b/c delete is delayed");
    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }
}

fn print_entities(entities: Entities) {
    println!("Print entities in next system (deleted are gone)");
    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }
}

fn main() {
    let mut world = World::default();

    world.exec(fun_with_entities);

    println!("----");

    world.exec(print_entities);
}
