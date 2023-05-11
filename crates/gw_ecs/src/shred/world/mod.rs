//! Module for resource related types

pub use self::{
    data::{ReadRes, ReadResSetup, TryReadRes, TryWriteRes, WriteRes, WriteResSetup},
    // entry::Entry,
    setup::{NoSetup, SetupDefault, SetupHandler},
};

mod data;
mod entry;
// mod res_downcast;
#[macro_use]
mod setup;
mod resource;
mod resources;

pub use resource::*;
pub use resources::*;

pub(crate) use setup::fetch_panic;
