use super::AI;
use crate::action::{Action, NeedInputAction};
use crate::prelude::*;
use crate::world::World;

#[derive(Clone, Copy, Debug)]
pub struct PlayerAI {}

impl PlayerAI {
    pub fn new() -> AI {
        AI::Player(PlayerAI {})
    }

    pub fn next_action(&mut self, entity: EntityId, _level: &World) -> Action {
        NeedInputAction::new(entity)
    }
}
