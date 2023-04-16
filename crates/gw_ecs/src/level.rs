use crate::{
    component::{Component, ComponentSet},
    entity::Entities,
    refcell::{AtomicRef, AtomicRefMut},
    resource::{Resource, Resources},
    storage::SparseSet,
    Entity,
};
// use downcast_rs::{impl_downcast, Downcast};

// pub trait Unique: 'static + Downcast {}
// impl<T> Unique for T where T: 'static {}
// impl_downcast!(Unique);

#[derive(Default)]
pub struct Level {
    pub(crate) index: usize,
    pub(crate) resources: Resources,
}

impl Level {
    pub fn new() -> Self {
        let mut res = Resources::default();
        res.insert(Entities::new());

        Level {
            index: 0,
            resources: res,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    // spawn
    pub fn spawn<'a, S: ComponentSet<'a>>(&mut self, comps: S) -> Entity {
        let entity = {
            let mut entities = self.get_unique_mut::<Entities>().unwrap();
            entities.spawn()
        };
        comps.spawn(self, entity);
        entity
    }

    // despawn

    pub fn insert_unique<U: Resource>(&mut self, unique: U) {
        self.resources.insert(unique);
    }

    pub fn get_unique<U: Resource>(&self) -> Option<AtomicRef<U>> {
        self.resources.get::<U>()
    }

    pub fn get_unique_mut<U: Resource>(&self) -> Option<AtomicRefMut<U>> {
        self.resources.get_mut::<U>()
    }

    // TODO - Used?
    pub fn register_component<C: Component>(&mut self) {
        if self.get_unique_mut::<SparseSet<C>>().is_some() {
            return;
        }
        let storage: SparseSet<C> = SparseSet::new();
        self.insert_unique(storage);
    }

    pub fn add_component<C: Component>(&self, entity: Entity, comp: C) {
        self.get_component_mut::<C>().unwrap().insert(entity, comp);
    }

    pub fn get_component<C: Component>(&self) -> Option<AtomicRef<SparseSet<C>>> {
        self.resources.get::<SparseSet<C>>()
    }

    pub fn get_component_mut<C: Component>(&self) -> Option<AtomicRefMut<SparseSet<C>>> {
        self.resources.get_mut::<SparseSet<C>>()
    }
}
