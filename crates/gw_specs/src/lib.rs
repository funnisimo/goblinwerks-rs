pub mod atomic_refcell;
pub mod ecs;
pub mod globals;
#[macro_use]
pub mod shred;
pub mod world;

pub use shred::{Resource, ResourceId, World as Resources};
pub use world::World;
