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
