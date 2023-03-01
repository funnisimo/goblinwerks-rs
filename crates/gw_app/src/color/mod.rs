use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

mod rgba;
pub use rgba::*;

pub mod named;

mod parse;
pub use parse::*;

lazy_static! {
    pub static ref COLORS: Mutex<HashMap<String, RGBA>> = Mutex::new(HashMap::new());
}

pub fn init_colors() {
    named::add_named_colors_to_palette();
    register_color("none", (0, 0, 0, 0).into());
    register_color("null", (0, 0, 0, 0).into());
}

pub fn register_color(name: &str, color: RGBA) {
    COLORS.lock().unwrap().insert(name.to_owned(), color);
}

fn static_color(name: &str) -> Option<RGBA> {
    match COLORS.lock().unwrap().get(name) {
        None => None,
        Some(rgba) => Some(rgba.clone()),
    }
}
