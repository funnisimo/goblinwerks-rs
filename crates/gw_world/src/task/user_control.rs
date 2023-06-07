use super::TaskResult;
use crate::action::{ActionResult, BoxedAction};
use gw_ecs::prelude::{Entity, World};

#[derive(Default)]
pub struct UserAction {
    action: Option<BoxedAction>,
}

impl UserAction {
    pub fn new(action: BoxedAction) -> Self {
        UserAction {
            action: Some(action),
        }
    }

    pub fn take(&mut self) -> Option<BoxedAction> {
        self.action.take()
    }

    pub fn set(&mut self, action: BoxedAction) {
        self.action.replace(action);
    }
}

pub fn user_control_ai(world: &mut World, entity: Entity) -> TaskResult {
    // let hero_entity = get_hero_entity(ecs);

    let action = world.write_resource::<UserAction>().take();
    match action {
        None => TaskResult::Retry,
        Some(user_action) => execute_actor_action(user_action, world, entity),
    }
}

pub fn execute_actor_action(action: BoxedAction, world: &mut World, _entity: Entity) -> TaskResult {
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
