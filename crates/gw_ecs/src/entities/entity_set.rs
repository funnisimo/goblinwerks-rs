use atomic_refcell::{AtomicRef, AtomicRefMut};

use super::{Entities, Entity, EntityId};

pub trait EntitySet<'a> {
    type Result: 'a;

    fn fetch(&self, entities: &'a Entities) -> Self::Result;
}

impl<'a> EntitySet<'a> for EntityId {
    type Result = AtomicRef<'a, Entity>;

    fn fetch(&self, entities: &'a Entities) -> Self::Result {
        entities.get(*self).unwrap()
    }
}

impl<'a> EntitySet<'a> for (EntityId,) {
    type Result = (AtomicRef<'a, Entity>,);

    fn fetch(&self, entities: &'a Entities) -> Self::Result {
        (entities.get(self.0).unwrap(),)
    }
}

impl<'a> EntitySet<'a> for (EntityId, EntityId) {
    type Result = (AtomicRef<'a, Entity>, AtomicRef<'a, Entity>);

    fn fetch(&self, entities: &'a Entities) -> Self::Result {
        (entities.get(self.0).unwrap(), entities.get(self.1).unwrap())
    }
}

/////////////////////////////////////////
/////////////////////////////////////////

pub trait EntityMutSet<'a> {
    type Result: 'a;

    fn fetch_mut(&self, entities: &'a mut Entities) -> Self::Result;
}

impl<'a> EntityMutSet<'a> for EntityId {
    type Result = AtomicRefMut<'a, Entity>;

    fn fetch_mut(&self, entities: &'a mut Entities) -> Self::Result {
        entities.get_mut(*self).unwrap()
    }
}

impl<'a> EntityMutSet<'a> for (EntityId,) {
    type Result = (AtomicRefMut<'a, Entity>,);

    fn fetch_mut(&self, entities: &'a mut Entities) -> Self::Result {
        (entities.get_mut(self.0).unwrap(),)
    }
}

impl<'a> EntityMutSet<'a> for (EntityId, EntityId) {
    type Result = (AtomicRefMut<'a, Entity>, AtomicRefMut<'a, Entity>);

    fn fetch_mut(&self, entities: &'a mut Entities) -> Self::Result {
        (
            entities.get_mut(self.0).unwrap(),
            entities.get_mut(self.1).unwrap(),
        )
    }
}
