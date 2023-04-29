pub mod atomic_refcell;
pub mod ecs;
pub mod globals;
#[macro_use]
pub mod shred;
pub mod specs;

pub mod world;

pub use ecs::Ecs;
pub use globals::{ReadGlobal, ReadGlobalExpect, WriteGlobal, WriteGlobalExpect};
pub use shred::{Read, ReadExpect, Write, WriteExpect};
pub use shred::{Resource, ResourceId, SystemData, World as Resources};
pub use world::World;

pub use shred::{BatchController, Dispatcher, DispatcherBuilder};
pub use specs::{
    join::*,
    storage::{DenseVecStorage, MaskedStorage, VecStorage},
    world::Builder,
    Component, Entities, Entity, EntityBuilder, ReadStorage, System, WriteStorage,
};

#[cfg(feature = "parallel")]
pub use crate::shred::AsyncDispatcher;
#[cfg(feature = "parallel")]
pub use rayon::iter::ParallelIterator;

#[cfg(feature = "derive")]
pub use gw_macro::Component; // ConvertSaveLoad
#[cfg(feature = "derive")]
pub use gw_macro::SystemData;
