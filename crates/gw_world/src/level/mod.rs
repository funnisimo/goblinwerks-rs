// use crate::{log::Logger, task::Executor};
// use gw_ecs::prelude::World;
// use gw_util::rng::RandomNumberGenerator;

pub struct NeedsDraw {
    needs: bool,
}

impl NeedsDraw {
    pub fn new() -> Self {
        NeedsDraw { needs: true }
    }

    pub fn clear(&mut self) {
        self.needs = false;
    }

    pub fn set(&mut self) {
        self.needs = true;
    }

    pub fn needs_draw(&self) -> bool {
        self.needs
    }
}

impl Default for NeedsDraw {
    fn default() -> Self {
        NeedsDraw { needs: true }
    }
}
