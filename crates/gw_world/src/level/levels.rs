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
#[derive(Default)]
pub struct Levels {
    cache: Vec<Level>,
    current: usize,
    start_map: Option<String>,
}

impl Levels {
    pub fn new() -> Self {
        Levels {
            cache: Vec::new(),
            current: 0,
            start_map: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn get_start_map(&self) -> Option<&String> {
        self.start_map.as_ref()
    }

    pub fn set_start_map(&mut self, id: &str) {
        self.start_map = Some(id.to_string());
    }

    pub fn insert(&mut self, level: Level) {
        if self.start_map.is_none() {
            self.start_map = Some(level.id.clone());
        }
        self.cache.push(level);
    }

    /// puts the given level into the given index, returns the old level
    /// panics if index is out of bounds
    pub fn replace(&mut self, level: Level) -> Option<Level> {
        if let Some(index) = self.index_of(&level.id) {
            Some(std::mem::replace(&mut self.cache[index], level))
        } else {
            None
        }
    }

    pub fn get(&self, id: &str) -> Option<&Level> {
        match self.index_of(id) {
            None => None,
            Some(index) => self.cache.get(index),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Level> {
        match self.index_of(id) {
            None => None,
            Some(index) => self.cache.get_mut(index),
        }
    }

    pub fn current_id(&self) -> &str {
        &self.cache[self.current].id
    }

    pub fn set_current(&mut self, id: &str) {
        match self.index_of(id) {
            None => panic!("Trying to activate invalid level - {}", id),
            Some(index) => {
                self.current = index;
                self.current_mut().set_needs_draw()
            }
        }
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

    fn index_of(&self, id: &str) -> Option<usize> {
        self.cache.iter().position(|level| level.id == id)
    }

    pub fn move_current_entity(&mut self, entity: Entity, to_id: &str) -> Entity {
        let to_index = match self.index_of(to_id) {
            None => panic!("Trying to move entity to unknown level - {}", to_id),
            Some(v) => v,
        };

        self.move_entity_from_to(entity, self.current, to_index)
    }

    fn move_entity_from_to(
        &mut self,
        entity: Entity,
        from_index: usize,
        to_index: usize,
    ) -> Entity {
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
