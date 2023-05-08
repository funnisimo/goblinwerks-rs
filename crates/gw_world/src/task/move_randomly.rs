use super::{execute_actor_action, TaskResult};
use crate::{
    action::move_step::MoveStepAction,
    being::{Being, MoveFlags},
};
use gw_ecs::{Entity, World};
use gw_util::{point::DIRS, rng::RandomNumberGenerator};

pub fn move_randomly_ai(world: &mut World, entity: Entity) -> TaskResult {
    let beings = world.read_component::<Being>();
    let act_time = match beings.get(entity) {
        None => return TaskResult::Finished,
        Some(being) => being.act_time as u64,
    };

    let chance = match beings.get(entity) {
        None => 50,
        Some(being) => {
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

    let mut rng = world.write_resource::<RandomNumberGenerator>();

    if rng.chance(chance) {
        let index = rng.range(0, 4) as usize;
        let dir = DIRS[index];
        let action = Box::new(MoveStepAction::new(entity, dir.x, dir.y));
        drop(rng);
        drop(beings);

        return execute_actor_action(action, world, entity);
    }

    TaskResult::Success(act_time)
}
