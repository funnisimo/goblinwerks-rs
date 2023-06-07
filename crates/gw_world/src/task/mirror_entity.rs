use super::{execute_actor_action, get_hero_entity_point, TaskResult};
use crate::{action::move_step::MoveStepAction, position::Position};
use gw_ecs::prelude::{Component, Entity, ReadComp, World, WriteComp};
use gw_util::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Component, Serialize, Deserialize)]
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
    world: &mut World,
    entity: Entity,
    mirror_entity: Entity,
) -> Result<(), String> {
    let last_xy = match world.read_component::<Position>().get(mirror_entity) {
        None => return Err("Entity does not have a position.".to_string()),
        Some(pos) => pos.point(),
    };

    match world.entities().is_alive(entity) {
        false => return Err("No such entity.".to_string()),
        true => {
            world
                .write_component()
                .insert(entity, MirrorEntity::new(mirror_entity, last_xy));
        }
    }

    Ok(())
}

/*
let entity = command_buffer.push(());

command_buffer.add_component(entity, Position(123.0));
command_buffer.remove(entity);

command_buffer.flush(&mut world, &mut resources);
 */

pub fn mirror_entity_ai(world: &mut World, entity: Entity) -> TaskResult {
    let unit = {
        let (mut mirror, positions) = <(
            // ReadComp<Being>,
            WriteComp<MirrorEntity>,
            ReadComp<Position>,
            // ReadRes<LazyUpdate>,
        )>::fetch(world);

        if !mirror.contains(entity) {
            let (hero_entity, hero_pt) = get_hero_entity_point(world);
            // We are not setup to mirror anyone, so lets mirror the hero
            if hero_entity == entity {
                // We are the hero though...  Lets fail...
                return TaskResult::Finished;
            }

            let me = MirrorEntity::new(hero_entity, hero_pt);
            mirror.insert(entity, me);
        }

        let mut me = mirror.get_mut(entity).unwrap();

        // let _mirror_info = match beings.get(me.entity) {
        //     None => {
        //         // Remove MirrorEntity
        //         return TaskResult::Finished;
        //     }
        //     Some(info) => info,
        // };

        let new_mirror_pt = positions.get(me.entity).unwrap().point();

        let delta = new_mirror_pt - me.last_xy;
        let unit = delta.as_dir();

        me.last_xy = new_mirror_pt; // update in place
        unit
    };

    // move in that direction
    let action = Box::new(MoveStepAction::new(entity, unit.x, unit.y));
    return execute_actor_action(action, world, entity);
}
