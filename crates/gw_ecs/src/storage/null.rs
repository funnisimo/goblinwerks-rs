// use super::{DistinctStorage, UnprotectedStorage};
// use crate::{
//     components::{CompMut, CompRef},
//     entity::Index,
// };
// use hibitset::BitSetLike;

// /// A null storage type, used for cases where the component
// /// doesn't contain any data and instead works as a simple flag.
// pub struct NullStorage<T>(T);

// impl<T> UnprotectedStorage<T> for NullStorage<T>
// where
//     T: Default,
// {
//     #[cfg(feature = "nightly")]
//     type AccessMut<'a>
//     where
//         T: 'a,
//     = &'a mut T;

//     unsafe fn clean<B>(&mut self, _has: B)
//     where
//         B: BitSetLike,
//     {
//     }

//     unsafe fn get(&self, _: Index, last_system_tick: u32, world_tick: u32) -> CompRef<T> {
//         &self.0
//     }

//     unsafe fn get_mut(&mut self, _: Index, last_system_tick: u32, world_tick: u32) -> CompMut<T> {
//         &mut self.0
//     }

//     unsafe fn insert(&mut self, _: Index, _: T, world_tick: u32) {}

//     unsafe fn remove(&mut self, _: Index, world_tick: u32) -> T {
//         Default::default()
//     }
// }

// impl<T> Default for NullStorage<T>
// where
//     T: Default,
// {
//     fn default() -> Self {
//         use std::mem::size_of;

//         assert_eq!(size_of::<T>(), 0, "NullStorage can only be used with ZST");

//         NullStorage(Default::default())
//     }
// }

// /// This is safe because you cannot mutate ZSTs.
// unsafe impl<T> DistinctStorage for NullStorage<T> {}
