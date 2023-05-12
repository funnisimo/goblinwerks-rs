use super::{DistinctStorage, UnprotectedStorage};
use crate::specs::world::Index;
use hibitset::BitSetLike;
use std::collections::HashMap;

/// `HashMap`-based storage. Best suited for rare components.
///
/// This uses the [hashbrown::HashMap] internally.
pub struct HashMapStorage<T>(HashMap<Index, T>);

impl<T> Default for HashMapStorage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> UnprotectedStorage<T> for HashMapStorage<T> {
    #[cfg(feature = "nightly")]
    type AccessMut<'a>
    where
        T: 'a,
    = &'a mut T;

    unsafe fn clean<B>(&mut self, _has: B)
    where
        B: BitSetLike,
    {
        //nothing to do
    }

    unsafe fn get(&self, id: Index) -> &T {
        &self.0[&id]
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut T {
        self.0.get_mut(&id).unwrap()
    }

    unsafe fn insert(&mut self, id: Index, v: T) {
        self.0.insert(id, v);
    }

    unsafe fn remove(&mut self, id: Index) -> T {
        self.0.remove(&id).unwrap()
    }
}

unsafe impl<T> DistinctStorage for HashMapStorage<T> {}
