use super::{DistinctStorage, StorageCell, UnprotectedStorage};
use crate::entity::Index;
use hibitset::BitSetLike;
use std::collections::BTreeMap;

/// BTreeMap-based storage.
pub struct BTreeStorage<T>(BTreeMap<Index, StorageCell<T>>);

impl<T> Default for BTreeStorage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> UnprotectedStorage<T> for BTreeStorage<T> {
    #[cfg(feature = "nightly")]
    type AccessMut<'a>
    where
        T: 'a,
    = &'a mut T;

    unsafe fn clean<B>(&mut self, _has: B)
    where
        B: BitSetLike,
    {
        // nothing to do
    }

    unsafe fn get(&self, id: Index) -> &StorageCell<T> {
        &self.0[&id]
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut StorageCell<T> {
        self.0.get_mut(&id).unwrap()
    }

    unsafe fn insert(&mut self, id: Index, v: StorageCell<T>) {
        self.0.insert(id, v);
    }

    unsafe fn remove(&mut self, id: Index) -> StorageCell<T> {
        self.0.remove(&id).unwrap()
    }
}

unsafe impl<T> DistinctStorage for BTreeStorage<T> {}
