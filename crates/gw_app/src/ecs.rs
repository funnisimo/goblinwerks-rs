use crate::console::Program;
use crate::fps::Fps;
use crate::img::Images;
use crate::loader::Loader;
use crate::messages::Messages;
use crate::{font::Fonts, log, App, AppConfig, AppInput};
use legion::systems::Resource;
pub use legion::*;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub size: (u32, u32),
    pub real_size: (u32, u32),
    pub screen_size: (u32, u32),
    pub hidpi_factor: f32,
}

#[derive(Debug, Clone)]
pub struct Time {
    pub now: f64,
    pub delta: f64,
}

impl Time {
    pub fn new(now: f64, delta: f64) -> Self {
        Time { now, delta }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            now: 0.0,
            delta: 0.0,
        }
    }
}

pub struct Ecs {
    pub world: World,
    pub resources: Resources,
}

impl Ecs {
    pub fn new() -> Self {
        Ecs {
            world: World::default(),
            resources: Resources::default(),
        }
    }
}

pub fn init_ecs(ecs: &mut Ecs, app: &App, options: &AppConfig) {
    let resources = &mut ecs.resources;

    // FPS
    resources.insert(Fps::new(options.fps));

    // Window Sizes
    let real_window_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
    let real_window_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

    let screen_resolution = app.screen_resolution();

    let (x_offset, y_offset) = if options.fullscreen && cfg!(not(target_arch = "wasm32")) {
        let x_offset = (screen_resolution.0 - real_window_width) as i32 / 2;
        let y_offset = (screen_resolution.1 - real_window_height) as i32 / 2;
        (x_offset, y_offset)
    } else {
        (0, 0)
    };

    let window_info = WindowInfo {
        size: options.size,
        real_size: (real_window_width, real_window_height),
        screen_size: screen_resolution,
        hidpi_factor: app.hidpi_factor(),
    };

    log(&format!(
        "Screen size: {:?}, window_size: {:?}, offset {}x{}, real_window_size: {:?},  hidpi factor: {}",
        window_info.screen_size,
        window_info.size,
        x_offset,
        y_offset,
        window_info.real_size,
        window_info.hidpi_factor
    ));

    resources.insert(window_info);

    // GL + Console Program
    let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
    gl.viewport(x_offset, y_offset, real_window_width, real_window_height);
    gl.enable(uni_gl::Flag::Blend as i32);
    // gl.enable(uni_gl::Flag::DepthTest as i32);   // If using ZPos
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(uni_gl::BufferBit::Color);
    // gl.clear(uni_gl::BufferBit::Depth);  // If using ZPos
    gl.blend_equation(uni_gl::BlendEquation::FuncAdd);
    gl.blend_func(
        uni_gl::BlendMode::SrcAlpha,
        uni_gl::BlendMode::OneMinusSrcAlpha,
    );

    resources.insert(Program::new(&gl));

    resources.insert(Fonts::new(&gl));
    resources.insert(Images::new());

    resources.insert(gl);

    // App Input
    let input = if cfg!(target_arch = "wasm32") {
        AppInput::new(
            (options.size.0, options.size.1),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    } else {
        AppInput::new(
            (real_window_width, real_window_height),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    };
    resources.insert(input);

    resources.insert(Time::default());
    resources.insert(Messages::new());
    resources.insert(Loader::new());

    log("Configured ECS");
}

pub fn scoped_resource<F, R, T>(ecs: &mut Ecs, func: F) -> T
where
    F: FnOnce(&mut Ecs, &mut R) -> T,
    R: Resource,
{
    let mut resource = ecs.resources.remove::<R>().unwrap();
    let result = func(ecs, &mut resource);
    ecs.resources.insert(resource);
    return result;
}
