use uni_gl::BufferBit;

use super::input::AppInput;
use crate::ecs::{init_ecs, scoped_resource, Ecs, Time, WindowInfo};
use crate::ecs::{systems::ResourceSet, Read, Write};
use crate::fps::Fps;
use crate::load_screen::LoadingScreen;
use crate::loader::Loader;
use crate::messages::Messages;
use crate::screen::BoxedScreen;
use crate::{log, App, AppBuilder, AppConfig, AppEvent, Screen, ScreenResult, RGBA};
// use std::sync::Arc;

/// What is returned by the internal update and input functions
enum RunnerEvent {
    /// Save a screenshot. parameter = file path.
    /// The file name must have a .png extension.
    /// This is ignored on WASM platform.
    Capture(String),
    /// end the program
    Exit,
    /// Skip to next stage of processing (input->update->render)
    Next,
}

/// This is the game application. It handles the creation of the game window, the window events including player input events and runs the main game loop.
pub struct Runner {
    /// The uni_gl::App that controls the window
    app: Option<crate::app::App>,
    /// All of the configuration settings
    builder: AppBuilder,
    /// Maximum number of update calls to do in one frame
    max_frameskip: i32,

    ecs: Option<Ecs>,
    screens: Vec<Box<dyn Screen>>,
    // screen_resolution: (u32, u32),
    // real_screen_size: (u32, u32),
}

impl Runner {
    pub fn new(mut builder: AppBuilder) -> Self {
        let options = &mut builder.config;
        // options.headless = false;
        let app = crate::app::App::new(options.clone());

        // let real_screen_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
        // let real_screen_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

        // let screen_resolution = app.screen_resolution();

        log("Runner created");

        Self {
            ecs: None,
            app: Some(app),
            builder,
            max_frameskip: 5,
            screens: Vec::new(),
            // screen_resolution,
            // real_screen_size: (real_screen_width, real_screen_height),
        }
    }

    fn config(&self) -> &AppConfig {
        &self.builder.config
    }

    fn push(&mut self, ctx: &mut Ecs, mut screen: Box<dyn Screen>) {
        screen.setup(ctx);
        if screen.is_full_screen() {
            clear_screen(ctx, None);
        }
        self.screens.push(screen);
    }

    // pub fn load_file(&mut self, path: &str, cb: Box<LoadCallback>) -> Result<(), LoadError> {
    //     self.app_ctx.as_mut().unwrap().load_file(path, cb)
    // }

    // pub fn load_font(&mut self, font_path: &str) -> Result<(), LoadError> {
    //     self.app_ctx.as_mut().unwrap().load_font(font_path)
    // }

    // pub fn load_image(&mut self, image_path: &str) -> Result<(), LoadError> {
    //     self.app_ctx.as_mut().unwrap().load_image(image_path)
    // }

    fn resize(
        &mut self,
        ecs: &mut Ecs,
        hidpi_factor: f32,
        (real_screen_width, real_screen_height): (u32, u32),
    ) {
        log(format!(
            "runner::resize - {}x{}, hidpi={}",
            real_screen_width, real_screen_height, hidpi_factor
        ));

        {
            let (mut window_info, mut input, gl) = <(
                Write<WindowInfo>,
                Write<AppInput>,
                Read<uni_gl::WebGLRenderingContext>,
            )>::fetch_mut(&mut ecs.resources);

            let (x_offset, y_offset) =
                if self.config().fullscreen && cfg!(not(target_arch = "wasm32")) {
                    let x_offset = (window_info.screen_size.0 - real_screen_width) as i32 / 2;
                    let y_offset = (window_info.screen_size.1 - real_screen_height) as i32 / 2;
                    (x_offset, y_offset)
                } else {
                    (0, 0)
                };
            window_info.real_size = (real_screen_width, real_screen_height);
            window_info.size = (
                (real_screen_width as f32 / hidpi_factor) as u32,
                (real_screen_height as f32 / hidpi_factor) as u32,
            );

            gl.viewport(x_offset, y_offset, real_screen_width, real_screen_height);

            // let con_size = self.api.con().size();
            if cfg!(target_arch = "wasm32") {
                input.resize(
                    self.config().size,
                    // con_size,
                    (x_offset as u32, y_offset as u32),
                )
            } else {
                input.resize(
                    (real_screen_width, real_screen_height),
                    // con_size,
                    (x_offset as u32, y_offset as u32),
                )
            };
        }

        // engine.resize(&mut self.api);
        for screen in self.screens.iter_mut() {
            screen.resize(ecs);
        }
    }

    fn handle_event(&mut self, ctx: &mut Ecs, ev: &mut AppEvent) -> Option<RunnerEvent> {
        {
            let mut input = ctx.resources.get_mut::<AppInput>().unwrap();
            input.on_event(ev);
        }

        if let Some(mode) = self.screens.last_mut() {
            match mode.input(ctx, ev) {
                ScreenResult::Continue => (),
                ScreenResult::Capture(name) => return Some(RunnerEvent::Capture(name)),
                ScreenResult::Pop => {
                    clear_screen(ctx, None);
                    mode.teardown(ctx);
                    self.screens.pop();
                    match self.screens.last_mut() {
                        Some(m) => m.resume(ctx),
                        _ => {}
                    }
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Replace(next) => {
                    clear_screen(ctx, None);
                    mode.teardown(ctx);
                    self.screens.pop();
                    self.push(ctx, next);
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Push(next) => {
                    mode.pause(ctx);
                    self.push(ctx, next);
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Quit => {
                    log("Received Quit");
                    return Some(RunnerEvent::Exit);
                }
            }
        }
        None
    }

    fn handle_messages(&mut self, ctx: &mut Ecs) -> Option<RunnerEvent> {
        let messages = ctx.resources.get_mut::<Messages>().unwrap().take();

        for (id, val) in messages {
            if let Some(screen) = self.screens.last_mut() {
                match screen.message(ctx, id, val) {
                    ScreenResult::Capture(name) => return Some(RunnerEvent::Capture(name)),
                    ScreenResult::Pop => {
                        clear_screen(ctx, None);
                        screen.teardown(ctx);
                        self.screens.pop();
                        match self.screens.last_mut() {
                            Some(m) => m.resume(ctx),
                            _ => {}
                        }
                        // self.render(ctx);
                        return Some(RunnerEvent::Next);
                    }
                    ScreenResult::Replace(next) => {
                        clear_screen(ctx, None);
                        screen.teardown(ctx);
                        self.screens.pop();
                        self.push(ctx, next);
                        // self.render(ctx);
                        return Some(RunnerEvent::Next);
                    }
                    ScreenResult::Push(next) => {
                        screen.pause(ctx);
                        self.push(ctx, next);
                        // self.render(ctx);
                        return Some(RunnerEvent::Next);
                    }
                    ScreenResult::Quit => {
                        log("Received Quit");
                        return Some(RunnerEvent::Exit);
                    }
                    ScreenResult::Continue => {}
                }
            }
        }
        None
    }

    fn handle_input(
        &mut self,
        ctx: &mut Ecs,
        hidpi_factor: f32,
        events: &mut Vec<crate::app::AppEvent>,
    ) -> Option<RunnerEvent> {
        for evt in events.iter_mut() {
            if let crate::app::AppEvent::Resized(size) = evt {
                self.resize(ctx, hidpi_factor, *size);
            } else {
                if let Some(ev) = self.handle_event(ctx, evt) {
                    match ev {
                        RunnerEvent::Exit => {
                            self.screens.clear(); // clear all screens on quit
                        }
                        _ => {}
                    }
                    return Some(ev);
                }
            }
        }
        None
    }

    #[deprecated]
    pub fn run_screen(self, screen: BoxedScreen) {
        self.run(screen)
    }

    pub fn run(mut self, screen: BoxedScreen) {
        // self.api.set_font_path(&self.options.font_path);
        let app = self.app.take().unwrap();

        let mut last_frame_time = crate::app::perf_now();
        let mut next_frame = last_frame_time;
        let mut called = false;

        let mut create = Some(screen);
        log(format!("Runner started"));

        app.run(move |app: &mut crate::app::App| match self.ecs.take() {
            None => {
                for ev in app.events.borrow().iter() {
                    match ev {
                        AppEvent::Ready => {
                            log("Runner ready");

                            let mut ctx = Ecs::new();
                            init_ecs(&mut ctx, &app, self.config());
                            if let Some(func) = create.take() {
                                self.do_startup_files(&mut ctx);
                                self.do_startup_screen(&mut ctx, func);
                            }
                            self.ecs = Some(ctx);
                        }
                        _ => {}
                    }
                }
            }
            Some(mut ctx) => {
                if called {
                    scoped_resource(&mut ctx, |ecs: &mut Ecs, loader: &mut Loader| {
                        loader.load_files(ecs);
                    });
                }
                self.do_frame(&mut ctx, app, &mut last_frame_time, &mut next_frame);
                self.ecs = Some(ctx);

                called = true;
            }
        });
    }

    fn do_startup_files(&mut self, ctx: &mut Ecs) {
        let mut loader = ctx.resources.get_mut::<Loader>().unwrap();

        for (font, transform) in self.builder.fonts.drain(..) {
            let (to_glyph, from_glyph) = transform.unwrap();
            loader
                .load_font_with_transform(&font, to_glyph, from_glyph)
                .expect("Failed to load font.");
        }
        for image in self.builder.images.drain(..) {
            loader.load_image(&image).expect("Failed to load image.");
        }
        for (path, func) in self.builder.files.drain(..) {
            loader.load_file(&path, func).expect("Failed to load file.");
        }
    }

    fn do_startup_screen(&mut self, ctx: &mut Ecs, func: BoxedScreen) {
        let has_files_to_load = ctx.resources.get::<Loader>().unwrap().has_files_to_load();
        let mut screen = match has_files_to_load {
            false => func,
            true => {
                log("Using loading screen");
                LoadingScreen::new(func)
            }
        };

        screen.setup(ctx);
        self.screens.push(screen);
    }

    fn do_frame(
        &mut self,
        ctx: &mut Ecs,
        app: &mut App,
        last_frame_time: &mut f64,
        next_frame: &mut f64,
    ) {
        if self.screens.is_empty() {
            return crate::app::App::exit();
        }

        if let Some(event) =
            self.handle_input(ctx, app.hidpi_factor(), &mut *app.events.borrow_mut())
        {
            match event {
                RunnerEvent::Capture(filepath) => {
                    capture_screen(ctx, &filepath);
                    return;
                }
                RunnerEvent::Exit => {
                    log("App Exit");
                    return crate::app::App::exit();
                }
                RunnerEvent::Next => {}
            }
        }

        if let Some(event) = self.handle_messages(ctx) {
            match event {
                RunnerEvent::Capture(filepath) => {
                    capture_screen(ctx, &filepath);
                    return;
                }
                RunnerEvent::Exit => {
                    log("App Exit");
                    return crate::app::App::exit();
                }
                RunnerEvent::Next => {}
            }
        }

        let fps_goal = ctx.resources.get::<Fps>().unwrap().goal();

        let mut skipped_frames: i32 = -1;
        let time = crate::app::perf_now();
        let skip_ticks = match fps_goal {
            0 => time - *last_frame_time,
            x => 1.0 / x as f64,
        };

        while time >= *last_frame_time && skipped_frames < self.max_frameskip {
            // self.app_ctx.frame_time_ms = SKIP_TICKS as f32 * 1000.0; // TODO - Use real elapsed time?
            ctx.resources.insert(Time::new(time, skip_ticks * 1000.0)); // TODO - Use real elapsed time?

            if let Some(event) = self.update(ctx) {
                match event {
                    RunnerEvent::Capture(filepath) => capture_screen(ctx, &filepath),
                    RunnerEvent::Exit => return crate::app::App::exit(),
                    RunnerEvent::Next => {}
                }
            }
            *last_frame_time += skip_ticks;
            // next_tick += SKIP_TICKS;
            skipped_frames += 1;
        }

        ctx.resources.get_mut::<AppInput>().unwrap().on_frame_end();

        if skipped_frames == self.max_frameskip {
            // next_tick = time + SKIP_TICKS;
            *last_frame_time = time + skip_ticks;
        }
        if fps_goal == 0 || time >= *next_frame {
            self.render(ctx);
            ctx.resources.get_mut::<Fps>().unwrap().step();

            if fps_goal > 0 {
                *next_frame += 1.0 / fps_goal as f64;
            }
        }
    }

    fn update(&mut self, ctx: &mut Ecs) -> Option<RunnerEvent> {
        if let Some(screen) = self.screens.last_mut() {
            match screen.update(ctx) {
                ScreenResult::Continue => (),
                ScreenResult::Capture(name) => return Some(RunnerEvent::Capture(name)),
                ScreenResult::Pop => {
                    clear_screen(ctx, None);
                    screen.teardown(ctx);
                    self.screens.pop();
                    match self.screens.last_mut() {
                        Some(m) => m.resume(ctx),
                        _ => {}
                    }
                }
                ScreenResult::Replace(next) => {
                    clear_screen(ctx, None);
                    screen.teardown(ctx);
                    self.screens.pop();
                    self.push(ctx, next);
                }
                ScreenResult::Push(next) => {
                    screen.pause(ctx);
                    self.push(ctx, next);
                }
                ScreenResult::Quit => {
                    return Some(RunnerEvent::Exit);
                }
            }
        }
        None
    }

    /// This is called before drawing the console on the screen. The framerate depends on the screen frequency, the graphic cards and on whether you activated vsync or not.
    /// The framerate is not reliable so don't update time related stuff in this function.
    /// The screen will display the content of the root console provided by `api.con()`
    fn render(&mut self, ctx: &mut Ecs) {
        // Find last full screen mode (that is where we start drawing)
        let mut start_idx = 0;
        for (idx, m) in self.screens.iter().enumerate() {
            if m.is_full_screen() {
                start_idx = idx;
            }
        }
        clear_screen(ctx, None);
        for screen in self.screens.iter_mut().skip(start_idx) {
            screen.render(ctx);
        }
    }
}

// fn create_ctx(app: &App, options: &AppConfig) -> AppContext {
//     let real_screen_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
//     let real_screen_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

//     let screen_resolution = app.screen_resolution();
//     let (x_offset, y_offset) = if options.fullscreen && cfg!(not(target_arch = "wasm32")) {
//         let x_offset = (screen_resolution.0 - real_screen_width) as i32 / 2;
//         let y_offset = (screen_resolution.1 - real_screen_height) as i32 / 2;
//         (x_offset, y_offset)
//     } else {
//         (0, 0)
//     };
//     log(&format!(
//         "Screen size {} x {} offset {} x {} GL viewport : {} x {}  hidpi factor : {}",
//         options.size.0,
//         options.size.1,
//         x_offset,
//         y_offset,
//         real_screen_width,
//         real_screen_height,
//         app.hidpi_factor()
//     ));

//     let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
//     gl.viewport(x_offset, y_offset, real_screen_width, real_screen_height);
//     gl.enable(uni_gl::Flag::Blend as i32);
//     // gl.enable(uni_gl::Flag::DepthTest as i32);   // If using ZPos
//     gl.clear_color(0.0, 0.0, 0.0, 1.0);
//     gl.clear(uni_gl::BufferBit::Color);
//     // gl.clear(uni_gl::BufferBit::Depth);  // If using ZPos
//     gl.blend_equation(uni_gl::BlendEquation::FuncAdd);
//     gl.blend_func(
//         uni_gl::BlendMode::SrcAlpha,
//         uni_gl::BlendMode::OneMinusSrcAlpha,
//     );

//     let input = if cfg!(target_arch = "wasm32") {
//         AppInput::new(
//             (options.size.0, options.size.1),
//             // (options.console_width, options.console_height),
//             (x_offset as u32, y_offset as u32),
//         )
//     } else {
//         AppInput::new(
//             (real_screen_width, real_screen_height),
//             // (options.console_width, options.console_height),
//             (x_offset as u32, y_offset as u32),
//         )
//     };

//     AppContext::new(gl, options.size.clone(), input, options.fps)
// }

/// This captures an in-game screenshot and saves it to the file
fn capture_screen(ecs: &Ecs, filepath: &str) {
    if cfg!(not(target_arch = "wasm32")) {
        let (gl, window_info) =
            <(Read<uni_gl::WebGLRenderingContext>, Read<WindowInfo>)>::fetch(&ecs.resources);

        let (w, h) = window_info.real_size;

        let mut img = image::DynamicImage::new_rgba8(w, h);
        let pixels = img.as_mut_rgba8().unwrap();

        gl.pixel_storei(uni_gl::PixelStorageMode::PackAlignment, 1);
        gl.read_pixels(
            0,
            0,
            w,
            h,
            uni_gl::PixelFormat::Rgba,
            uni_gl::PixelType::UnsignedByte,
            pixels,
        );

        // disabled on wasm target
        image::save_buffer(
            filepath,
            &image::imageops::flip_vertical(&img),
            w,
            h,
            image::ColorType::Rgba8,
        )
        .expect("Failed to save buffer to the specified path");
    } else {
        log("Screen capture not supported on web platform");
    }
}

pub fn clear_screen(ecs: &mut Ecs, color: Option<RGBA>) {
    let gl = ecs
        .resources
        .get::<uni_gl::WebGLRenderingContext>()
        .unwrap();

    gl.clear(uni_gl::BufferBit::Depth); // If using ZPos
    gl.clear(BufferBit::Color);
    match color {
        None => {}
        Some(c) => {
            let data = c.to_f32();
            gl.clear_color(data.0, data.1, data.2, data.3);
        }
    }
}
