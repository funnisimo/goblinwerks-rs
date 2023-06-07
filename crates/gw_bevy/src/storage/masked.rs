use super::{AnyStorage, UnprotectedStorage};
use crate::{
    components::Component,
    entity::{Entity, EntityBuilder, Index},
    world::World,
};
use hibitset::{BitSet, BitSetLike};

/// The `UnprotectedStorage` together with the `BitSet` that knows
/// about which elements are stored, and which are not.
pub struct MaskedStorage<T: Component> {
    pub(super) mask: BitSet,
    pub(super) inner: T::Storage,
}

impl<T: Component> Default for MaskedStorage<T>
where
    T::Storage: Default,
{
    fn default() -> Self {
        Self {
            mask: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<T: Component> MaskedStorage<T> {
    /// Creates a new `MaskedStorage`. This is called when you register
    /// a new component type within the world.
    pub fn new(inner: T::Storage) -> MaskedStorage<T> {
        MaskedStorage {
            mask: BitSet::new(),
            inner,
        }
    }

    pub(crate) fn open_mut(&mut self) -> (&BitSet, &mut T::Storage) {
        (&self.mask, &mut self.inner)
    }

    /// Clear the contents of this storage.
    pub fn clear(&mut self) {
        // SAFETY: `self.mask` is the correct mask as specified.
        unsafe {
            self.inner.clean(&self.mask);
        }
        self.mask.clear();
    }

    /// Remove an element by a given index.
    pub fn remove(&mut self, id: Index, _world_tick: u32) -> Option<T> {
        if self.mask.remove(id) {
            // SAFETY: We checked the mask (`remove` returned `true`)
            Some(unsafe { self.inner.remove(id).data })
        } else {
            None
        }
    }

    /// Drop an element by a given index.
    pub fn drop(&mut self, id: Index, _world_tick: u32) {
        if self.mask.remove(id) {
            // SAFETY: We checked the mask (`remove` returned `true`)
            unsafe {
                self.inner.drop(id);
            }
        }
    }

    fn check_change_ticks(&mut self, world_tick: u32) {
        let MaskedStorage { mask, inner } = self;
        for id in mask.iter() {
            let comp = unsafe { inner.get_mut(id) };
            comp.ticks.check_ticks(world_tick);
        }
    }
}

impl<T: Component> Drop for MaskedStorage<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> AnyStorage for MaskedStorage<T>
where
    T: Component,
{
    fn drop(&mut self, entities: &[Entity], world_tick: u32) {
        for entity in entities {
            MaskedStorage::drop(self, entity.id(), world_tick);
        }
    }

    fn register(&self, world: &mut World) {
        world.register::<T>();
    }

    fn maintain(&mut self, world_ticks: u32) {
        self.check_change_ticks(world_ticks);
    }

    fn try_move_component(&mut self, entity: Entity, source_tick: u32, dest: &mut EntityBuilder) {
        dest.maybe_insert::<T>(self.remove(entity.id(), source_tick));
    }
}
