use crate::action::{Action, ActionResult};
use crate::level::Level;
use gw_app::ecs::Entity;

#[derive(Copy, Clone, Debug)]
pub struct NeedInputAction {
    pub entity: Entity,
}

impl NeedInputAction {
    pub fn new(entity: Entity) -> NeedInputAction {
        NeedInputAction { entity }
    }
}

impl Action for NeedInputAction {
    fn execute(&mut self, level: &mut Level) -> ActionResult {
        match level.world.contains(self.entity) {
            false => {
                // TODO - log?  This is an action on a non-existant entity.
                ActionResult::Dead(self.entity)
            }
            true => ActionResult::WaitForInput,
        }
    }
}
