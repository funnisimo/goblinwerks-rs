//! Prelude module
//!
//! Contains all of the most common traits, structures,

pub use super::join::Join;
#[cfg(feature = "parallel")]
pub use super::join::ParJoin;
pub use crate::shred::{
    Accessor, Dispatcher, DispatcherBuilder, Read, ReadExpect, Resource, ResourceId, RunNow,
    StaticAccessor, System, SystemData, Write, WriteExpect,
};
pub use crate::World;
pub use hibitset::BitSet;
pub use shrev::ReaderId;

#[cfg(feature = "parallel")]
pub use crate::shred::AsyncDispatcher;
#[cfg(feature = "parallel")]
pub use rayon::iter::ParallelIterator;

pub use super::{
    changeset::ChangeSet,
    storage::{
        ComponentEvent, DefaultVecStorage, DenseVecStorage, FlaggedStorage, HashMapStorage,
        NullStorage, ReadStorage, Storage, Tracked, VecStorage, WriteStorage,
    },
    world::{Builder, Component, Entities, Entity, EntityBuilder, LazyUpdate, WorldExt},
};
