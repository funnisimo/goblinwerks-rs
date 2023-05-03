//! Module for resource related types

pub use self::{
    data::{ReadRes, ReadResExpect, TryReadRes, TryWriteRes, WriteRes, WriteResExpect},
    // entry::Entry,
    setup::{DefaultIfMissing, PanicIfMissing, SetupHandler},
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
