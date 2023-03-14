use atomic_refcell::{AtomicRef, AtomicRefMut};

use crate::{
    entities::{Entities, Entity, EntityId},
    resources::{Resource, Resources},
};

pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        World {
            resources: Resources::default(),
            entities: Entities::default(),
        }
    }

    pub fn insert<R: Resource>(&mut self, res: R) {
        self.resources.insert(res)
    }

    pub fn get<R: Resource>(&self) -> Option<AtomicRef<R>> {
        self.resources.get::<R>()
    }

    pub fn get_mut<R: Resource>(&mut self) -> Option<AtomicRefMut<R>> {
        self.resources.get_mut::<R>()
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    pub fn spawn_entity(&mut self) -> AtomicRefMut<Entity> {
        self.entities.create()
    }

    pub fn get_entity(&self, id: EntityId) -> Option<AtomicRef<Entity>> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&self, id: EntityId) -> Option<AtomicRefMut<Entity>> {
        self.entities.get_mut(id)
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        self.entities.kill(id)
    }
}
