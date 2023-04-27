use crate::Component;
use crate::Entity;
use std::slice::{Iter, IterMut};

pub(crate) enum SparseEntry<T> {
    Empty,
    Used(Entity, T),
}

pub struct SparseSet<T>
where
    T: Component,
{
    sparse: Vec<usize>,
    dense: Vec<SparseEntry<T>>,
}

impl<T> SparseSet<T>
where
    T: Component,
{
    pub fn new() -> Self {
        SparseSet {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, val: T) {
        let index = {
            match self.dense.iter().position(|e| match e {
                SparseEntry::Empty => true,
                _ => false,
            }) {
                None => {
                    self.dense.push(SparseEntry::Empty);
                    self.dense.len() - 1
                }
                Some(index) => index,
            }
        };

        if self.sparse.len() <= entity.index() {
            self.sparse.resize(entity.index() + 1, 0);
        }
        self.sparse[entity.index()] = index;
        self.dense[index] = SparseEntry::Used(entity, val);
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(index) = self.sparse.get(entity.index()) {
            if let Some(entry) = self.dense.get(*index) {
                match entry {
                    SparseEntry::Empty => {}
                    SparseEntry::Used(ent, _) => {
                        if *ent == entity {
                            self.dense[*index] = SparseEntry::Empty;
                        }
                    }
                }
            }
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        match self.sparse.get(entity.index()) {
            None => false,
            Some(index) => match self.dense.get(*index) {
                Some(SparseEntry::Used(ent, _)) => *ent == entity,
                _ => false,
            },
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        match self.sparse.get(entity.index()) {
            None => None,
            Some(index) => match self.dense.get(*index) {
                Some(SparseEntry::Used(ent, val)) if *ent == entity => Some(val),
                _ => None,
            },
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        match self.sparse.get(entity.index()) {
            None => None,
            Some(index) => match self.dense.get_mut(*index) {
                Some(SparseEntry::Used(ent, val)) if *ent == entity => Some(val),
                _ => None,
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        SparseSetIter::new(self.dense.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        SparseSetIterMut::new(self.dense.iter_mut())
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        SparseSetEntities::new(self.dense.iter())
    }

    pub(crate) fn as_slice(&self) -> &[SparseEntry<T>] {
        self.dense.as_slice()
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [SparseEntry<T>] {
        self.dense.as_mut_slice()
    }
}

struct SparseSetIter<'s, T>
where
    T: Component + 's,
{
    iter: Iter<'s, SparseEntry<T>>,
}

impl<'s, T> SparseSetIter<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: Iter<'s, SparseEntry<T>>) -> Self {
        SparseSetIter { iter }
    }
}

impl<'s, T> Iterator for SparseSetIter<'s, T>
where
    T: Component + 's,
{
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(SparseEntry::Empty) => {}
                Some(SparseEntry::Used(ent, val)) => {
                    if ent.is_alive() {
                        return Some(val);
                    }
                }
            }
        }
    }
}

struct SparseSetIterMut<'s, T>
where
    T: Component + 's,
{
    iter: IterMut<'s, SparseEntry<T>>,
}

impl<'s, T> SparseSetIterMut<'s, T>
where
    T: Component + 's,
{
    pub fn new(iter: IterMut<'s, SparseEntry<T>>) -> Self {
        SparseSetIterMut { iter }
    }
}

impl<'s, T> Iterator for SparseSetIterMut<'s, T>
where
    T: Component + 's,
{
    type Item = &'s mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(SparseEntry::Empty) => {}
                Some(SparseEntry::Used(ent, val)) => {
                    if ent.is_alive() {
                        return Some(val);
                    }
                }
            }
        }
    }
}

struct SparseSetEntities<'s, T> {
    iter: Iter<'s, SparseEntry<T>>,
}

impl<'s, T> SparseSetEntities<'s, T> {
    pub fn new(iter: Iter<'s, SparseEntry<T>>) -> Self {
        SparseSetEntities { iter }
    }
}

impl<'s, T> Iterator for SparseSetEntities<'s, T> {
    type Item = &'s Entity;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(SparseEntry::Empty) => {}
                Some(SparseEntry::Used(ent, val)) => {
                    if ent.is_alive() {
                        return Some(ent);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Entity;

    #[test]
    fn basics() {
        let mut ss: SparseSet<u32> = SparseSet::new();

        let mut ent = Entity::new(1);

        assert_eq!(ss.get(ent), None);

        ss.insert(ent, 4);
        assert_eq!(ss.get(ent), Some(&4));

        ent.revive(); // increment generation
        assert_eq!(ss.get(ent), None);

        ss.insert(ent, 5);
        assert_eq!(ss.get(ent), Some(&5));

        ss.insert(ent, 6);
        assert_eq!(ss.get(ent), Some(&6));

        {
            let mut_val = ss.get_mut(ent).unwrap();
            *mut_val = 10;
        }

        assert_eq!(ss.get(ent), Some(&10));
    }

    #[test]
    fn iter() {
        let mut ss: SparseSet<u32> = SparseSet::new();

        for i in 1..=10 {
            let ent = Entity::new(i);
            ss.insert(ent, i);
        }

        assert_eq!(ss.iter().count(), 10);
        assert_eq!(ss.iter_mut().count(), 10);

        assert_eq!(ss.iter().next().unwrap(), &1);

        let ent = Entity::new(4);
        ss.remove(ent);

        assert_eq!(ss.iter().count(), 9);
        assert_eq!(ss.iter_mut().count(), 9);

        assert_eq!(ss.get(ent), None);
    }
}
