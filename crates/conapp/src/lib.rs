mod app;
mod builder;
pub mod codepage437;
pub mod color;
mod context;
pub mod draw;
mod file;
mod font;
mod img;
mod input;
mod load_screen;
pub mod point;
mod runner;
mod screen;
mod simple;
pub mod text;

pub use app::{
    now, perf_now, App, AppConfig, AppEvent, KeyEvent, MouseButtonEvent, VirtualKeyCode,
};
pub use builder::*;
pub use color::{BLACK, RGBA, WHITE}; // so common it is better to just re-export them
pub use context::*;
pub use draw::{BorderType, TextAlign};
pub use file::*;
pub use font::Font;
pub use img::*;
pub use input::AppInput;
pub use load_screen::*;
pub use point::Point;
pub use runner::*;
pub use screen::*;
pub use simple::*;

pub fn console<T: AsRef<str>>(msg: T) {
    app::App::print(msg.as_ref());
}

pub use uni_gl::WebGLRenderingContext;

#[cfg(feature = "ecs")]
pub mod ecs {
    pub use bevy_ecs::*;
}
