use super::AI;
use crate::action::{Action, DeadAction, IdleAction, MeleeAction, MoveStepAction, TalkAction};
// use crate::actor::AIFlags;
use crate::pathfinding::{a_star_search, EntitySource};
use crate::prelude::*;
use crate::world::World;

#[derive(Clone, Copy, Debug)]
pub struct BasicMonster {
    // flags: AIFlags,
}

impl BasicMonster {
    pub fn new() -> AI {
        AI::BasicMonster(BasicMonster {
            // flags: AIFlags::empty(),
        })
    }

    pub fn next_action(&mut self, entity: EntityId, world: &World) -> Action {
        let entity_pos = match world.entity_point(entity) {
            Some(point) => point,
            None => return DeadAction::new(entity),
        };

        let player = world.hero_entity();
        let can_see_player = match world.get_fov(player) {
            Some(fov) => fov.borrow().is_visible(entity_pos.x, entity_pos.y),
            None => false,
        };

        if !can_see_player {
            return IdleAction::new(entity);
        }

        world.set_update_sidebar(); // something is probably going to change

        let player_pos = world.hero_point();

        let entity_source = EntitySource::new(entity);
        match a_star_search(entity_pos, player_pos, entity_source, false) {
            Some(path) => {
                if path.len() > 1 {
                    let step = path[1] - path[0];
                    return MoveStepAction::new(entity, step.x, step.y);
                } else if path.len() == 1 {
                    // next to player
                    // return TalkAction::new(entity, "Tag, you are it");
                    return MeleeAction::new(entity, player);
                }
            }
            None => {}
        }

        TalkAction::new(entity, "I see you")
    }
}
