use gw_ecs::{Entity, World};
use gw_world::{
    action::move_step::MoveStepAction, being::do_entity_action, hero::Hero, map::Map,
    position::Position, task::TaskResult,
};

/// Try to move toward the hero - will be stopped by the counters.
pub fn shopkeeper(world: &mut World, entity: Entity) -> TaskResult {
    let hero_entity = world.read_resource::<Hero>().entity;

    // log(format!("SHOPKEEPER - {:?}", entity));

    let hero_point = match world.read_component::<Position>().get(hero_entity) {
        None => {
            // log("- no hero_point");
            return TaskResult::Success(100);
        }
        Some(pos) => pos.point(),
    };

    let entity_point = match world.read_component::<Position>().get(entity) {
        None => {
            // log("- no entity point");
            return TaskResult::Success(100);
        }
        Some(pos) => pos.point(),
    };

    // log(format!("- entity_point={:?}", entity_point));
    // log(format!("- hero_point={:?}", hero_point));

    let move_dir = hero_point - entity_point;

    // log(format!("- move_dir={:?}", move_dir));

    // shopkeepers always have horizontal shops with counters separating them from the patrons
    // they should move to try to stay accesible by the patron (directly above/below)
    let dx = move_dir.x.signum();
    let dy = if move_dir.y.abs() <= 2 {
        0
    } else {
        // if blocked => 0 else ...
        let map = world.read_resource::<Map>();

        match map.get_wrapped_index(entity_point.x + dx, entity_point.y + move_dir.y.signum()) {
            Some(index) => {
                if map.is_blocked(index) {
                    0
                } else {
                    move_dir.y.signum()
                }
            }
            None => return TaskResult::Success(100),
        }
    };

    // log(format!("- dx={}, dy={}", dx, dy));

    if dx == 0 && dy == 0 {
        // log("- no dx,dy");
        return TaskResult::Success(100);
    }

    // what about constantly trying to bump diagonally???
    // what about bumping into other shopkeepers???  Should we move randomly if there is a closer shopkeeper?

    // log(format!("- move: {},{}", dx, dy));

    do_entity_action(Box::new(MoveStepAction::new(entity, dx, dy)), world, entity)
}
