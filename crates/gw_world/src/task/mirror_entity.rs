use super::{execute_actor_action, get_hero_entity_point, TaskResult};
use crate::{
    action::move_step::MoveStepAction,
    being::Being,
    level::{get_current_level, get_current_level_mut, Level},
    position::Position,
};
use gw_app::{
    ecs::{systems::CommandBuffer, Entity, IntoQuery, Read, TryRead},
    Ecs,
};
use gw_util::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MirrorEntity {
    last_xy: Point,
    entity: Entity,
}

impl MirrorEntity {
    fn new(entity: Entity, point: Point) -> Self {
        MirrorEntity {
            entity,
            last_xy: point,
        }
    }
}

pub fn start_mirror_entity(
    level: &mut Level,
    entity: Entity,
    mirror_entity: Entity,
) -> Result<(), String> {
    let last_xy = match level.world.entry(mirror_entity) {
        None => return Err("No such mirror entity.".to_string()),
        Some(entry) => match entry.get_component::<Position>() {
            Err(_) => return Err("Entity does not have a position.".to_string()),
            Ok(pos) => pos.point(),
        },
    };

    match level.world.entry(entity) {
        None => return Err("No such entity.".to_string()),
        Some(mut entry) => entry.add_component(MirrorEntity::new(mirror_entity, last_xy)),
    }

    Ok(())
}

/*
let entity = command_buffer.push(());

command_buffer.add_component(entity, Position(123.0));
command_buffer.remove(entity);

command_buffer.flush(&mut world, &mut resources);
 */

pub fn mirror_entity_ai(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    let level = get_current_level(ecs);
    let mut command_buffer = CommandBuffer::new(&level.world);
    let mut query = <(Read<Being>, TryRead<MirrorEntity>, Read<Position>)>::query();

    let entity_info = query.get(&level.world, entity).unwrap();

    let mut me = match entity_info.1 {
        None => {
            let (hero_entity, hero_pt) = get_hero_entity_point(ecs);
            // We are not setup to mirror anyone, so lets mirror the hero
            if hero_entity == entity {
                // We are the hero though...  Lets fail...
                return TaskResult::Finished;
            }

            let me = MirrorEntity::new(hero_entity, hero_pt);
            command_buffer.add_component(entity, me.clone());
            me
        }
        Some(me) => me.clone(),
    };

    let mirror_info = match query.get(&level.world, me.entity) {
        Err(_) => {
            // Remove MirrorEntity
            return TaskResult::Finished;
        }
        Ok(info) => info,
    };

    let new_mirror_pt = mirror_info.2.point();

    let delta = new_mirror_pt - me.last_xy;
    let unit = delta.as_dir();

    me.last_xy = new_mirror_pt;

    // move in that direction
    let action = Box::new(MoveStepAction::new(entity, unit.x, unit.y));

    drop(mirror_info);
    drop(entity_info);
    drop(level);

    command_buffer.add_component(entity, me);
    let mut level = get_current_level_mut(ecs);
    let Level {
        world, resources, ..
    } = &mut *level;
    command_buffer.flush(world, resources);
    drop(level);

    return execute_actor_action(action, ecs, entity);
}
