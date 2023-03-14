use crate::resources::Resources;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use downcast_rs::{impl_downcast, Downcast};

use super::ChangeFlags;

/// Blanket trait for resource types.
pub trait Component: 'static + Downcast {}
impl<T> Component for T where T: 'static {}
impl_downcast!(Component);

#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    pub(crate) index: u32,
    pub(crate) generation: u32,
}

impl EntityId {
    pub(crate) fn new(index: usize) -> Self {
        EntityId {
            index: index as u32,
            generation: 0,
        }
    }

    pub(crate) fn next_generation(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }
}

pub struct Entity {
    pub(crate) id: EntityId,
    components: Resources,
    flags: ChangeFlags,
}

impl Entity {
    pub(crate) fn new(index: usize) -> Self {
        Entity {
            id: EntityId::new(index),
            components: Resources::default(),
            flags: ChangeFlags::ADDED,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn index(&self) -> usize {
        self.id.index as usize
    }

    pub fn is_dead(&self) -> bool {
        self.flags.contains(ChangeFlags::DEAD)
    }

    pub fn revive(&mut self) {
        self.flags = ChangeFlags::ADDED;
        self.id.next_generation();
        self.components = Resources::default();
    }

    pub fn kill(&mut self) {
        self.flags |= ChangeFlags::DELETED | ChangeFlags::DEAD;
    }

    pub fn get<C>(&self) -> Option<AtomicRef<C>>
    where
        C: Component,
    {
        self.components.get::<C>()
    }

    pub fn get_mut<C>(&mut self) -> Option<AtomicRefMut<C>>
    where
        C: Component,
    {
        self.flags.insert(ChangeFlags::CHANGED);
        self.components.get_mut::<C>()
    }

    pub fn insert<C>(&mut self, comp: C)
    where
        C: Component,
    {
        self.components.insert(comp);
        self.flags.insert(ChangeFlags::CHANGED);
    }

    pub fn remove<C>(&mut self) -> Option<C>
    where
        C: Component,
    {
        self.flags.insert(ChangeFlags::CHANGED);
        self.components.remove::<C>()
    }
}
