use gw_app::{ecs::Entity, log, Ecs};
use gw_util::point::{Point, DIRS};
use gw_world::{
    action::{move_step::MoveStepAction, BoxedAction},
    being::{Being, MoveFlags},
    level::get_current_level_mut,
    position::Position,
};

#[derive(Clone, Debug)]
pub struct AnchorPos(pub Point);

/// Try to wander around anchor point - which is usually the point that the actor was created at
pub fn anchored_wander(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let mut level = get_current_level_mut(ecs);

    // log(format!("ANCHORED WANDER - {:?}", entity));

    let chance = {
        let entry = level.world.entry(entity).unwrap();
        let actor = entry.get_component::<Being>().unwrap();
        if actor.move_flags.contains(MoveFlags::RAND100) {
            100
        } else {
            let mut chance = 0;
            chance += match actor.move_flags.contains(MoveFlags::RAND25) {
                true => 25,
                false => 0,
            };
            chance += match actor.move_flags.contains(MoveFlags::RAND50) {
                true => 50,
                false => 0,
            };

            if chance == 0 {
                chance = 12;
            }
            chance
        }
    };

    if level.rng.chance(100 - chance) {
        return None;
    }

    let mut entry = level.world.entry(entity).unwrap();

    let entity_point = match entry.get_component::<Position>() {
        Err(_) => {
            // log("- no entity point");
            return None;
        }
        Ok(pos) => pos.point(),
    };

    let anchor_point = match entry.get_component::<AnchorPos>() {
        Ok(anchor) => anchor.0.clone(),
        Err(_) => {
            entry.add_component(AnchorPos(entity_point.clone()));
            entity_point.clone()
        }
    };

    // log(format!("- entity_point={:?}", entity_point));
    // log(format!("- anchor_point={:?}", anchor_point));

    let dir = if level.rng.chance_in(1, 4) {
        // 25% chance - move towards anchor
        (anchor_point - entity_point).as_dir()
    } else {
        // Otherwise pick a random (of 8) direction to move
        let dir_index = level.rng.rand(8);
        match DIRS.get(dir_index as usize) {
            None => unreachable!("Not reachable"),
            Some(d) => d.clone(), // Error!  Not possible!
        }
    };

    // Set action time to be 3-5 x act time so there is a delay before next action

    Some(Box::new(MoveStepAction::new(entity, dir.x, dir.y)))
}

/// Just move randomly every now and then...
pub fn random_wander(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let mut level = get_current_level_mut(ecs);

    // log(format!("RANDOM WANDER - {:?}", entity));

    let chance = {
        let entry = level.world.entry(entity).unwrap();
        let actor = entry.get_component::<Being>().unwrap();
        if actor.move_flags.contains(MoveFlags::RAND100) {
            100
        } else {
            let mut chance = 0;
            chance += match actor.move_flags.contains(MoveFlags::RAND25) {
                true => 25,
                false => 0,
            };
            chance += match actor.move_flags.contains(MoveFlags::RAND50) {
                true => 50,
                false => 0,
            };

            if chance == 0 {
                chance = 12;
            }
            chance
        }
    };

    if level.rng.chance(100 - chance) {
        return None;
    }

    // Otherwise pick a random (of 8) direction to move
    let dir_index = level.rng.rand(8);
    let dir = match DIRS.get(dir_index as usize) {
        None => unreachable!("Not reachable"),
        Some(d) => d.clone(), // Error!  Not possible!
    };

    // Set action time to be 3-5 x act time so there is a delay before next action

    Some(Box::new(MoveStepAction::new(entity, dir.x, dir.y)))
}
