pub mod atomic_refcell;
pub mod ecs;
pub mod globals;
#[macro_use]
pub mod shred;
pub mod bevy;
pub mod legion;
pub mod specs;

pub mod schedule;
pub mod storage;
pub mod world;

pub use ecs::Ecs;
pub use globals::{ReadGlobal, ReadGlobalDefault, WriteGlobal, WriteGlobalDefault};
pub use shred::{ReadRes, ReadResSetup, TryReadRes, TryWriteRes, WriteRes, WriteResSetup};
pub use shred::{Resource, ResourceId, Resources, SystemData};
pub use world::World;

// pub use shred::{Dispatcher, DispatcherBuilder};
pub use specs::{
    join::*,
    world::{Builder, Commands},
    Component, Entities, Entity, EntityBuilder, System,
};
pub use storage::{DenseVecStorage, MaskedStorage, ReadComp, Storage, VecStorage, WriteComp};

// #[cfg(feature = "parallel")]
// pub use crate::shred::AsyncDispatcher;

#[cfg(feature = "parallel")]
pub use rayon::iter::ParallelIterator;

#[cfg(feature = "derive")]
pub use gw_macro::Component; // ConvertSaveLoad
#[cfg(feature = "derive")]
pub use gw_macro::SystemData;

pub use atomize;
