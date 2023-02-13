mod native_keycode;

use crate::console;

use self::native_keycode::translate_scan_code;
use super::events;
use super::translate_virtual_key;
use super::AppConfig;
use super::AppEvent;
use super::{File, FileSystem};
use glutin;
use glutin::Context;
use glutin::PossiblyCurrent;
use glutin::WindowedContext;
use std::cell::RefCell;
use std::env;
use std::os::raw::c_void;
use std::process;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::ModifiersState;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::monitor::VideoMode;
use winit::window::Fullscreen;
use winit::window::WindowBuilder;

enum WindowContext {
    // Headless(Context<PossiblyCurrent>),
    Normal(WindowedContext<PossiblyCurrent>),
}

impl WindowContext {
    fn hidpi_factor(&self) -> f32 {
        match self {
            WindowContext::Normal(ref w) => w.window().scale_factor() as f32,
            // _ => 1.0,
        }
    }

    fn window(&self) -> &WindowedContext<PossiblyCurrent> {
        match self {
            WindowContext::Normal(ref w) => w,
            // _ => unimplemented!(),
        }
    }

    fn context(&self) -> &Context<PossiblyCurrent> {
        match self {
            WindowContext::Normal(w) => w.context(),
            // WindowContext::Headless(w) => w,
        }
    }

    fn swap_buffers(&self) -> Result<(), glutin::ContextError> {
        match self {
            WindowContext::Normal(ref w) => w.swap_buffers(),
            // WindowContext::Headless(_) => Ok(()),
        }
    }
}

struct InputState {
    modifiers: ModifiersState,
    mouse_pos: (f32, f32),
    had_mouse_move: bool,
}

impl InputState {
    fn new() -> Self {
        InputState {
            modifiers: ModifiersState::empty(),
            mouse_pos: (0.0, 0.0),
            had_mouse_move: false,
        }
    }

    fn shift(&self) -> bool {
        self.modifiers.shift()
    }

    fn alt(&self) -> bool {
        self.modifiers.alt()
    }

    fn ctrl(&self) -> bool {
        self.modifiers.ctrl()
    }

    fn logo(&self) -> bool {
        self.modifiers.logo()
    }
}

/// the main application struct
pub struct App {
    window: WindowContext,
    events_loop: Option<EventLoop<()>>,
    intercept_close_request: bool,
    input_state: InputState,
    // pub events: Rc<RefCell<Vec<AppEvent>>>,
    pub events: Rc<RefCell<Vec<AppEvent>>>,
    dropped_files: Vec<File>,
    fullscreen_resolution: VideoMode,
    fps: u32,
}

fn get_virtual_key(input: KeyboardInput) -> String {
    match input.virtual_keycode {
        Some(k) => {
            let mut s = translate_virtual_key(k).into();
            if s == "" {
                s = format!("{:?}", k);
            }
            s
        }
        None => "".into(),
    }
}

fn get_scan_code(input: KeyboardInput) -> String {
    translate_scan_code(input.scancode & 0xFF).into()
}

fn translate_event(e: Event<()>, input_state: &mut InputState) -> Option<AppEvent> {
    if let Event::WindowEvent {
        event: winevent, ..
    } = e
    {
        match winevent {
            WindowEvent::MouseInput { state, button, .. } => {
                let button_num = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                    MouseButton::Other(val) => val as usize,
                };
                let event = events::MouseButtonEvent {
                    button: button_num,
                    pos: input_state.mouse_pos,
                };
                match state {
                    ElementState::Pressed => Some(AppEvent::MouseDown(event)),
                    ElementState::Released => Some(AppEvent::MouseUp(event)),
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                input_state.mouse_pos = position.into();
                // Some(AppEvent::MousePos(input_state.mouse_pos))
                input_state.had_mouse_move = true;
                None
            }
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => Some(AppEvent::KeyDown(events::KeyEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    key_code: input.virtual_keycode.unwrap(),
                    shift: input_state.shift(),
                    alt: input_state.alt(),
                    ctrl: input_state.ctrl(),
                })),
                ElementState::Released => Some(AppEvent::KeyUp(events::KeyEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    key_code: input.virtual_keycode.unwrap(),
                    shift: input_state.shift(),
                    alt: input_state.alt(),
                    ctrl: input_state.ctrl(),
                })),
            },
            WindowEvent::ReceivedCharacter(c) => Some(AppEvent::CharEvent(c)),
            WindowEvent::Resized(size) => Some(AppEvent::Resized(size.into())),
            WindowEvent::CloseRequested => Some(AppEvent::CloseRequested),
            WindowEvent::DroppedFile(path) => {
                Some(AppEvent::FileDropped(path.to_str().unwrap().to_owned()))
            }
            _ => None,
        }
    } else {
        None
    }
}

impl App {
    /// create a new game window
    pub fn new(config: AppConfig) -> App {
        use glutin::*;
        let events_loop = EventLoop::new();
        let gl_req = GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (2, 0),
        };
        let fullscreen_resolution = events_loop
            .available_monitors()
            .nth(0)
            .unwrap()
            .video_modes()
            .nth(0)
            .unwrap();
        // let window = if config.headless {
        //     let headless_context = ContextBuilder::new()
        //         .with_gl(gl_req)
        //         .with_gl_profile(GlProfile::Core)
        //         .build_headless(&events_loop, (config.size.0, config.size.1).into())
        //         .unwrap();

        //     WindowContext::Headless(unsafe { headless_context.make_current().unwrap() })
        // } else {
        let window_builder = WindowBuilder::new()
            .with_title(config.title)
            .with_fullscreen(if config.fullscreen {
                Some(Fullscreen::Exclusive(fullscreen_resolution.clone()))
            } else {
                None
            })
            .with_resizable(config.resizable)
            .with_inner_size(LogicalSize::new(config.size.0, config.size.1));

        let windowed_context = ContextBuilder::new()
            .with_vsync(config.vsync)
            .with_gl(gl_req)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window_builder, &events_loop)
            .unwrap();

        windowed_context
            .window()
            .set_cursor_visible(config.show_cursor);

        let window = WindowContext::Normal(unsafe { windowed_context.make_current().unwrap() });
        // };

        App {
            window,
            events_loop: Some(events_loop),
            intercept_close_request: config.intercept_close_request,
            events: Rc::new(RefCell::new(Vec::new())),
            dropped_files: Vec::new(),
            input_state: InputState::new(),
            fullscreen_resolution,
            fps: config.fps,
        }
    }

    /// return the screen resolution in physical pixels
    pub fn screen_resolution(&self) -> (u32, u32) {
        let WindowContext::Normal(ref glwindow) = self.window;
        if let Some(ref monitor) = glwindow.window().current_monitor() {
            return monitor.size().into();
        }
        (0, 0)
    }

    pub fn viewport_size(&self) -> (u32, u32) {
        let size = self.window.window().window().inner_size();
        (size.width, size.height)
    }

    /// return the command line / URL parameters
    pub fn params() -> Vec<String> {
        let mut params: Vec<String> = env::args().collect();
        params.remove(0);
        params
    }

    /// activate or deactivate fullscreen. only works on native target
    pub fn set_fullscreen(&mut self, b: bool) {
        let WindowContext::Normal(ref glwindow) = self.window;
        if b {
            glwindow.window().set_fullscreen(Some(Fullscreen::Exclusive(
                self.fullscreen_resolution.clone(),
            )));
        } else {
            glwindow.window().set_fullscreen(None);
        }
    }

    /// print a message on standard output (native) or js console (web)
    pub fn print<T: Into<String>>(msg: T) {
        println!("{}", msg.into());
    }

    /// exit current process (close the game window). On web target, this does nothing.
    pub fn exit() {
        process::exit(0);
    }

    /// returns the HiDPI factor for current screen
    pub fn hidpi_factor(&self) -> f32 {
        self.window.hidpi_factor()
    }

    fn proc_address(&self, name: &str) -> *const c_void {
        self.window.context().get_proc_address(name) as *const c_void
    }

    /// return the opengl context for this window
    pub fn canvas<'p>(&'p self) -> Box<dyn 'p + FnMut(&str) -> *const c_void> {
        Box::new(move |name| self.proc_address(name))
    }

    fn handle_event(&mut self, event: Event<()>) -> (bool, bool) {
        let mut running = true;
        let mut next_frame = false;
        match event {
            Event::RedrawRequested(_) => {}
            Event::MainEventsCleared => {
                next_frame = true;
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if !self.intercept_close_request {
                        running = false;
                    }
                }
                WindowEvent::Resized(size) => {
                    // Fixed for Windows which minimized to emit a Resized(0,0) event
                    if size.width != 0 && size.height != 0 {
                        self.window.window().resize(*size);
                    }
                }
                WindowEvent::ModifiersChanged(new_state) => {
                    self.input_state.modifiers = *new_state;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    // issue tracked in https://github.com/tomaka/winit/issues/41
                    // Right now we handle it manually.
                    if cfg!(target_os = "macos") {
                        if let Some(keycode) = input.virtual_keycode {
                            if keycode == VirtualKeyCode::Q && self.input_state.logo() {
                                running = false;
                            }
                        }
                    }
                }
                WindowEvent::DroppedFile(ref path) => {
                    let filepath = path.to_str().unwrap();
                    self.dropped_files.push(FileSystem::open(filepath).unwrap());
                }
                _ => (),
            },
            _ => (),
        };

        if let Some(app_event) = translate_event(event, &mut self.input_state) {
            // println!("uni app event - {:?}", app_event);
            let mut ev = self.events.borrow_mut();
            if match app_event {
                AppEvent::CharEvent(ch) => match ev.iter_mut().last() {
                    // eat char events for backspace
                    Some(AppEvent::KeyDown(key_down)) => {
                        key_down.key = ch.to_string();
                        match key_down.key_code {
                            VirtualKeyCode::Back | VirtualKeyCode::Delete => false,
                            _ => true,
                        }
                    }
                    _ => true,
                },
                _ => true,
            } {
                ev.push(app_event);
            }
        }

        (running, next_frame)
    }

    pub fn dropped_file(&mut self) -> Option<File> {
        self.dropped_files.pop()
    }

    /// start the game loop, calling provided callback every frame
    pub fn run<'a, F>(mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut Self) -> (),
    {
        self.events.borrow_mut().push(AppEvent::Ready);

        let frame_ms = if self.fps > 0 {
            console(format!("Running at {} fps", self.fps));
            Duration::from_millis(1000 / self.fps as u64)
        } else {
            console(format!("Fps limit not set, using 1000"));
            Duration::from_millis(1)
        };
        let mut next_frame_time = Instant::now() + frame_ms;

        let events_loop = self.events_loop.take().unwrap();
        events_loop.run(move |event, _, control_flow| {
            // control_flow.set_poll();

            let (running, next_frame) = self.handle_event(event);
            if !running {
                control_flow.set_exit();
                return;
            }

            if next_frame {
                next_frame_time = next_frame_time + frame_ms;
                //do mouse pos event
                if self.input_state.had_mouse_move {
                    self.events
                        .borrow_mut()
                        .push(AppEvent::MousePos(self.input_state.mouse_pos));
                    self.input_state.had_mouse_move = false;
                }

                callback(&mut self);
                self.events.borrow_mut().clear();
                self.window.swap_buffers().unwrap();
            }
            control_flow.set_wait_until(next_frame_time);
        });
    }
}

/// return the seconds since the epoch
pub fn now() -> f64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_secs_f64()
}

/// return a time in secs
pub fn perf_now() -> f64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_secs_f64()
}
