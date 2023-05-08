use crate::{
    action::{ActionResult, BoxedAction},
    task::TaskResult,
};
use gw_ecs::{Entity, World};

// #[must_use]
// fn get_next_action(entity: Entity, ecs: &mut World) -> BoxedAction {
//     let (ai_fn, idle_time) = {
//         let mut level = get_current_level_mut(ecs);
//         let mut entry = match level.world.entry(entity) {
//             None => return Box::new(DeadAction::new(entity)),
//             Some(entry) => entry,
//         };

//         let actor = entry.get_component_mut::<Being>().unwrap();
//         match actor.next_action.take() {
//             Some(action) => return action,
//             None => (actor.ai.current(), actor.act_time),
//         }
//     };

//     ai_fn(ecs, entity).unwrap_or(Box::new(IdleAction::new(entity, idle_time)))
// }

pub fn do_entity_action(action: BoxedAction, world: &mut World, _entity: Entity) -> TaskResult {
    let mut action = action;
    loop {
        match action.execute(world) {
            ActionResult::Dead(_) => {
                // no rescedule - entity dead
                return TaskResult::Finished;
            }
            ActionResult::Done(time) => {
                return TaskResult::Success(time as u64);
            }
            ActionResult::Fail(_msg) => {
                return TaskResult::Retry;
            }
            ActionResult::Replace(new_action) => {
                // do_debug!("{} - Replace result - {:?}", entity, new_action);
                action = new_action;
            }
            ActionResult::WaitForInput => {
                // debug_msg(format!("{} - Wait for input", entity));
                return TaskResult::Retry;
            }
            ActionResult::Retry => {
                return TaskResult::Retry;
            }
            ActionResult::PushMode(mode) => {
                return TaskResult::PushMode(mode);
            }
        }
    }
}
