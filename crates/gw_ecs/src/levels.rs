use crate::level::Level;
use crate::refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use crate::Component;

type RegisterFn = dyn Fn(&mut Level) -> ();

pub struct Levels {
    levels: Vec<AtomicRefCell<Level>>,
    registry: Vec<Box<RegisterFn>>,
    current: usize,
}

impl Levels {
    pub fn new() -> Self {
        Levels {
            levels: vec![AtomicRefCell::new(Level::new())],
            current: 0,
            registry: Vec::new(),
        }
    }

    pub fn register_component<C: Component>(&mut self) {
        for level in self.levels.iter() {
            level.borrow_mut().register_component::<C>();
        }

        // TODO - prevent duplicates...
        self.registry
            .push(Box::new(|level| level.register_component::<C>()));
    }

    pub fn create(&mut self) -> AtomicRefMut<Level> {
        let mut level = Level::new();
        level.index = self.levels.len();
        for reg in self.registry.iter() {
            reg(&mut level);
        }
        self.levels.push(AtomicRefCell::new(level));
        self.get_mut(self.levels.len() - 1).unwrap()
    }

    pub fn select(&mut self, index: usize) {
        self.current = index;
    }

    pub fn current_index(&self) -> usize {
        self.current
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
