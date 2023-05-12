use std::mem::MaybeUninit;

use super::{DistinctStorage, SliceAccess, UnprotectedStorage};
use crate::specs::world::Index;
use hibitset::BitSetLike;

/// Dense vector storage. Has a redirection 2-way table
/// between entities and components, allowing to leave
/// no gaps within the data.
///
/// Note that this only stores the data (`T`) densely; indices
/// to the data are stored in a sparse `Vec`.
///
/// `as_slice()` and `as_mut_slice()` indices are local to this
/// `DenseVecStorage` at this particular moment. These indices
/// cannot be compared with indices from any other storage, and
/// a particular entity's position within this slice may change
/// over time.
pub struct DenseVecStorage<T> {
    data: Vec<T>,
    entity_id: Vec<Index>,
    data_id: Vec<MaybeUninit<Index>>,
}

impl<T> Default for DenseVecStorage<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            entity_id: Default::default(),
            data_id: Default::default(),
        }
    }
}

impl<T> SliceAccess<T> for DenseVecStorage<T> {
    type Element = T;

    /// Returns a slice of all the components in this storage.
    ///
    /// Indices inside the slice do not correspond to anything in particular,
    /// and especially do not correspond with entity IDs.
    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        self.data.as_slice()
    }

    /// Returns a mutable slice of all the components in this storage.
    ///
    /// Indices inside the slice do not correspond to anything in particular,
    /// and especially do not correspond with entity IDs.
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Element] {
        self.data.as_mut_slice()
    }
}

impl<T> UnprotectedStorage<T> for DenseVecStorage<T> {
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

    unsafe fn get(&self, id: Index) -> &T {
        let did = self.data_id.get_unchecked(id as usize).assume_init();
        self.data.get_unchecked(did as usize)
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut T {
        let did = self.data_id.get_unchecked(id as usize).assume_init();
        self.data.get_unchecked_mut(did as usize)
    }

    unsafe fn insert(&mut self, id: Index, v: T) {
        let id = id as usize;
        if self.data_id.len() <= id {
            let delta = id + 1 - self.data_id.len();
            self.data_id.reserve(delta);
            self.data_id.set_len(id + 1);
        }
        self.data_id
            .get_unchecked_mut(id)
            .as_mut_ptr()
            .write(self.data.len() as Index);
        self.entity_id.push(id as Index);
        self.data.push(v);
    }

    unsafe fn remove(&mut self, id: Index) -> T {
        let did = self.data_id.get_unchecked(id as usize).assume_init();
        let last = *self.entity_id.last().unwrap();
        self.data_id
            .get_unchecked_mut(last as usize)
            .as_mut_ptr()
            .write(did);
        self.entity_id.swap_remove(did as usize);
        self.data.swap_remove(did as usize)
    }
}

unsafe impl<T> DistinctStorage for DenseVecStorage<T> {}
