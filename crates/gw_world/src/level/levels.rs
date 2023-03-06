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

    pub fn insert(&mut self, level: Level) {
        self.cache.push(level);
    }

    pub fn set_current(&mut self, id: usize) {
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
}
