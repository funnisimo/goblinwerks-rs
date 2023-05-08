use crate::action::{Action, ActionResult};
use gw_ecs::{Entity, World};

#[derive(Copy, Clone, Debug)]
pub struct IdleAction {
    pub entity: Entity,
    pub time: u32,
}

impl IdleAction {
    pub fn new(entity: Entity, time: u32) -> IdleAction {
        IdleAction { entity, time }
    }
}

impl Action for IdleAction {
    fn execute(&mut self, world: &mut World) -> ActionResult {
        match world.entities().is_alive(self.entity) {
            false => {
                // TODO - log?  This is an action on a non-existant entity.
                ActionResult::Dead(self.entity)
            }
            true => ActionResult::Done(self.time),
        }
    }
}
