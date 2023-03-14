use super::Entity;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::slice::Iter;

pub struct EntityIter<'a> {
    iter: Iter<'a, AtomicRefCell<Entity>>,
}

impl<'a> EntityIter<'a> {
    pub(crate) fn new(iter: Iter<'a, AtomicRefCell<Entity>>) -> Self {
        EntityIter { iter }
    }
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = AtomicRef<'a, Entity>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(v) => Some(v.borrow()),
        }
    }
}

pub struct EntityIterMut<'a> {
    iter: Iter<'a, AtomicRefCell<Entity>>,
}

impl<'a> EntityIterMut<'a> {
    pub(crate) fn new(iter: Iter<'a, AtomicRefCell<Entity>>) -> Self {
        EntityIterMut { iter }
    }
}

impl<'a> Iterator for EntityIterMut<'a> {
    type Item = AtomicRefMut<'a, Entity>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(v) => Some(v.borrow_mut()),
        }
    }
}
