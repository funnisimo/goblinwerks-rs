// use crate::{
//     globals::{GlobalRef, Globals},
//     shred::SetupHandler,
//     Resource, ResourceId, World,
// };
// use std::{collections::HashSet, marker::PhantomData};

mod function_system;
mod system;
mod system_param;

pub use function_system::*;
pub use system::*;
pub use system_param::*;

// pub trait SystemParam {
//     type State: Default;
//     type Fetch<'a>;

//     fn setup(world: &mut World) -> Self::State;
//     fn fetch<'a>(world: &'a World, state: &mut Self::State) -> Self::Fetch<'a>;

//     fn reads() -> HashSet<ResourceId> {
//         HashSet::new()
//     }
//     fn writes() -> HashSet<ResourceId> {
//         HashSet::new()
//     }
// }

// // /////////////////////////////////////////////////

// impl SystemParam for &World {
//     type State = ();
//     type Fetch<'a> = &'a World;

//     fn setup(world: &mut World) -> Self::State {
//         ()
//     }

//     fn fetch<'a>(world: &'a World, state: &mut Self::State) -> Self::Fetch<'a> {
//         world
//     }
// }

// // /////////////////////////////////////////////////

// pub(crate) struct GlobalRes<T>(PhantomData<T>);
// // pub(crate) struct GlobalSet;

// /// Allows to fetch a resource in a system immutably.
// ///
// /// If the resource isn't strictly required, you should use `Option<Read<T>>`.
// ///
// /// # Type parameters
// ///
// /// * `T`: The type of the resource
// /// * `F`: The setup handler (default: `DefaultProvider`)
// pub struct ReadGlobal<T, F = ()>
// where
//     T: Resource,
//     F: SetupHandler<T>,
// {
//     fetch: PhantomData<T>,
//     setup: PhantomData<F>,
// }

// // impl<'a, T, F> ReadGlobal<'a, T, F>
// // where
// //     T: Resource,
// // {
// //     fn new(fetch: GlobalFetch<'a, T>) -> Self {
// //         ReadGlobal {
// //             fetch,
// //             phantom: PhantomData,
// //         }
// //     }
// // }

// // impl<'a, T, F> Deref for ReadGlobal<'a, T, F>
// // where
// //     T: Resource,
// // {
// //     type Target = T;

// //     fn deref(&self) -> &T {
// //         &self.fetch
// //     }
// // }

// // impl<'a, T, F> From<GlobalRef<'a, T>> for ReadGlobal<'a, T, F> {
// //     fn from(fetch: GlobalRef<'a, T>) -> Self {
// //         ReadGlobal {
// //             fetch,
// //             phantom: PhantomData,
// //         }
// //     }
// // }

// impl<T, F> SystemParam for ReadGlobal<T, F>
// where
//     T: Resource,
//     F: SetupHandler<T>,
// {
//     type State = ();
//     type Fetch<'a> = GlobalRef<'a, T>;

//     fn setup(world: &mut World) -> Self::State {
//         F::setup(world);
//         ()
//     }

//     fn fetch<'a>(world: &'a World, _: &mut Self::State) -> Self::Fetch<'a> {
//         world.read_global::<T>()
//     }

//     fn reads() -> HashSet<ResourceId> {
//         let mut reads = HashSet::new();
//         reads.insert(ResourceId::new::<Globals>());
//         reads.insert(ResourceId::new::<GlobalRes<T>>());
//         reads
//     }

//     // fn writes() -> HashSet<ResourceId> {
//     //     vec![]
//     // }
// }

// // /////////////////////////////////////////////////

// pub trait System {
//     fn setup(&mut self, world: &mut World) {
//         let _ = world;
//     }
//     fn run(&mut self, world: &World);
//     fn teardown(&mut self, world: &mut World) {
//         let _ = world;
//     }
// }

// pub type SystemCall = dyn FnMut(&World) -> ();

// pub struct FunctionSystem<P>
// where
//     P: SystemParam,
// {
//     func: Box<dyn for<'a> Fn(&mut Self, &World) -> ()>,
//     state: P::State,
// }

// impl<P> System for FunctionSystem<P>
// where
//     P: SystemParam,
// {
//     fn setup(&mut self, world: &mut World) {
//         self.state = P::setup(world);
//     }

//     fn run(&mut self, world: &World) {
//         let FunctionSystem {
//             func, mut state, ..
//         } = self;

//         let mut fetch = P::fetch(world, &mut state);

//         (func)(&mut fetch);
//     }
// }

// pub trait IntoSystem<A> {
//     fn into_system(self) -> Box<dyn System>;
// }

// impl<F, A> IntoSystem<A> for F
// where
//     F: Fn(A) -> () + 'static,
//     A: SystemParam,
// {
//     fn into_system(self) -> Box<dyn System> {
//         Box::new(FunctionSystem {
//             func: Box::new(move |world: &World, state: &mut A::State| {
//                 let mut data = A::fetch(world, state);
//                 (self.func)(&mut data);
//             }),
//             state: (),
//         })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::World;

//     #[derive(Default)]
//     struct TestRes(u32);

//     fn world_param(world: &World) {
//         world.write_resource::<TestRes>().0 = 1;
//     }

//     #[test]
//     fn world_system() {
//         let mut world = World::empty(1);

//         world.insert_resource(TestRes(0));

//         let mut system = world_param.into_system();

//         assert_eq!(world.read_resource::<TestRes>().0, 0);

//         system.run(&mut world);

//         assert_eq!(world.read_resource::<TestRes>().0, 1);
//     }
// }
