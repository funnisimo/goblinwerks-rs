use super::MaskedStorage;
use crate::components::Component;
use crate::{entity::Index, join::Join};
use hibitset::BitSet;

/// A draining storage wrapper which has a `Join` implementation
/// that removes the components.
pub struct Drain<'a, T: Component> {
    /// The masked storage
    pub data: &'a mut MaskedStorage<T>,
}

impl<'a, T> Join for Drain<'a, T>
where
    T: Component,
{
    type Mask = BitSet;
    type Type = T;
    type Value = &'a mut MaskedStorage<T>;

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        let mask = self.data.mask.clone();

        (mask, self.data)
    }

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn get(value: &mut Self::Value, id: Index) -> T {
        value.remove(id).expect("Tried to access same index twice")
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
