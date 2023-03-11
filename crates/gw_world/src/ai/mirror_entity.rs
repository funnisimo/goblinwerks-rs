use crate::action::{move_step::MoveStepAction, BoxedAction};
use crate::actor::Actor;
use crate::level::Level;
use crate::position::Position;
use gw_app::ecs::Entity;
use gw_util::point::Point;
use serde::{Deserialize, Serialize};

use super::AiHandler;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MirrorState {
    last_xy: Point,
    entity: Entity,
}

impl MirrorState {
    fn new(entity: Entity, point: &Point) -> Self {
        MirrorState {
            entity,
            last_xy: point.clone(),
        }
    }
}

pub struct MirrorEntity;

impl AiHandler for MirrorEntity {
    fn on_enter(&self, _level: &mut Level, _entity: Entity) -> () {
        // Nothing???
    }

    fn next_action(&self, level: &mut Level, entity: Entity) -> Option<BoxedAction> {
        let (last_point, mirror_entity) = {
            let mut entry = level.world.entry(entity).unwrap();
            match entry.get_component_mut::<MirrorState>() {
                Err(_) => {
                    let actor = entry.get_component_mut::<Actor>().unwrap();
                    actor.ai.pop();
                    return None;
                }
                Ok(state) => {
                    let last_xy = state.last_xy.clone();
                    (last_xy, state.entity)
                }
            }
        };

        let new_mirror_pos = {
            let mirror_entry = level.world.entry(mirror_entity).unwrap();
            mirror_entry.get_component::<Position>().unwrap().point()
        };

        let delta = new_mirror_pos - last_point;
        let unit = delta.as_dir();

        {
            let mut entry = level.world.entry(entity).unwrap();
            let mut state = entry.get_component_mut::<MirrorState>().unwrap();
            state.last_xy = new_mirror_pos;
        }

        // move in that direction
        Some(Box::new(MoveStepAction::new(entity, unit.x, unit.y)))
    }

    fn on_exit(&self, level: &mut Level, entity: Entity) -> () {
        // Pop State?
        let mut entry = level.world.entry(entity).unwrap();
        entry.remove_component::<MirrorState>();
    }
}

pub fn ai_mirror_entity(level: &mut Level, entity: Entity, mirror_entity: Entity) {
    let mirror_state = {
        let mirror_entry = level.world.entry(mirror_entity).unwrap();
        let pos = mirror_entry.get_component::<Position>().unwrap();
        MirrorState::new(mirror_entity, &pos.point())
    };

    let mut entry = level.world.entry(entity).unwrap();

    let actor = entry.get_component_mut::<Actor>().unwrap();
    actor.ai.push("MIRROR_ENTITY");
    entry.add_component(mirror_state);
}
