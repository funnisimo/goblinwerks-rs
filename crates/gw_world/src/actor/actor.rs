use crate::action::BoxedAction;
use crate::ai::AI;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result};

#[derive(Serialize, Deserialize, Default)]
pub struct Actor {
    pub busy_time: u32,
    pub act_time: u32,

    pub(super) flavor: Option<String>,
    pub(super) description: Option<String>,

    #[serde(skip)] // Always move from level to level with no action - ai can add later
    pub next_action: Option<BoxedAction>,

    pub ai: AI,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            busy_time: 0,
            act_time: 100,
            next_action: None,
            ai: AI::new(),
            flavor: None,
            description: None,
        }
    }

    pub fn with_ai(mut self, ai: &str) -> Self {
        self.ai.push(ai);
        self
    }
}

impl Clone for Actor {
    fn clone(&self) -> Self {
        let mut out = Actor::new();
        out.busy_time = self.busy_time;
        out.act_time = self.act_time;
        out.ai = self.ai.clone();
        out.flavor = self.flavor.clone();
        out.description = self.description.clone();
        out
    }
}

impl Debug for Actor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = f.debug_struct("Actor");

        s.field("act_time", &self.act_time);
        s.field("busy_time", &self.busy_time);
        s.field("ai", &self.ai);
        s.field("flavor", &self.flavor);
        s.field("description", &self.description);
        s.field("next_action", &self.next_action.is_some());

        s.finish()
    }
}
