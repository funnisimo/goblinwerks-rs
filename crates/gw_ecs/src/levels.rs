use crate::level::Level;
use crate::refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

#[derive(Default)]
pub struct Levels {
    levels: Vec<AtomicRefCell<Level>>,
    current: usize,
}

impl Levels {
    pub fn new() -> Self {
        Levels {
            levels: Vec::new(),
            current: 0,
        }
    }

    pub fn insert(&mut self, level: Level) {
        let mut level = level;
        level.index = self.levels.len();
        self.levels.push(AtomicRefCell::new(level));
    }

    pub fn select(&mut self, index: usize) {
        self.current = index;
    }

    pub fn current(&self) -> AtomicRef<Level> {
        self.levels
            .get(self.current)
            .expect("No current level!")
            .borrow()
    }

    pub fn current_mut(&self) -> AtomicRefMut<Level> {
        self.levels
            .get(self.current)
            .expect("No current level!")
            .borrow_mut()
    }

    pub fn get(&self, index: usize) -> Option<AtomicRef<Level>> {
        self.levels.get(index).map(|v| v.borrow())
    }

    pub fn get_mut(&self, index: usize) -> Option<AtomicRefMut<Level>> {
        self.levels.get(index).map(|v| v.borrow_mut())
    }
}
