//! Different types of storages you can use for your components.

use super::{DistinctStorage, SliceAccess, UnprotectedStorage};
use crate::entity::Index;
use hibitset::BitSetLike;

/// Vector storage, like `VecStorage`, but allows safe access to the
/// interior slices because unused slots are always initialized.
///
/// Requires the component to implement `Default`.
///
/// `as_slice()` and `as_mut_slice()` indices correspond to entity IDs.
/// These can be compared to other `DefaultVecStorage`s, to other
/// `VecStorage`s, and to `Entity::id()`s for live entities.
pub struct DefaultVecStorage<T>(Vec<T>);

impl<T> Default for DefaultVecStorage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> UnprotectedStorage<T> for DefaultVecStorage<T>
where
    T: Default,
{
    #[cfg(feature = "nightly")]
    type AccessMut<'a>
    where
        T: 'a,
    = &'a mut T;

    unsafe fn clean<B>(&mut self, _has: B)
    where
        B: BitSetLike,
    {
        self.0.clear();
    }

    unsafe fn get(&self, id: Index) -> &T {
        self.0.get_unchecked(id as usize)
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut T {
        self.0.get_unchecked_mut(id as usize)
    }

    unsafe fn insert(&mut self, id: Index, v: T) {
        let id = id as usize;

        if self.0.len() <= id {
            // fill all the empty slots with default values
            self.0.resize_with(id, Default::default);
            // store the desired value
            self.0.push(v)
        } else {
            // store the desired value directly
            self.0[id] = v;
        }
    }

    unsafe fn remove(&mut self, id: Index) -> T {
        // make a new default value
        let mut v = T::default();
        // swap it into the vec
        std::ptr::swap(self.0.get_unchecked_mut(id as usize), &mut v);
        // return the old value
        v
    }
}

unsafe impl<T> DistinctStorage for DefaultVecStorage<T> {}

impl<T> SliceAccess<T> for DefaultVecStorage<T> {
    type Element = T;

    /// Returns a slice of all the components in this storage.
    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        self.0.as_slice()
    }

    /// Returns a mutable slice of all the components in this storage.
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Element] {
        self.0.as_mut_slice()
    }
}
