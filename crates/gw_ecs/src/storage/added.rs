//! Joining of components for iteration over entities with specific components.
use crate::join::{Join, ParJoin};
use crate::{change_detection::DetectChanges, entity::Index};

/// A `Join`-able structure that yields all indices, returning `None` for all
/// missing elements and `Some(T)` for found elements.
///
/// For usage see [`Join::maybe()`].
///
/// WARNING: Do not have a join of only `Added`s. Otherwise the join will
/// iterate over every single index of the bitset. If you want a join with
/// all `Added`s, add an `EntitiesRes` to the join as well to bound the
/// join to all entities that are alive.
///
/// [`Join::maybe()`]: ../join/trait.Join.html#method.maybe
pub struct Added<J: Join>(pub J);

impl<J> Added<J>
where
    J: Join,
{
    pub fn new(source: J) -> Self {
        Added(source)
    }
}

impl<T> Join for Added<T>
where
    T: Join,
    <T as Join>::Item: DetectChanges,
{
    type Mask = <T as Join>::Mask;
    type Item = <T as Join>::Item;
    type Storage = <T as Join>::Storage;

    // SAFETY: This wraps another implementation of `open`, making it dependent on
    // `J`'s correctness. We can safely assume `J` is valid, thus this must be
    // valid, too. No invariants to meet.
    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        self.0.open()
    }

    // SAFETY: No invariants to meet and the unsafe code checks the mask, thus
    // fulfills the requirements for calling `get`
    unsafe fn get(
        storage: &mut Self::Storage,
        id: Index,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<Self::Item> {
        match <T as Join>::get(storage, id, last_system_tick, world_tick) {
            None => None,
            Some(comp) => match comp.is_added() {
                true => Some(comp),
                false => None,
            },
        }
    }
}

// SAFETY: This is safe as long as `T` implements `ParJoin` safely.  `Added`
// relies on `T as Join` for all storage access and safely wraps the inner
// `Join` API, so it should also be able to implement `ParJoin`.
#[cfg(feature = "parallel")]
unsafe impl<T> ParJoin for Added<T>
where
    T: ParJoin,
    <T as Join>::Item: DetectChanges,
{
}

///////////////////////////////////////

// /// A `Join`-able structure that yields all indices, returning `None` for all
// /// missing elements and `Some(T)` for found elements.
// ///
// /// For usage see [`Join::maybe()`].
// ///
// /// WARNING: Do not have a join of only `AddedMut`s. Otherwise the join will
// /// iterate over every single index of the bitset. If you want a join with
// /// all `AddedMut`s, add an `EntitiesRes` to the join as well to bound the
// /// join to all entities that are alive.
// ///
// /// [`Join::maybe()`]: ../join/trait.Join.html#method.maybe
// pub struct AddedMut<J: Join>(pub J);

// impl<J> AddedMut<J>
// where
//     J: Join,
// {
//     pub fn new(source: J) -> Self {
//         AddedMut(source)
//     }
// }

// impl<T> Join for AddedMut<T>
// where
//     T: Join,
//     <T as Join>::Item: DetectChanges,
// {
//     type Mask = <T as Join>::Mask;
//     type Item = <T as Join>::Item;
//     type Storage = <T as Join>::Storage;

//     // SAFETY: This wraps another implementation of `open`, making it dependent on
//     // `J`'s correctness. We can safely assume `J` is valid, thus this must be
//     // valid, too. No invariants to meet.
//     unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
//         self.0.open()
//     }

//     // SAFETY: No invariants to meet and the unsafe code checks the mask, thus
//     // fulfills the requirements for calling `get`
//     unsafe fn get(
//         storage: &mut Self::Storage,
//         id: Index,
//         last_system_tick: u32,
//         world_tick: u32,
//     ) -> Option<Self::Item> {
//         match <T as Join>::get(storage, id, last_system_tick, world_tick) {
//             None => None,
//             Some(comp) => match comp.is_added() {
//                 true => Some(comp),
//                 false => None,
//             },
//         }
//     }
// }

// // SAFETY: This is safe as long as `T` implements `ParJoin` safely.  `AddedMut`
// // relies on `T as Join` for all storage access and safely wraps the inner
// // `Join` API, so it should also be able to implement `ParJoin`.
// #[cfg(feature = "parallel")]
// unsafe impl<T> ParJoin for AddedMut<T> where T: ParJoin {}

#[cfg(test)]
mod tests {
    use crate as gw_ecs;
    use crate::prelude::*;

    #[derive(Debug, Component, Default)]
    struct CompA(u32);

    #[derive(Debug, Component, Default)]
    struct CompB(u32);

    #[test]
    fn added_join() {
        let mut world = World::default();

        world.register::<CompA>();

        for i in 0..5 {
            world.spawn(CompA(i));
        }

        {
            let read = world.read_component::<CompA>();
            assert_eq!(read.join().count(), 5);
            assert_eq!(read.added().join().count(), 5);
        }

        world.maintain();

        {
            let read = world.read_component::<CompA>();
            assert_eq!(read.join().count(), 5);
            assert_eq!(read.added().join().count(), 0);
        }
    }

    #[test]
    fn added_join_mut() {
        let mut world = World::default();

        world.register::<CompA>();

        for i in 0..5 {
            world.spawn(CompA(i));
        }

        {
            let mut write = world.write_component::<CompA>();
            assert_eq!(write.join().count(), 5);
            assert_eq!(write.added().join().count(), 5);
        }

        world.maintain();

        {
            let mut write = world.write_component::<CompA>();
            assert_eq!(write.join().count(), 5);
            assert_eq!(write.added().join().count(), 0);
        }
    }

    #[test]
    fn added_join_multi() {
        let mut world = World::default();

        world.register::<CompA>();
        world.register::<CompB>();

        for i in 0..5 {
            world.spawn((CompA(i), CompB(i)));
            world.spawn(CompA(i + 10));
            world.spawn(CompB(i + 20));
        }

        {
            let read = world.read_component::<CompA>();
            assert_eq!(read.join().count(), 10);
            assert_eq!(read.added().join().count(), 10);
        }

        {
            let read_a = world.read_component::<CompA>();
            let mut read_b = world.write_component::<CompB>();

            let mut count = 0;
            for (_a, _b) in (read_a.added(), read_b.added()).join() {
                count += 1;
            }
            assert_eq!(count, 5);
        }

        world.maintain();

        {
            let read = world.read_component::<CompA>();
            assert_eq!(read.join().count(), 10);
            assert_eq!(read.added().join().count(), 0);
        }

        {
            let mut read_a = world.write_component::<CompA>();
            let read_b = world.read_component::<CompB>();

            let mut count = 0;
            for (_a, _b) in (read_a.added(), read_b.added()).join() {
                count += 1;
            }
            assert_eq!(count, 0);
        }
    }
}
