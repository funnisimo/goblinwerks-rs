use gw_app::ecs::Entity;

pub struct Hero {
    pub entity: Entity,
}

impl Hero {
    pub fn new(entity: Entity) -> Self {
        Hero { entity }
    }
}
