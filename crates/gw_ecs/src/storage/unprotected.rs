use super::StorageCell;
use crate::entity::Index;
use hibitset::BitSetLike;

/// Used by the framework to quickly join components.
pub trait UnprotectedStorage<T>: Default {
    // / The wrapper through with mutable access of a component is performed.
    // #[cfg(feature = "nightly")]
    // type AccessMut<'a>: DerefMut<Target = T>
    // where
    //     Self: 'a;

    /// Clean the storage given a bitset with bits set for valid indices.
    /// Allows us to safely drop the storage.
    ///
    /// # Safety
    ///
    /// May only be called with the mask which keeps track of the elements
    /// existing in this storage.
    unsafe fn clean<B>(&mut self, has: B)
    where
        B: BitSetLike;

    /// Tries reading the data associated with an `Index`.
    /// This is unsafe because the external set used
    /// to protect this storage is absent.
    ///
    /// # Safety
    ///
    /// May only be called after a call to `insert` with `id` and
    /// no following call to `remove` with `id`.
    ///
    /// A mask should keep track of those states, and an `id` being contained
    /// in the tracking mask is sufficient to call this method.
    unsafe fn get(&self, id: Index) -> &StorageCell<T>;

    unsafe fn raw(&self, id: Index) -> &T {
        &self.get(id).data
    }

    // / Tries mutating the data associated with an `Index`.
    // / This is unsafe because the external set used
    // / to protect this storage is absent.
    // /
    // / # Safety
    // /
    // / May only be called after a call to `insert` with `id` and
    // / no following call to `remove` with `id`.
    // /
    // / A mask should keep track of those states, and an `id` being contained
    // / in the tracking mask is sufficient to call this method.
    // #[cfg(feature = "nightly")]
    // unsafe fn get_mut(&mut self, id: Index) -> Self::AccessMut<'_>;

    /// Tries mutating the data associated with an `Index`.
    /// This is unsafe because the external set used
    /// to protect this storage is absent.
    ///
    /// # Safety
    ///
    /// May only be called after a call to `insert` with `id` and
    /// no following call to `remove` with `id`.
    ///
    /// A mask should keep track of those states, and an `id` being contained
    /// in the tracking mask is sufficient to call this method.
    unsafe fn get_mut(&mut self, id: Index) -> &mut StorageCell<T>;

    unsafe fn raw_mut(&mut self, id: Index) -> &mut T {
        &mut self.get_mut(id).data
    }

    /// Inserts new data for a given `Index`.
    ///
    /// # Safety
    ///
    /// May only be called if `insert` was not called with `id` before, or
    /// was reverted by a call to `remove` with `id.
    ///
    /// A mask should keep track of those states, and an `id` missing from the
    /// mask is sufficient to call `insert`.
    unsafe fn insert(&mut self, id: Index, value: StorageCell<T>);

    /// Removes the data associated with an `Index`.
    ///
    /// # Safety
    ///
    /// May only be called if an element with `id` was `insert`ed and not yet
    /// removed / dropped.
    unsafe fn remove(&mut self, id: Index) -> StorageCell<T>;

    /// Drops the data associated with an `Index`.
    /// This could be used when a more efficient implementation for it exists
    /// than `remove` when the data is no longer needed.
    /// Defaults to simply calling `remove`.
    ///
    /// # Safety
    ///
    /// May only be called if an element with `id` was `insert`ed and not yet
    /// removed / dropped.
    unsafe fn drop(&mut self, id: Index) {
        self.remove(id);
    }
}
