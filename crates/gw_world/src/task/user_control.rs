use super::TaskResult;
use crate::{
    action::{ActionResult, BoxedAction},
    level::get_current_level_mut,
};
use gw_app::{ecs::Entity, Ecs};

pub struct UserAction(BoxedAction);

impl UserAction {
    pub fn new(action: BoxedAction) -> Self {
        UserAction(action)
    }
}

pub fn user_control_ai(ecs: &mut Ecs, entity: Entity) -> TaskResult {
    // let hero_entity = get_hero_entity(ecs);

    let mut level = get_current_level_mut(ecs);

    match level.resources.remove::<UserAction>() {
        None => TaskResult::Retry,
        Some(user_action) => {
            drop(level);
            execute_actor_action(user_action.0, ecs, entity)
        }
    }
}

pub fn execute_actor_action(action: BoxedAction, ecs: &mut Ecs, _entity: Entity) -> TaskResult {
    let mut action = action;
    loop {
        match action.execute(ecs) {
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
