use gw_ecs::{
    specs::Join, Component, DenseVecStorage, Entities, Entity, ReadComp, SystemData, World,
    WriteComp,
};
use gw_util::{
    grid::random_point_matching,
    path::a_star_search,
    point::{Point, DIRS},
    rng::RandomNumberGenerator,
};
use gw_world::{
    action::move_step::MoveStepAction,
    being::{do_entity_action, Being, MoveFlags},
    map::{ensure_area_grid, AreaGrid, Map},
    position::Position,
    task::TaskResult,
};
use std::collections::VecDeque;

#[derive(Clone, Debug, Component)]
pub struct AnchorPos(pub Point);

/// Try to wander around anchor point - which is usually the point that the actor was created at
pub fn anchored_wander(world: &mut World, entity: Entity) -> TaskResult {
    // log(format!("ANCHORED WANDER - {:?}", entity));

    let chance = {
        let beings = world.read_component::<Being>();
        let actor = beings.get(entity).unwrap();
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

    if world
        .write_resource::<RandomNumberGenerator>()
        .chance(100 - chance)
    {
        return TaskResult::Success(100);
    }

    let entity_point = match world.read_component::<Position>().get(entity) {
        None => {
            // log("- no entity point");
            return TaskResult::Success(100);
        }
        Some(pos) => pos.point(),
    };

    let anchor_point = {
        let mut anchor = world.write_component::<AnchorPos>();
        match anchor.get(entity) {
            Some(anchor) => anchor.0.clone(),
            None => {
                anchor
                    .insert(entity, AnchorPos(entity_point.clone()))
                    .unwrap();
                entity_point.clone()
            }
        }
    };

    // log(format!("- entity_point={:?}", entity_point));
    // log(format!("- anchor_point={:?}", anchor_point));

    let dir = if world
        .write_resource::<RandomNumberGenerator>()
        .chance_in(1, 4)
    {
        // 25% chance - move towards anchor
        (anchor_point - entity_point).as_dir()
    } else {
        // Otherwise pick a random (of 8) direction to move
        let dir_index = world.write_resource::<RandomNumberGenerator>().rand(8);
        match DIRS.get(dir_index as usize) {
            None => unreachable!("Not reachable"),
            Some(d) => d.clone(), // Error!  Not possible!
        }
    };

    // Set action time to be 3-5 x act time so there is a delay before next action

    do_entity_action(
        Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
        world,
        entity,
    )
}

/// Just move randomly every now and then...
pub fn random_move(world: &mut World, entity: Entity) -> TaskResult {
    // log(format!("RANDOM WANDER - {:?}", entity));

    let chance = {
        let beings = world.read_component::<Being>();
        let actor = beings.get(entity).unwrap();
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

    if world
        .write_resource::<RandomNumberGenerator>()
        .chance(100 - chance)
    {
        return TaskResult::Success(100);
    }

    // Otherwise pick a random (of 8) direction to move
    let dir_index = world.write_resource::<RandomNumberGenerator>().rand(8);
    let dir = match DIRS.get(dir_index as usize) {
        None => unreachable!("Not reachable"),
        Some(d) => d.clone(), // Error!  Not possible!
    };

    // Set action time to be 3-5 x act time so there is a delay before next action

    do_entity_action(
        Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
        world,
        entity,
    )
}

#[derive(Default, Component)]
pub struct FollowPath {
    path: VecDeque<Point>,
}

impl FollowPath {
    pub fn new(path: Vec<Point>) -> Self {
        FollowPath { path: path.into() }
    }
}

/// Just move randomly every now and then...
pub fn wander_horde(world: &mut World, entity: Entity) -> TaskResult {
    let (pos, next_step) = {
        let (entities, positions, mut follow_path) =
            <(Entities, ReadComp<Position>, WriteComp<FollowPath>)>::fetch(world);
        let mut query = (&positions, (&mut follow_path).maybe()).join();
        match query.get(entity, &entities) {
            None => return TaskResult::Finished,
            Some((p, None)) => (p.clone(), None),
            Some((p, Some(follow))) => (p.clone(), follow.path.pop_front()),
        }
    };

    let entity_pt = pos.point();

    // log(format!("RANDOM WANDER - {:?}", entity));
    match next_step {
        None => init_wander(world, entity),
        Some(next_pt) => {
            let dir = (next_pt - entity_pt).as_dir();

            do_entity_action(
                Box::new(MoveStepAction::new(entity, dir.x, dir.y)),
                world,
                entity,
            )
        }
    }
}

fn init_wander(world: &mut World, entity: Entity) -> TaskResult {
    ensure_area_grid(world);

    let area_grid = world.read_resource::<AreaGrid>();

    let start_point = world
        .read_component::<Position>()
        .get(entity)
        .unwrap()
        .point();

    let area_id = area_grid.get(start_point.x, start_point.y).unwrap();

    let wander_goal = match random_point_matching(
        &area_grid.grid(),
        area_id,
        &mut world.write_resource::<RandomNumberGenerator>(),
    ) {
        None => return TaskResult::Finished,
        Some(point) => point,
    };

    let path = {
        let map = world.read_resource::<Map>();
        a_star_search(start_point, wander_goal, &*map, false)
    };

    match path {
        None => {
            // We did not find a path to our goal location - that means something is blocking the way.
            // We will pick again next time, so be idle for now.
        }
        Some(path) => {
            let wander = FollowPath::new(path);
            world.write_component().insert(entity, wander);
        }
    }

    // TODO - This number needs to come from somewhere -- Being, Horde, other Component, ...
    TaskResult::Success(100)
}
