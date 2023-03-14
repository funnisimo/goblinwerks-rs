//! Contains types related to defining shared resources which can be accessed inside systems.
//!
//! Use resources to share persistent data between systems or to provide a system with state
//! external to entities.

#![allow(dead_code)]

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use downcast_rs::{impl_downcast, Downcast};

/// Blanket trait for resource types.
pub trait Resource: 'static + Downcast {}
impl<T> Resource for T where T: 'static {}
impl_downcast!(Resource);

pub struct ResourceCell {
    data: AtomicRefCell<Box<dyn Resource>>,
}

impl ResourceCell {
    pub(super) fn new(resource: Box<dyn Resource>) -> Self {
        Self {
            data: AtomicRefCell::new(resource),
        }
    }

    pub(super) fn into_inner(self) -> Box<dyn Resource> {
        self.data.into_inner()
    }

    /// # Safety
    /// Types which are !Sync should only be retrieved on the thread which owns the resource
    /// collection.
    pub fn get<T: Resource>(&self) -> AtomicRef<T> {
        let borrow = self.data.borrow();
        AtomicRef::map(borrow, |inner| inner.downcast_ref::<T>().unwrap())
    }

    /// # Safety
    /// Types which are !Send should only be retrieved on the thread which owns the resource
    /// collection.
    pub fn get_mut<T: Resource>(&self) -> AtomicRefMut<T> {
        let borrow = self.data.borrow_mut(); // panics if this is borrowed already

        AtomicRefMut::map(borrow, |inner| inner.downcast_mut::<T>().unwrap())
    }
}
