use crate::log::Logger;
use gw_app::ecs::{Resources, World};

pub struct Level {
    pub id: String,
    pub resources: Resources,
    pub world: World,
    pub logger: Logger,
}

impl Level {
    pub fn new(id: &str) -> Self {
        Level {
            id: id.to_string(),
            resources: Resources::default(),
            world: World::default(),
            logger: Logger::new(),
        }
    }
}
