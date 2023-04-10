use super::{execute_actor_action, TaskResult};
use crate::{
    action::move_step::MoveStepAction,
    being::{Being, MoveFlags},
    level::get_current_level_mut,
};
use gw_app::{
    ecs::{Entity, EntityStore},
    Ecs,
};
use gw_util::point::DIRS;

pub fn move_randomly_ai(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    let mut level = get_current_level_mut(ecs);
    let entry = match level.world.entry_mut(entity) {
        Err(_) => return TaskResult::Finished,
        Ok(entry) => entry,
    };

    let act_time = match entry.get_component::<Being>() {
        Err(_) => return TaskResult::Finished,
        Ok(being) => being.act_time as u64,
    };

    let chance = match entry.get_component::<Being>() {
        Err(_) => 50,
        Ok(being) => {
            if being.move_flags.contains(MoveFlags::RAND100) {
                100
            } else {
                let mut chance = 0;
                if being.move_flags.contains(MoveFlags::RAND25) {
                    chance += 25;
                }
                if being.move_flags.contains(MoveFlags::RAND50) {
                    chance += 50;
                }
                if chance == 0 {
                    chance = 12;
                }
                chance
            }
        }
    };

    drop(entry);

    let rng = &mut level.rng;

    if rng.chance(chance) {
        let index = rng.range(0, 4) as usize;
        let dir = DIRS[index];
        let action = Box::new(MoveStepAction::new(entity, dir.x, dir.y));
        drop(level);

        return execute_actor_action(action, ecs, entity);
    }

    TaskResult::Success(act_time)
}
