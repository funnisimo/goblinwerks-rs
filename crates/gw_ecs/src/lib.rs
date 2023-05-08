pub mod atomic_refcell;
pub mod ecs;
pub mod globals;
#[macro_use]
pub mod shred;
pub mod specs;

pub mod world;

pub mod utils;

pub use ecs::Ecs;
pub use globals::{ReadGlobal, ReadGlobalDefault, WriteGlobal, WriteGlobalDefault};
pub use shred::{ReadRes, ReadResSetup, TryReadRes, TryWriteRes, WriteRes, WriteResSetup};
pub use shred::{Resource, ResourceId, Resources, SystemData};
pub use world::World;

pub use shred::{Dispatcher, DispatcherBuilder};
pub use specs::{
    join::*,
    storage::{DenseVecStorage, MaskedStorage, Storage, VecStorage},
    world::{Builder, Commands},
    Component, Entities, Entity, EntityBuilder, ReadComp, System, WriteComp,
};

// #[cfg(feature = "parallel")]
// pub use crate::shred::AsyncDispatcher;

#[cfg(feature = "parallel")]
pub use rayon::iter::ParallelIterator;

#[cfg(feature = "derive")]
pub use gw_macro::Component; // ConvertSaveLoad
#[cfg(feature = "derive")]
pub use gw_macro::SystemData;

pub use atomize;
