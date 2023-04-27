use std::collections::VecDeque;

use gw_app::{ecs::Entity, ecs::EntityStore, ecs::IntoQuery, Ecs};
use gw_util::{
    grid::{random_point_matching, Grid},
    mask::get_area_mask,
    path::a_star_search,
    point::{Point, DIRS},
};
use gw_world::{
    action::move_step::MoveStepAction,
    being::{do_entity_action, Being, MoveFlags},
    level::{get_current_level_mut, Level},
    map::{ensure_area_grid, AreaGrid, Map},
    position::Position,
    task::TaskResult,
};

#[derive(Clone, Debug)]
pub struct AnchorPos(pub Point);

/// Try to wander around anchor point - which is usually the point that the actor was created at
pub fn anchored_wander(ecs: &mut Ecs, entity: Entity) -> TaskResult {
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
        return TaskResult::Success(100);
    }

    let mut entry = level.world.entry(entity).unwrap();

    let entity_point = match entry.get_component::<Position>() {
        Err(_) => {
            // log("- no entity point");
            return TaskResult::Success(100);
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

    drop(entry);

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

    drop(level);

    // Set action time to be 3-5 x act time so there is a delay before next action

    do_entity_action(
        Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
        ecs,
        entity,
    )
}

/// Just move randomly every now and then...
pub fn random_move(ecs: &mut Ecs, entity: Entity) -> TaskResult {
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
        return TaskResult::Success(100);
    }

    // Otherwise pick a random (of 8) direction to move
    let dir_index = level.rng.rand(8);
    let dir = match DIRS.get(dir_index as usize) {
        None => unreachable!("Not reachable"),
        Some(d) => d.clone(), // Error!  Not possible!
    };

    // Set action time to be 3-5 x act time so there is a delay before next action

    drop(level);

    do_entity_action(
        Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
        ecs,
        entity,
    )
}

pub struct FollowPath {
    path: VecDeque<Point>,
}

impl FollowPath {
    pub fn new(path: Vec<Point>) -> Self {
        FollowPath { path: path.into() }
    }
}

/// Just move randomly every now and then...
pub fn wander_horde(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    let mut level = get_current_level_mut(ecs);

    let mut query = <(&Position, Option<&mut FollowPath>)>::query();

    let (pos, follow_path) = match query.get_mut(&mut level.world, entity) {
        Err(_) => return TaskResult::Finished,
        Ok(d) => d,
    };

    let entity_pt = pos.point();

    // log(format!("RANDOM WANDER - {:?}", entity));
    match follow_path {
        None => {
            drop(level);
            init_wander(ecs, entity)
        }
        Some(follow) => {
            let next_pt = match follow.path.pop_front() {
                None => {
                    drop(level);
                    return init_wander(ecs, entity);
                }
                Some(step) => step,
            };

            let dir = (next_pt - entity_pt).as_dir();

            drop(query);
            drop(level);

            do_entity_action(
                Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
                ecs,
                entity,
            )
        }
    }
}

fn init_wander(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    ensure_area_grid(ecs);

    let mut level = get_current_level_mut(ecs);

    let Level {
        resources,
        world,
        rng,
        ..
    } = &mut *level;

    let area_grid = resources.get::<AreaGrid>().unwrap();

    let entry = world.entry(entity).unwrap();
    let start_point = entry.get_component::<Position>().unwrap().point();
    drop(entry);

    let area_id = area_grid.get(start_point.x, start_point.y).unwrap();

    let wander_goal = match random_point_matching(&area_grid.grid(), area_id, rng) {
        None => return TaskResult::Finished,
        Some(point) => point,
    };

    let path = {
        let map = resources.get::<Map>().unwrap();
        a_star_search(start_point, wander_goal, &*map, false)
    };

    match path {
        None => {
            // We did not find a path to our goal location - that means something is blocking the way.
            // We will pick again next time, so be idle for now.
        }
        Some(path) => {
            let wander = FollowPath::new(path);
            let mut entry = world.entry(entity).unwrap();
            entry.add_component(wander);
        }
    }

    // TODO - This number needs to come from somewhere -- Being, Horde, other Component, ...
    TaskResult::Success(100)
}
