use crate::{
    refcell::{AtomicRef, AtomicRefMut},
    resource::Resources,
};
use downcast_rs::{impl_downcast, Downcast};

pub trait Unique: 'static + Downcast {}
impl<T> Unique for T where T: 'static {}
impl_downcast!(Unique);

#[derive(Default)]
pub struct Level {
    pub(crate) index: usize,
    pub(crate) resources: Resources,
}

impl Level {
    pub fn new() -> Self {
        Level {
            index: 0,
            resources: Resources::default(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn insert_unique<U: Unique>(&mut self, unique: U) {
        self.resources.insert(unique);
    }

    pub fn unique<U: Unique>(&self) -> Option<AtomicRef<U>> {
        self.resources.get::<U>()
    }

    pub fn unique_mut<U: Unique>(&self) -> Option<AtomicRefMut<U>> {
        self.resources.get_mut::<U>()
    }
}
