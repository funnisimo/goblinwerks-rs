use super::{Component, ComponentIter, ComponentIterMut, ComponentStorage};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

#[derive(Default)]
pub struct VecStorage<T: Component> {
    data: Vec<AtomicRefCell<T>>,
}

impl<T> VecStorage<T>
where
    T: Component + Default,
{
    pub fn new() -> Self {
        VecStorage { data: Vec::new() }
    }
}

impl<'a, T: Component + Default> ComponentStorage<'a, T> for VecStorage<T> {
    type Iter = ComponentIter<'a, T>;
    type IterMut = ComponentIterMut<'a, T>;

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&'a self, index: usize) -> Option<AtomicRef<'a, T>> {
        match self.data.get(index) {
            None => None,
            Some(item) => Some(item.borrow()),
        }
    }

    fn get_mut(&'a self, index: usize) -> Option<AtomicRefMut<'a, T>> {
        match self.data.get(index) {
            None => None,
            Some(item) => Some(item.borrow_mut()),
        }
    }

    fn iter(&'a self, start_inclusive: usize, end_exclusive: usize) -> Self::Iter {
        ComponentIter::new(&self.data[start_inclusive..end_exclusive])
    }

    fn iter_mut(&'a self, start_inclusive: usize, end_exclusive: usize) -> Self::IterMut {
        ComponentIterMut::new(&self.data[start_inclusive..end_exclusive])
    }
}
