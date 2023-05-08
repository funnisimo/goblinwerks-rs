use crate::action::{Action, ActionResult};
use gw_ecs::{Entity, World};

#[derive(Copy, Clone, Debug)]
pub struct DeadAction {
    pub entity: Entity,
}

impl DeadAction {
    pub fn new(entity: Entity) -> DeadAction {
        DeadAction { entity }
    }
}

impl Action for DeadAction {
    fn execute(&mut self, _world: &mut World) -> ActionResult {
        // TODO - Delete entity from ECS?
        ActionResult::Dead(self.entity)
    }
}
