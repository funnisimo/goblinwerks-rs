use crate::action::{Action, ActionResult};
use gw_ecs::prelude::{Entity, World};

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
    fn execute(&mut self, world: &mut World) -> ActionResult {
        match world.entities().is_alive(self.entity) {
            false => {
                // TODO - log?  This is an action on a non-existant entity.
                ActionResult::Dead(self.entity)
            }
            true => ActionResult::WaitForInput,
        }
    }
}
