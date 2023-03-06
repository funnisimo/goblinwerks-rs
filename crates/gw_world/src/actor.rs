use crate::action::BoxedAction;
use crate::ai::idle::ai_idle;
use crate::ai::AiFn;

pub struct Actor {
    pub busy_time: u32,
    pub act_time: u32,
    pub next_action: Option<BoxedAction>,
    pub ai: AiFn,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            busy_time: 0,
            act_time: 100,
            next_action: None,
            ai: ai_idle,
        }
    }
}
