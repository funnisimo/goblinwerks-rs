use super::Component;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

#[doc(hidden)]
pub struct ComponentIter<'a, T> {
    data: &'a [AtomicRefCell<T>],
}

impl<'a, T> ComponentIter<'a, T> {
    pub fn new(data: &'a [AtomicRefCell<T>]) -> Self {
        ComponentIter { data }
    }
}

impl<'a, T: Component> Iterator for ComponentIter<'a, T> {
    type Item = AtomicRef<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            return None;
        }
        let (next, data) = self.data.split_first().unwrap();
        self.data = data;
        Some(next.borrow())
    }
}

#[doc(hidden)]
pub struct ComponentIterMut<'a, T> {
    data: &'a [AtomicRefCell<T>],
}

impl<'a, T> ComponentIterMut<'a, T> {
    pub fn new(data: &'a [AtomicRefCell<T>]) -> Self {
        ComponentIterMut { data }
    }
}

impl<'a, T: Component> Iterator for ComponentIterMut<'a, T> {
    type Item = AtomicRefMut<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            return None;
        }

        let (next, data) = self.data.split_first().unwrap();
        self.data = data;
        Some(next.borrow_mut())
    }
}
