use std::{fmt::Display, hash::Hash, slice::Iter};

const DEAD: u32 = 0x80000000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entity {
    id: u32,
    gen: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Entity { id, gen: 0 }
    }
    pub fn zero() -> Self {
        Entity { id: 0, gen: 0 }
    }
    pub fn dead() -> Self {
        Entity { id: 0, gen: DEAD }
    }

    pub(crate) fn kill(&mut self) {
        self.gen |= DEAD;
    }

    pub fn is_alive(&self) -> bool {
        (self.gen & DEAD) == 0
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub(crate) fn revive(&mut self) {
        if !self.is_alive() {
            self.gen &= !DEAD;
        }

        self.gen = self.gen.saturating_add(1);
        if !self.is_alive() {
            self.gen = 0;
        }
    }

    pub fn is_same_gen(&self, other: &Entity) -> bool {
        self.gen == other.gen
    }

    pub fn index(&self) -> usize {
        self.id.saturating_sub(1) as usize
    }
    pub fn gen(&self) -> u32 {
        self.gen
    }
}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.id, self.gen)
    }
}

pub struct Entities {
    data: Vec<Entity>,
}

impl Entities {
    pub fn new() -> Self {
        Entities { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn spawn(&mut self) -> Entity {
        for entry in self.data.iter_mut() {
            if entry.is_alive() {
                continue;
            }
            entry.revive();
            return entry.clone();
        }
        let new_entity = Entity::new(self.len() as u32 + 1);
        self.data.push(new_entity.clone());
        new_entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        if let Some(entry) = self.data.get_mut(entity.index()) {
            if entity.is_same_gen(entry) {
                entry.kill();
            }
        }
    }

    pub fn iter(&self) -> EntityIter<'_> {
        EntityIter::new(self.data.iter())
    }
}

pub struct EntityIter<'e> {
    iter: Iter<'e, Entity>,
}

impl<'e> EntityIter<'e> {
    pub fn new(iter: Iter<'e, Entity>) -> Self {
        EntityIter { iter }
    }
}

impl<'e> Iterator for EntityIter<'e> {
    type Item = &'e Entity;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(e) => {
                    if e.is_alive() {
                        return Some(e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn entity_basic() {
        let mut e = Entity::zero();
        let mut f = Entity::new(0);

        assert_eq!(e, f);
        assert_eq!(e.gen, 0x00000000);

        e.revive();
        assert_ne!(e, f);
        assert_eq!(e.gen, 0x00000001);

        f.revive();
        assert_eq!(e, f);

        e.kill();
        assert_ne!(e, f);
        assert!(!e.is_alive());
        assert_eq!(e.gen, 0x80000001);

        e.revive();
        assert!(e.is_alive());
        assert_eq!(e.gen, 0x00000002);
    }
}
