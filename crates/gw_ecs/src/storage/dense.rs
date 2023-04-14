use crate::Component;
use crate::Entity;
use std::slice::{Iter, IterMut};

#[derive(Default)]
enum DenseEntry<T> {
    #[default]
    Empty,
    Used(Entity, T),
}

pub struct DenseStorage<T>
where
    T: Component,
{
    data: Vec<DenseEntry<T>>,
}

impl<T> DenseStorage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        DenseStorage { data: Vec::new() }
    }

    pub fn insert(&mut self, entity: Entity, val: T) {
        let d = &mut self.data;
        match d.get_mut(entity.index()) {
            None => d.insert(entity.index(), DenseEntry::Used(entity, val)),
            Some(entry) => {
                *entry = DenseEntry::Used(entity, val);
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let d = &mut self.data;
        match d.get_mut(entity.index()) {
            None => {}
            Some(entry) => {
                *entry = DenseEntry::Empty;
            }
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        match self.data.get(entity.index()) {
            None => false,
            Some(entry) => match entry {
                DenseEntry::Empty => false,
                DenseEntry::Used(k, _v) => entity.is_same_gen(k),
            },
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        match self.data.get(entity.index()) {
            None => None,
            Some(entry) => match entry {
                DenseEntry::Empty => None,
                DenseEntry::Used(k, v) => {
                    if entity.is_same_gen(k) {
                        Some(v)
                    } else {
                        None
                    }
                }
            },
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        match self.data.get_mut(entity.index()) {
            None => None,
            Some(entry) => match entry {
                DenseEntry::Empty => None,
                DenseEntry::Used(k, v) => {
                    if entity.is_same_gen(k) {
                        Some(v)
                    } else {
                        None
                    }
                }
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        DenseIter::new(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        DenseIterMut::new(self.data.iter_mut())
    }
}

struct DenseIter<'s, T>
where
    T: Component + 's,
{
    iter: Iter<'s, DenseEntry<T>>,
}

impl<'s, T> DenseIter<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: Iter<'s, DenseEntry<T>>) -> Self {
        DenseIter { iter }
    }
}

impl<'s, T> Iterator for DenseIter<'s, T>
where
    T: Component + 's,
{
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(entry) => match entry {
                    DenseEntry::Empty => {}
                    DenseEntry::Used(k, v) => {
                        if k.is_alive() {
                            return Some(v);
                        }
                    }
                },
            }
        }
    }
}

struct DenseIterMut<'s, T>
where
    T: Component + 's,
{
    iter: IterMut<'s, DenseEntry<T>>,
}

impl<'s, T> DenseIterMut<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: IterMut<'s, DenseEntry<T>>) -> Self {
        DenseIterMut { iter }
    }
}

impl<'s, T> Iterator for DenseIterMut<'s, T>
where
    T: Component + 's,
{
    type Item = &'s mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(entry) => match entry {
                    DenseEntry::Empty => {}
                    DenseEntry::Used(k, v) => {
                        if k.is_alive() {
                            return Some(v);
                        }
                    }
                },
            }
        }
    }
}
