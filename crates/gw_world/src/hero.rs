use gw_ecs::Entity;

pub struct Hero {
    pub entity: Entity,
}

impl Hero {
    pub fn new(entity: Entity) -> Self {
        Hero { entity }
    }
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            entity: Entity::dead(),
        }
    }
}
