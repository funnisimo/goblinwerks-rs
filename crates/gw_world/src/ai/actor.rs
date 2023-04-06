use crate::action::BoxedAction;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::AI;

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub busy_time: u32,
    pub act_time: u32,

    #[serde(skip)] // Always move from level to level with no action - ai can add later
    pub next_action: Option<BoxedAction>,

    pub ai: AI,
}

impl Actor {
    pub fn new(ai: String) -> Self {
        Actor {
            busy_time: 0,
            act_time: 100,

            next_action: None,
            ai: AI::new(ai),
        }
    }
}

impl Clone for Actor {
    fn clone(&self) -> Self {
        let out = Actor {
            busy_time: self.busy_time,
            act_time: self.act_time,
            next_action: None,
            ai: self.ai.clone(),
        };
        println!("CLONE ACTOR - was: {:?}, clone: {:?}", self.ai, out.ai);
        out
    }
}

impl Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Actor");
        s.field("busy_time", &self.busy_time);
        s.field("act_time", &self.act_time);
        s.field("next_action", &self.next_action.is_some());
        s.field("ai", &self.ai);
        s.finish()
    }
}

impl Default for Actor {
    fn default() -> Self {
        Actor::new("IDLE".to_string())
    }
}
