use gw_app::ecs::{Resources, World};

pub struct Level {
    pub resources: Resources,
    pub world: World,
}

impl Level {
    pub fn new() -> Self {
        Level {
            resources: Resources::default(),
            world: World::default(),
        }
    }
}
