use super::Level;

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

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn push(&mut self, level: Level) {
        self.cache.push(level);
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
}
