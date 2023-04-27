// //! Helper module for some internals, most users don't need to interact with it.
// Replaced original with these references to allow Globals to work.

use crate::atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

/// Alias for AtomicRefCell
pub type TrustCell<T> = AtomicRefCell<T>;

/// Alias for AtomicRef
pub type Ref<'b, T> = AtomicRef<'b, T>;

/// Alias for AtomicRefMut
pub type RefMut<'b, T> = AtomicRefMut<'b, T>;
