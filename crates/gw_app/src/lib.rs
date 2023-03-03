pub mod app;
pub mod builder;
pub mod codepage437;
pub mod color;
pub mod console;
pub mod context;
pub mod draw;
pub mod ecs;
pub mod font;
pub mod fps;
pub mod img;
pub mod input;
pub mod load_screen;
pub mod loader;
pub mod messages;
pub mod runner;
pub mod screen;
pub mod text;

pub use app::{
    now, perf_now, App, AppConfig, AppEvent, KeyEvent, MouseButtonEvent, VirtualKeyCode,
};
pub use builder::AppBuilder;
pub use color::{BLACK, RGBA, WHITE}; // so common it is better to just re-export them
pub use draw::{BorderType, TextAlign};
pub use img::Image;
pub use input::AppInput;
// pub use load_screen::LoadingScreen;
pub use console::{Buffer, Console, Glyph};
pub use ecs::Ecs;
pub use runner::Runner;
pub use screen::{Key, MsgData, Screen, ScreenResult};

pub fn log<T: AsRef<str>>(msg: T) {
    app::App::print(msg.as_ref());
}

// pub use uni_gl::WebGLRenderingContext;
