use super::{Entity, EntityId, EntityIter, EntityIterMut};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

#[derive(Default)]
pub struct Entities {
    all: Vec<AtomicRefCell<Entity>>,
}

impl Entities {
    pub fn new() -> Self {
        Entities { all: Vec::new() }
    }

    pub fn create(&mut self) -> AtomicRefMut<Entity> {
        let index = self.all.len();

        if let Some(revive_idx) = self.all.iter().position(|entry| entry.borrow().is_dead()) {
            let mut entry = self.all[revive_idx].borrow_mut();
            entry.revive();
            return entry;
        }

        let new_entry = { AtomicRefCell::new(Entity::new(index)) };
        self.all.push(new_entry);
        self.all[index].borrow_mut()
    }

    pub fn get(&self, id: EntityId) -> Option<AtomicRef<Entity>> {
        match self.all.get(id.index as usize) {
            None => None,
            Some(entry) => {
                let entity = entry.borrow();
                if entity.id.generation != id.generation {
                    return None;
                }
                Some(entity)
            }
        }
    }

    pub fn get_mut(&self, id: EntityId) -> Option<AtomicRefMut<Entity>> {
        match self.all.get(id.index as usize) {
            None => None,
            Some(entry) => {
                let entity = entry.borrow_mut();
                if entity.id.generation != id.generation {
                    return None;
                }
                Some(entity)
            }
        }
    }

    pub fn kill(&mut self, id: EntityId) {
        if let Some(mut entity) = self.get_mut(id) {
            entity.kill();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = AtomicRef<Entity>> {
        EntityIter::new(self.all.iter())
    }

    pub fn iter_mut(&self) -> impl Iterator<Item = AtomicRefMut<Entity>> {
        EntityIterMut::new(self.all.iter())
    }

    pub fn fetch_mut(&self, ids: &[EntityId]) -> Vec<Option<AtomicRefMut<Entity>>> {
        ids.iter().map(|id| self.get_mut(*id)).collect()
    }
}
