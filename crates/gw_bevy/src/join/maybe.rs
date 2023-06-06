//! Joining of components for iteration over entities with specific components.
use super::Join;
use crate::entity::Index;
use hibitset::{BitSetAll, BitSetLike};

/// A `Join`-able structure that yields all indices, returning `None` for all
/// missing elements and `Some(T)` for found elements.
///
/// For usage see [`Join::maybe()`].
///
/// WARNING: Do not have a join of only `MaybeJoin`s. Otherwise the join will
/// iterate over every single index of the bitset. If you want a join with
/// all `MaybeJoin`s, add an `EntitiesRes` to the join as well to bound the
/// join to all entities that are alive.
///
/// [`Join::maybe()`]: ../join/trait.Join.html#method.maybe
pub struct MaybeJoin<J: Join>(pub J);

impl<T> Join for MaybeJoin<T>
where
    T: Join,
{
    type Mask = BitSetAll;
    type Item = Option<<T as Join>::Item>;
    type Storage = (<T as Join>::Mask, <T as Join>::Storage);

    // SAFETY: This wraps another implementation of `open`, making it dependent on
    // `J`'s correctness. We can safely assume `J` is valid, thus this must be
    // valid, too. No invariants to meet.
    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        let (mask, value, lst, wt) = self.0.open();
        (BitSetAll, (mask, value), lst, wt)
    }

    // SAFETY: No invariants to meet and the unsafe code checks the mask, thus
    // fulfills the requirements for calling `get`
    unsafe fn get(
        (mask, value): &mut Self::Storage,
        id: Index,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<Self::Item> {
        if mask.contains(id) {
            Some(<T as Join>::get(value, id, last_system_tick, world_tick))
        } else {
            Some(None)
        }
    }

    #[inline]
    fn is_unconstrained() -> bool {
        true
    }
}

// SAFETY: This is safe as long as `T` implements `ParJoin` safely.  `MaybeJoin`
// relies on `T as Join` for all storage access and safely wraps the inner
// `Join` API, so it should also be able to implement `ParJoin`.
#[cfg(feature = "parallel")]
unsafe impl<T> ParJoin for MaybeJoin<T> where T: ParJoin {}
