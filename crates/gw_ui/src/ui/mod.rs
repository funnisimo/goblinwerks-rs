#![allow(unused_variables, dead_code)]

// use std::collections::HashMap;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub enum Align {
    #[default]
    Min,
    Center,
    Max,
}

impl Align {
    pub const LEFT: Self = Self::Min;
    pub const CENTER: Self = Self::Center;
    pub const RIGHT: Self = Self::Max;

    pub const TOP: Self = Self::Min;
    pub const MIDDLE: Self = Self::Center;
    pub const BOTTOM: Self = Self::Max;
}

mod tag;
pub use tag::*;

mod element;
pub use element::*;

mod traits;
pub use traits::*;

mod ui;
pub use ui::*;

mod label;
pub use label::*;

mod dialog;
pub use dialog::*;

mod text;
pub use text::*;

mod button;
pub use button::*;

mod select;
pub use select::*;

mod body;
pub use body::*;

mod frame;
pub use frame::*;

mod span;
pub use span::*;

mod checkbox;
pub use checkbox::*;

mod list;
pub use list::*;

mod radio;
pub use radio::*;

mod div;
pub use div::*;

#[cfg(test)]
pub mod test;
