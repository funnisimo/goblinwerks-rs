//! **Sh**ared **re**source **d**ispatcher
//!
//! This library allows to dispatch
//! systems, which can have interdependencies,
//! shared and exclusive resource access, in parallel.
//!
//! # Examples
//!
//! ```rust
//! extern crate shred;
//!
//! use shred::{DispatcherBuilder, Read, Resource, ResourceId, System, SystemData, World, Write};
//!
//! #[derive(Debug, Default)]
//! struct ResA;
//!
//! #[derive(Debug, Default)]
//! struct ResB;
//!
//! # #[cfg(feature = "shred-derive")]
//! #[derive(SystemData)] // Provided with `shred-derive` feature
//! struct Data<'a> {
//!     a: Read<'a, ResA>,
//!     b: Write<'a, ResB>,
//! }
//!
//! struct EmptySystem;
//!
//! impl<'a> System<'a> for EmptySystem {
//!     type SystemData = Data<'a>;
//!
//!     fn run(&mut self, bundle: Data<'a>) {
//!         println!("{:?}", &*bundle.a);
//!         println!("{:?}", &*bundle.b);
//!     }
//! }
//!
//! let mut world = World::empty(0);
//! let mut dispatcher = DispatcherBuilder::new()
//!     .with(EmptySystem, "empty", &[])
//!     .build();
//! world.insert(ResA);
//! world.insert(ResB);
//!
//! dispatcher.dispatch(&mut world);
//! #
//! # // The following is required for the snippet to compile without the `shred-derive` feature.
//! #
//! # #[cfg(not(feature = "shred-derive"))]
//! # struct Data<'a> {
//! #     a: Read<'a, ResA>,
//! #     b: Write<'a, ResB>,
//! # }
//! #
//! # #[cfg(not(feature = "shred-derive"))]
//! # impl<'a> SystemData<'a> for Data<'a> {
//! #     fn setup(world: &mut World) {
//! #         Read::<'_, ResA>::setup(world);
//! #         Write::<'_, ResB>::setup(world);
//! #     }
//! #
//! #     fn fetch(world: &'a World) -> Self {
//! #         Self {
//! #             a: Read::<'_, ResA>::fetch(world),
//! #             b: Write::<'_, ResB>::fetch(world),
//! #         }
//! #     }
//! #
//! #     fn reads() -> Vec<ResourceId> {
//! #         Read::<'_, ResA>::reads()
//! #     }
//! #
//! #     fn writes() -> Vec<ResourceId> {
//! #         Write::<'_, ResB>::writes()
//! #     }
//! # }
//! ```
//!
//! Once you are more familiar with how system data and parallelization works,
//! you can take look at a more flexible and performant way to dispatch:
//! `ParSeq`. Using it is bit trickier, but it allows dispatching without any
//! virtual function calls.

#![deny(unused_must_use, clippy::disallowed_types)]
// #![warn(missing_docs)]

pub mod cell;

mod dispatch;
// mod meta;
mod system;
#[macro_use]
mod resources;

/// A reexport of the `#[derive(SystemData]` macro provided by `shred-derive`.
/// This requires that the `shred-derive` feature is enabled.
// #[cfg(feature = "derive")]
// pub use gw_macro::SystemData;

// #[cfg(feature = "parallel")]
// pub use self::dispatch::AsyncDispatcher;

#[cfg(feature = "parallel")]
pub use self::dispatch::{Par, ParSeq, RunWithPool, Seq};
pub use self::{
    dispatch::{
        // BatchAccessor, BatchController, BatchUncheckedWorld,
        Dispatcher,
        DispatcherBuilder,
        // MultiDispatchController, MultiDispatcher,
    },
    resources::{
        DefaultIfMissing,
        // Entry, Fetch, FetchMut,
        PanicIfMissing,
        ReadRes,
        ReadResExpect,
        Resource,
        ResourceId,
        Resources,
        SetupHandler,
        TryReadRes,
        TryWriteRes,
        WriteRes,
        WriteResExpect,
    },
    // meta::{CastFrom, MetaIter, MetaIterMut, MetaTable},
    system::{
        Accessor, AccessorCow, DynamicSystemData, RunNow, RunningTime, StaticAccessor, System,
        SystemData,
    },
};

pub(crate) use resources::fetch_panic;
