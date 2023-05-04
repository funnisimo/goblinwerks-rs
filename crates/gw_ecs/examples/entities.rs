use gw_ecs::specs::EntitiesRes;
use gw_ecs::specs::Join;

fn main() {
    let mut entities = EntitiesRes::default();

    entities.create();
    let b = entities.create();
    entities.create();
    let d = entities.create();

    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }

    println!("----");

    let _ = entities.delete(b);
    let _ = entities.delete(d);

    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }

    println!("----");

    let deleted = entities.maintain();

    for del in deleted {
        println!("deleted = {:?}", del);
    }

    for (entity,) in (&entities,).join() {
        println!("entities = {:?} - {}", entity, entities.is_alive(entity));
    }
}
