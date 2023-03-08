use crate::log::Logger;
use gw_app::ecs::{Resources, World};

pub struct Level {
    pub id: String,
    pub resources: Resources,
    pub world: World,
    pub logger: Logger,
    needs_draw: bool,
}

impl Level {
    pub fn new(id: &str) -> Self {
        Level {
            id: id.to_string(),
            resources: Resources::default(),
            world: World::default(),
            logger: Logger::new(),
            needs_draw: true,
        }
    }

    pub fn needs_draw(&self) -> bool {
        self.needs_draw
    }

    pub fn clear_needs_draw(&mut self) {
        self.needs_draw = false;
    }

    pub fn set_needs_draw(&mut self) {
        self.needs_draw = true;
    }
}
