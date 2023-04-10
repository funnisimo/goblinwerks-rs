use crate::{being::Being, level::get_current_level};
use gw_app::{
    ecs::{Entity, EntityStore},
    Ecs,
};

use super::TaskResult;

pub fn idle_ai(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    let level = get_current_level(ecs);
    match level.world.entry_ref(entity) {
        Err(_) => TaskResult::Finished,
        Ok(entry) => match entry.get_component::<Being>() {
            Err(_) => TaskResult::Finished,
            Ok(being) => TaskResult::Success(being.act_time as u64),
        },
    }
}
