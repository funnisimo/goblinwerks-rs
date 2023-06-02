use super::{DistinctStorage, SliceAccess, UnprotectedStorage};
use crate::entity::Index;
use hibitset::BitSetLike;
use std::mem::MaybeUninit;

/// Vector storage. Uses a simple `Vec`. Supposed to have maximum
/// performance for the components mostly present in entities.
///
/// `as_slice()` and `as_mut_slice()` indices correspond to
/// entity IDs. These can be compared to other `VecStorage`s, to
/// other `DefaultVecStorage`s, and to `Entity::id()`s for live
/// entities.
pub struct VecStorage<T>(Vec<MaybeUninit<T>>);

impl<T> Default for VecStorage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> SliceAccess<T> for VecStorage<T> {
    type Element = MaybeUninit<T>;

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        self.0.as_slice()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Element] {
        self.0.as_mut_slice()
    }
}

impl<T> UnprotectedStorage<T> for VecStorage<T> {
    #[cfg(feature = "nightly")]
    type AccessMut<'a>
    where
        T: 'a,
    = &'a mut T;

    unsafe fn clean<B>(&mut self, has: B)
    where
        B: BitSetLike,
    {
        use std::ptr;
        for (i, v) in self.0.iter_mut().enumerate() {
            if has.contains(i as u32) {
                // drop in place
                ptr::drop_in_place(&mut *v.as_mut_ptr());
            }
        }
        self.0.set_len(0);
    }

    unsafe fn get(&self, id: Index) -> &T {
        &*self.0.get_unchecked(id as usize).as_ptr()
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut T {
        &mut *self.0.get_unchecked_mut(id as usize).as_mut_ptr()
    }

    unsafe fn insert(&mut self, id: Index, v: T) {
        let id = id as usize;
        if self.0.len() <= id {
            let delta = id + 1 - self.0.len();
            self.0.reserve(delta);
            self.0.set_len(id + 1);
        }
        // Write the value without reading or dropping
        // the (currently uninitialized) memory.
        *self.0.get_unchecked_mut(id as usize) = MaybeUninit::new(v);
    }

    unsafe fn remove(&mut self, id: Index) -> T {
        use std::ptr;
        ptr::read(self.get(id))
    }
}

unsafe impl<T> DistinctStorage for VecStorage<T> {}
