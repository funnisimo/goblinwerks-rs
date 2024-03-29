use super::Join;
use hibitset::{BitProducer, BitSetLike};
use rayon::iter::{
    plumbing::{bridge_unindexed, Folder, UnindexedConsumer, UnindexedProducer},
    ParallelIterator,
};
use std::cell::UnsafeCell;

/// The purpose of the `ParJoin` trait is to provide a way
/// to access multiple storages in parallel at the same time with
/// the merged bit set.
///
/// # Safety
///
/// The implementation of `ParallelIterator` for `ParJoin` makes multiple
/// assumptions on the structure of `Self`. In particular, `<Self as Join>::get`
/// must be callable from multiple threads, simultaneously, without mutating
/// values not exclusively associated with `id`.
// NOTE: This is currently unspecified behavior. It seems very unlikely that it
// breaks in the future, but technically it's not specified as valid Rust code.
pub unsafe trait ParJoin: Join {
    /// Create a joined parallel iterator over the contents.
    fn par_join(self) -> JoinParIter<Self>
    where
        Self: Sized,
    {
        if <Self as Join>::is_unconstrained() {
            log::warn!(
                "`ParJoin` possibly iterating through all indices, you might've made a join with all `MaybeJoin`s, which is unbounded in length."
            );
        }

        JoinParIter(self)
    }
}

/// `JoinParIter` is a `ParallelIterator` over a group of `Storages`.
#[must_use]
pub struct JoinParIter<J>(J);

impl<J> ParallelIterator for JoinParIter<J>
where
    J: Join + Send,
    J::Mask: Send + Sync,
    J::Item: Send,
    J::Storage: Send,
{
    type Item = J::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let (keys, values, last_system_tick, world_tick) = unsafe { self.0.open() };
        // Create a bit producer which splits on up to three levels
        let producer = BitProducer((&keys).iter(), 3);
        // HACK: use `UnsafeCell` to share `values` between threads;
        // this is the unspecified behavior referred to above.
        let values = UnsafeCell::new(values);

        bridge_unindexed(
            JoinProducer::<J>::new(producer, &values, last_system_tick, world_tick),
            consumer,
        )
    }
}

struct JoinProducer<'a, J>
where
    J: Join + Send,
    J::Mask: Send + Sync + 'a,
    J::Item: Send,
    J::Storage: Send + 'a,
{
    keys: BitProducer<'a, J::Mask>,
    values: &'a UnsafeCell<J::Storage>,
    last_system_tick: u32,
    world_tick: u32,
}

impl<'a, J> JoinProducer<'a, J>
where
    J: Join + Send,
    J::Item: Send,
    J::Storage: 'a + Send,
    J::Mask: 'a + Send + Sync,
{
    fn new(
        keys: BitProducer<'a, J::Mask>,
        values: &'a UnsafeCell<J::Storage>,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Self {
        JoinProducer {
            keys,
            values,
            last_system_tick,
            world_tick,
        }
    }
}

// SAFETY: `Send` is safe to implement if all components of `Self` are logically
// `Send`. `keys` already has `Send` implemented, thus no reasoning is required.
// `values` is a reference to an `UnsafeCell` wrapping `J::Storage`;
// `J::Storage` is constrained to implement `Send`.
// `UnsafeCell` provides interior mutability, but the specification of it allows
// sharing as long as access does not happen simultaneously; this makes it
// generally safe to `Send`, but we are accessing it simultaneously, which is
// technically not allowed. Also see https://github.com/slide-rs/specs/issues/220
unsafe impl<'a, J> Send for JoinProducer<'a, J>
where
    J: Join + Send,
    J::Item: Send,
    J::Storage: 'a + Send,
    J::Mask: 'a + Send + Sync,
{
}

impl<'a, J> UnindexedProducer for JoinProducer<'a, J>
where
    J: Join + Send,
    J::Item: Send,
    J::Storage: 'a + Send,
    J::Mask: 'a + Send + Sync,
{
    type Item = J::Item;

    fn split(self) -> (Self, Option<Self>) {
        let (cur, other) = self.keys.split();
        let values = self.values;
        let first = JoinProducer::new(cur, values, self.last_system_tick, self.world_tick);
        let second =
            other.map(|o| JoinProducer::new(o, values, self.last_system_tick, self.world_tick));

        (first, second)
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        let JoinProducer { values, keys, .. } = self;
        let iter = keys
            .0
            .map(|idx| unsafe {
                // This unsafe block should be safe if the `J::get`
                // can be safely called from different threads with distinct indices.

                // The indices here are guaranteed to be distinct because of the fact
                // that the bit set is split.
                J::get(
                    &mut *values.get(),
                    idx,
                    self.last_system_tick,
                    self.world_tick,
                )
            })
            .filter_map(|v| v);

        folder.consume_iter(iter)
    }
}
