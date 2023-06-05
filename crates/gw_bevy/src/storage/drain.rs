use super::MaskedStorage;
use crate::components::Component;
use crate::{entity::Index, join::Join};
use hibitset::BitSet;

/// A draining storage wrapper which has a `Join` implementation
/// that removes the components.
pub struct Drain<'a, T: Component> {
    /// The masked storage
    pub data: &'a mut MaskedStorage<T>,
    pub last_system_tick: u32,
    pub world_tick: u32,
}

impl<'a, T> Join for Drain<'a, T>
where
    T: Component,
{
    type Mask = BitSet;
    type Item = T; // TODO - CompRef<'i, T> << So that you can get the is_updated, is_inserted info
    type Storage = &'a mut MaskedStorage<T>;

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        let mask = self.data.mask.clone();

        (mask, self.data, self.last_system_tick, self.world_tick)
    }

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn get(
        value: &mut Self::Storage,
        id: Index,
        _last_system_tick: u32,
        world_tick: u32,
    ) -> T {
        value
            .remove(id, world_tick)
            .expect("Tried to access same index twice")
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Builder;
    use crate::storage::DenseVecStorage;
    use crate::world::World;
    use crate::{components::Component, join::Join};

    #[test]
    fn basic_drain() {
        #[derive(Debug, PartialEq, Default)]
        struct Comp;

        impl Component for Comp {
            type Storage = DenseVecStorage<Self>;
        }

        let mut world = World::default();
        world.register::<Comp>();

        world.create_entity().id();
        let b = world.create_entity().with(Comp).id();
        let c = world.create_entity().with(Comp).id();
        world.create_entity().id();
        let e = world.create_entity().with(Comp).id();

        let mut comps = world.write_component::<Comp>();
        let entities = world.entities();

        {
            let mut iter = (comps.drain(), &entities).join();

            assert_eq!(iter.next().unwrap(), (Comp, b));
            assert_eq!(iter.next().unwrap(), (Comp, c));
            assert_eq!(iter.next().unwrap(), (Comp, e));
        }

        assert_eq!((&comps).join().count(), 0);
    }
}
