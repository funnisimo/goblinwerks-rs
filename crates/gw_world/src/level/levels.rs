use super::Level;
use gw_app::ecs::{Component, Deserialize, Entity, Registry, Serialize};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTRY: Mutex<Registry<String>> = Mutex::new(Registry::new());
}

pub fn register_component<C>(name: &str)
where
    for<'d> C: Component + Serialize + Deserialize<'d>,
{
    if let Ok(mut registry) = REGISTRY.lock() {
        registry.register::<C>(name.to_string());
    }
}

// TODO - Make this id:&str based instead of index:usize based
//      - allows for replace/remove semantics

pub struct Levels {
    cache: Vec<Level>,
    current: usize,
}

impl Levels {
    pub fn new() -> Self {
        Levels {
            cache: Vec::new(),
            current: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn push(&mut self, level: Level) -> usize {
        self.cache.push(level);
        self.cache.len() - 1
    }

    /// puts the given level into the given index, returns the old level
    /// panics if index is out of bounds
    pub fn replace(&mut self, index: usize, level: Level) -> Level {
        std::mem::replace(&mut self.cache[index], level)
    }

    pub fn get(&self, index: usize) -> Option<&Level> {
        self.cache.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Level> {
        self.cache.get_mut(index)
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn set_current_index(&mut self, id: usize) {
        if id >= self.cache.len() {
            panic!(
                "Trying to activate invalid level - {} (out of {})",
                id,
                self.cache.len()
            );
        }

        self.current = id;
        self.current_mut().set_needs_draw()
    }

    pub fn current(&self) -> &Level {
        &self.cache[self.current]
    }

    pub fn current_mut(&mut self) -> &mut Level {
        &mut self.cache[self.current]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Level> {
        self.cache.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Level> {
        self.cache.iter_mut()
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.cache.iter().position(|level| level.id == id)
    }

    pub fn move_current_entity(&mut self, entity: Entity, to_index: usize) -> Entity {
        self.move_entity(entity, self.current, to_index)
    }

    pub fn move_entity(&mut self, entity: Entity, from_index: usize, to_index: usize) -> Entity {
        if from_index == to_index {
            return entity;
        }

        if to_index >= self.len() || from_index >= self.len() {
            panic!("Invalid to_index");
        }

        let (source, dest) = if from_index < to_index {
            let (a, b) = self.cache.split_at_mut(to_index);
            (&mut a[from_index], &mut b[0])
        } else {
            let (a, b) = self.cache.split_at_mut(from_index);
            (&mut b[0], &mut a[to_index])
        };

        super::move_entity(entity, source, dest)
    }
}
