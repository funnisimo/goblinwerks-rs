use crate::being::Being;
use gw_ecs::{Entity, World};

use super::TaskResult;

pub fn idle_ai(world: &mut World, entity: Entity) -> TaskResult {
    match world.read_component::<Being>().get(entity) {
        None => TaskResult::Finished,
        Some(being) => TaskResult::Success(being.act_time as u64),
    }
}
