use crate::Component;
use crate::Entity;
use std::collections::{
    hash_map::{Iter, IterMut},
    HashMap,
};

struct SparseEntry<T>(Entity, T);

pub struct SparseStorage<T>
where
    T: Component,
{
    data: HashMap<usize, SparseEntry<T>>,
}

impl<T> SparseStorage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        SparseStorage {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, val: T) {
        let d = &mut self.data;
        d.insert(entity.index(), SparseEntry(entity, val));
    }

    pub fn remove(&mut self, entity: Entity) {
        let d = &mut self.data;
        d.remove(&entity.index());
    }

    pub fn contains(&self, entity: Entity) -> bool {
        match self.data.get(&entity.index()) {
            None => false,
            Some(entry) => entity.is_same_gen(&entry.0),
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        match self.data.get(&entity.index()) {
            None => None,
            Some(entry) => {
                if entity.is_same_gen(&entry.0) {
                    Some(&entry.1)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        match self.data.get_mut(&entity.index()) {
            None => None,
            Some(entry) => {
                if entity.is_same_gen(&entry.0) {
                    Some(&mut entry.1)
                } else {
                    None
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        SparseIter::new(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        SparseIterMut::new(self.data.iter_mut())
    }
}

struct SparseIter<'s, T>
where
    T: Component + 's,
{
    iter: Iter<'s, usize, SparseEntry<T>>,
}

impl<'s, T> SparseIter<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: Iter<'s, usize, SparseEntry<T>>) -> Self {
        SparseIter { iter }
    }
}

impl<'s, T> Iterator for SparseIter<'s, T>
where
    T: Component + 's,
{
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some((_k, v)) => {
                    if v.0.is_alive() {
                        return Some(&v.1);
                    }
                }
            }
        }
    }
}

struct SparseIterMut<'s, T>
where
    T: Component + 's,
{
    iter: IterMut<'s, usize, SparseEntry<T>>,
}

impl<'s, T> SparseIterMut<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: IterMut<'s, usize, SparseEntry<T>>) -> Self {
        SparseIterMut { iter }
    }
}

impl<'s, T> Iterator for SparseIterMut<'s, T>
where
    T: Component + 's,
{
    type Item = &'s mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some((_k, v)) => {
                    if v.0.is_alive() {
                        return Some(&mut v.1);
                    }
                }
            }
        }
    }
}
