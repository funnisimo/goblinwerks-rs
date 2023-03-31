use gw_app::{ecs::Entity, log, Ecs};
use gw_world::{
    action::{move_step::MoveStepAction, BoxedAction},
    hero::Hero,
    level::get_current_level_mut,
    map::Map,
    position::Position,
};

/// Try to move toward the hero - will be stopped by the counters.
pub fn shopkeeper(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let mut level = get_current_level_mut(ecs);

    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    log(format!("SHOPKEEPER - {:?}", entity));

    let hero_point = match level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
    {
        Err(_) => {
            log("- no hero_point");
            return None;
        }
        Ok(pos) => pos.point(),
    };

    let entity_point = match level
        .world
        .entry(entity)
        .unwrap()
        .get_component::<Position>()
    {
        Err(_) => {
            log("- no entity point");
            return None;
        }
        Ok(pos) => pos.point(),
    };

    log(format!("- entity_point={:?}", entity_point));
    log(format!("- hero_point={:?}", hero_point));

    let move_dir = hero_point - entity_point;

    log(format!("- move_dir={:?}", move_dir));

    // shopkeepers always have horizontal shops with counters separating them from the patrons
    // they should move to try to stay accesible by the patron (directly above/below)
    let dx = move_dir.x.signum();
    let dy = if move_dir.y.abs() <= 2 {
        0
    } else {
        // if blocked => 0 else ...
        let map = level.resources.get::<Map>().unwrap();

        match map.get_wrapped_index(entity_point.x + dx, entity_point.y + move_dir.y.signum()) {
            Some(index) => {
                if map.is_blocked(index) {
                    0
                } else {
                    move_dir.y.signum()
                }
            }
            None => return None,
        }
    };

    log(format!("- dx={}, dy={}", dx, dy));

    if dx == 0 && dy == 0 {
        log("- no dx,dy");
        return None;
    }

    // what about constantly trying to bump diagonally???
    // what about bumping into other shopkeepers???  Should we move randomly if there is a closer shopkeeper?

    log(format!("- move: {},{}", dx, dy));

    Some(Box::new(MoveStepAction::new(entity, dx, dy)))
}
