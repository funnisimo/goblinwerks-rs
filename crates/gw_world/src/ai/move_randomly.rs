use super::AI;
use crate::action::{Action, IdleAction, MoveStepAction};
use crate::prelude::*;
use crate::utils::DIRS;
use crate::world::World;

#[derive(Clone, Copy, Debug)]
pub struct MoveRandomly {
    chance: u32,
}

impl MoveRandomly {
    pub fn new(chance: u32) -> AI {
        AI::MoveRandomly(MoveRandomly { chance })
    }

    pub fn next_action(&mut self, entity: EntityId, level: &World) -> Action {
        let mut rng = level.rng().borrow_mut();

        if rng.range(0, 100) >= self.chance as i32 {
            return IdleAction::new(entity);
        }

        let index = rng.range(0, 4) as usize;
        let dir = DIRS[index];
        MoveStepAction::new(entity, dir.x, dir.y)
    }
}
