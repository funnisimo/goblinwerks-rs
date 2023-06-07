use super::{AnyStorage, UnprotectedStorage};
use crate::{
    components::Component,
    entity::{Entity, EntityBuilder, Index},
    event::Events,
    world::World,
};
use hibitset::{BitSet, BitSetLike};

/// The `UnprotectedStorage` together with the `BitSet` that knows
/// about which elements are stored, and which are not.
#[derive(Default)]
pub struct MaskedStorage<T: Component> {
    pub(super) mask: BitSet,
    pub(super) inner: T::Storage,
    pub(crate) removed: Events<Entity>,
}

impl<T: Component> MaskedStorage<T> {
    /// Creates a new `MaskedStorage`. This is called when you register
    /// a new component type within the world.
    pub fn new(inner: T::Storage) -> MaskedStorage<T> {
        MaskedStorage {
            mask: BitSet::new(),
            inner,
            removed: Default::default(),
        }
    }

    pub(crate) fn open_mut(&mut self) -> (&BitSet, &mut T::Storage) {
        (&self.mask, &mut self.inner)
    }

    /// Clear the contents of this storage.
    /// Does not send any removed events
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
    pub fn drop(&mut self, id: Index, _world_tick: u32) -> bool {
        if self.mask.remove(id) {
            // SAFETY: We checked the mask (`remove` returned `true`)
            unsafe {
                self.inner.drop(id);
            }
            return true;
        }
        false
    }

    fn update_ticks(&mut self, world_tick: u32) {
        let MaskedStorage {
            mask,
            inner,
            removed,
        } = self;
        for id in mask.iter() {
            let comp = unsafe { inner.get_mut(id) };
            comp.ticks.check_ticks(world_tick);
        }
        removed.update();
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
            if MaskedStorage::drop(self, entity.id(), world_tick) {
                self.removed.send(*entity);
            }
        }
    }

    fn register(&self, world: &mut World) {
        world.register::<T>();
    }

    fn maintain(&mut self, world_ticks: u32) {
        self.update_ticks(world_ticks);
    }

    fn try_move_component(&mut self, entity: Entity, source_tick: u32, dest: &mut EntityBuilder) {
        if let Some(old) = self.remove(entity.id(), source_tick) {
            dest.insert(old);
            self.removed.send(entity);
        }
    }
}
