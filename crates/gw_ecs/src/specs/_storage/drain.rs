use hibitset::BitSet;

use crate::specs::{
    join::Join,
    storage::MaskedStorage,
    world::{Component, Index},
};

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
    #[test]
    fn basic_drain() {
        use crate::specs::{
            join::Join,
            storage::DenseVecStorage,
            world::{Builder, Component},
        };
        use crate::World;

        #[derive(Debug, PartialEq)]
        struct Comp;

        impl Component for Comp {
            type Storage = DenseVecStorage<Self>;
        }

        let mut world = World::empty(0);
        world.register::<Comp>();

        world.create_entity().build();
        let b = world.create_entity().with(Comp).build();
        let c = world.create_entity().with(Comp).build();
        world.create_entity().build();
        let e = world.create_entity().with(Comp).build();

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