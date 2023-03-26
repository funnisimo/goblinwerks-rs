use crate::action::{Action, ActionResult};
use crate::level::get_current_level;
use gw_app::ecs::Entity;
use gw_app::Ecs;

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
    fn execute(&mut self, ecs: &mut Ecs) -> ActionResult {
        let level = get_current_level(ecs);
        match level.world.contains(self.entity) {
            false => {
                // TODO - log?  This is an action on a non-existant entity.
                ActionResult::Dead(self.entity)
            }
            true => ActionResult::WaitForInput,
        }
    }
}
