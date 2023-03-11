use serde::{Deserialize, Serialize};

use crate::action::BoxedAction;
use crate::ai::AI;

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub busy_time: u32,
    pub act_time: u32,

    #[serde(skip)] // Always move from level to level with no action - ai can add later
    pub next_action: Option<BoxedAction>,

    pub ai: AI,
}

impl Actor {
    pub fn new(ai_name: &str) -> Self {
        let mut ai = AI::new();
        ai.push(ai_name);

        Actor {
            busy_time: 0,
            act_time: 100,
            next_action: None,
            ai,
        }
    }
}
