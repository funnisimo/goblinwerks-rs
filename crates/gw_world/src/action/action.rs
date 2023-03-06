use gw_app::ecs::{Ecs, Entity};
use gw_app::screen::BoxedScreen;

pub enum ActionResult {
    Done(u32),            // Action done, queue entity in u64 time
    Replace(BoxedAction), // Action done, try this action instead
    Retry,                // Action not done, try this actor again (possibly changed the action)
    WaitForInput,         // Action not done, put back in scheduler
    PushMode(BoxedScreen),
    Fail(String), // Did not complete action, drop
    Dead(Entity), // Action resulted in entity being dead
}

pub trait Action: Send + Sync {
    fn execute(&mut self, ecs: &mut Ecs) -> ActionResult;
}

pub type BoxedAction = Box<dyn Action>;

pub type ActionFn = dyn FnMut(&mut Ecs) -> ActionResult + 'static + Send + Sync;
pub type BoxedActionFn = Box<ActionFn>;

impl<F> Action for F
where
    F: FnMut(&mut Ecs) -> ActionResult + 'static + Send + Sync,
{
    fn execute(&mut self, ecs: &mut Ecs) -> ActionResult {
        (self)(ecs)
    }
}

// pub struct Actions {
//     data: Vec<BoxedAction>,
// }

// impl Actions {
//     pub fn new() -> Self {
//         Actions { data: Vec::new() }
//     }

//     pub fn push<F>(&mut self, func: F)
//     where
//         F: Action + 'static,
//     {
//         self.data.push(Box::new(func));
//     }

//     pub fn execute(&mut self, ecs: &mut Ecs) {
//         for sys in self.data.iter_mut() {
//             sys.execute(ecs);
//         }
//     }
// }
