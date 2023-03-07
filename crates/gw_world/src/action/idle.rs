use crate::action::{Action, ActionResult};
use crate::level::Level;
use gw_app::ecs::Entity;

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
    fn execute(&mut self, level: &mut Level) -> ActionResult {
        match level.world.contains(self.entity) {
            false => {
                // TODO - log?  This is an action on a non-existant entity.
                ActionResult::Dead(self.entity)
            }
            true => ActionResult::Done(self.time),
        }
    }
}
