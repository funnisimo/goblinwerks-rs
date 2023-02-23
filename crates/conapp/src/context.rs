use crate::app::File;
use crate::color::RGBA;
use crate::console::Program;
use crate::font::{parse_char_size, Font};
use crate::img::Image;
use crate::input::AppInput;
use crate::{log, MsgData};
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::{BufferBit, WebGLRenderingContext};

#[cfg(feature = "ecs")]
use bevy_ecs::prelude::*;
// use legion::*;

pub static SUBCELL_BYTES: &[u8] = include_bytes!("../resources/subcell.png");
pub static TERMINAL_8X8_BYTES: &[u8] = include_bytes!("../resources/terminal_8x8.png");

pub type LoadCallback = dyn Fn(Vec<u8>, &mut AppContext) -> Result<(), LoadError>;

pub struct LoadInfo {
    path: String,
    file: File,
    cb: Option<Box<LoadCallback>>,
}

impl LoadInfo {
    fn new(path: &str, cb: Box<LoadCallback>, file: File) -> Self {
        LoadInfo {
            path: path.to_owned(),
            cb: Some(cb),
            file,
        }
    }
}

// struct AsyncFile(String, File, Option<Vec<u8>>);

#[derive(Debug)]
pub enum LoadError {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
    ParseError(String),
    ProcessError(String),
}

pub struct AppContext {
    // pub(super) cons: Vec<Console>,
    pub(crate) input: AppInput,
    pub(crate) fps: Fps,
    pub(crate) screen_size: (u32, u32),
    pub(crate) frame_time_ms: f64,
    pub(crate) gl: WebGLRenderingContext,
    pub(crate) fonts: HashMap<String, Rc<Font>>,
    pub(crate) images: HashMap<String, Rc<Image>>,
    pub(crate) ready: bool,
    // pub(crate) file_loader: FileLoader,
    pub(crate) simple_program: Program,
    pub(crate) files_to_load: Vec<LoadInfo>,
    // #[cfg(feature = "ecs")]
    // pub resources: Resources,
    // #[cfg(feature = "ecs")]
    // pub world: World,
    #[cfg(feature = "ecs")]
    pub world: World,
    pub(crate) messages: Option<Vec<(String, Option<MsgData>)>>,
}

impl AppContext {
    pub(crate) fn new(
        gl: WebGLRenderingContext,
        screen_size: (u32, u32),
        input: AppInput,
        fps_goal: u32,
    ) -> Self {
        let mut ctx = AppContext {
            input,
            fps: Fps::new(fps_goal),
            screen_size: screen_size,
            frame_time_ms: 0.0,
            simple_program: Program::new(&gl),
            gl,
            fonts: HashMap::new(),
            images: HashMap::new(),
            ready: false,
            // file_loader: FileLoader::new(),
            files_to_load: Vec::new(),
            // #[cfg(feature = "ecs")]
            // resources: Resources::default(),
            #[cfg(feature = "ecs")]
            world: World::default(),
            messages: Some(Vec::new()),
        };

        let sub_cell_font = Rc::new(Font::new(&ctx.gl, SUBCELL_BYTES, (4, 4)));
        let default_font = Rc::new(Font::new(&ctx.gl, TERMINAL_8X8_BYTES, (8, 8)));
        ctx.insert_font("SUBCELL", sub_cell_font);
        ctx.insert_font("DEFAULT", default_font);

        log(format!(
            "AppContext::new - screen_size={:?}",
            ctx.screen_size()
        ));

        ctx
    }

    pub(crate) fn resize(&mut self, screen_width: u32, screen_height: u32) {
        log(format!(
            "appcontext::resize - {}x{}",
            screen_width, screen_height
        ));
        self.screen_size = (screen_width, screen_height);
    }

    pub fn has_files_to_load(&self) -> bool {
        !self.files_to_load.is_empty()
    }

    pub(crate) fn load_files(&mut self) -> bool {
        if self.ready {
            return true;
        }
        while self.has_files_to_load() {
            let file = &mut self.files_to_load.get_mut(0).unwrap().file;
            if file.is_ready() {
                match file.read_binary() {
                    Err(e) => {
                        println!("Failed to read file - {:?}", e);
                    }
                    Ok(data) => {
                        let mut info = self.files_to_load.remove(0);
                        let cb = info.cb.take().unwrap();
                        match cb(data, self) {
                            Err(e) => {
                                println!("Error processing file({}) - {:?}", &info.path, e);
                            }
                            Ok(_) => {
                                println!("Processed file({})", &info.path);
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
        if self.has_files_to_load() {
            return false;
        }

        self.ready = true;
        log("All files loaded - ready");
        true
    }

    pub fn gl(&self) -> &WebGLRenderingContext {
        &self.gl
    }

    pub fn clear(&self, color: Option<RGBA>) {
        self.gl.clear(uni_gl::BufferBit::Depth); // If using ZPos
        self.gl.clear(BufferBit::Color);
        match color {
            None => {}
            Some(c) => {
                let data = c.to_f32();
                self.gl.clear_color(data.0, data.1, data.2, data.3);
            }
        }
    }

    pub fn input(&self) -> &AppInput {
        &self.input
    }

    pub fn fps(&self) -> u32 {
        self.fps.current()
    }

    pub fn average_fps(&self) -> u32 {
        self.fps.average()
    }

    pub fn frame_time_ms(&self) -> f64 {
        self.frame_time_ms
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    // pub fn simple_console(&mut self, width: u32, height: u32, fontpath: &str) -> Console {
    //     Console::new(width, height, fontpath)
    // }

    pub fn load_file(&mut self, path: &str, cb: Box<LoadCallback>) -> Result<(), LoadError> {
        log(format!("loading file - {}", path));
        match crate::app::FileSystem::open(path) {
            Ok(mut f) => {
                log(format!("file open - {}", path));
                if f.is_ready() {
                    match f.read_binary() {
                        Ok(buf) => {
                            return cb(buf, self);
                        }
                        Err(e) => Err(LoadError::ReadError(e)),
                    }
                } else {
                    log(format!("loading async file {}", path));
                    self.files_to_load.push(LoadInfo::new(path, cb, f));
                    self.ready = false;
                    Ok(())
                }
            }
            Err(e) => Err(LoadError::OpenError(e)),
        }
    }

    pub fn load_font(&mut self, font_path: &str) -> Result<(), LoadError> {
        let char_size = parse_char_size(font_path);
        let path = font_path.to_owned();

        self.load_file(
            font_path,
            Box::new(move |data, app: &mut AppContext| {
                let font = Rc::new(Font::new(app.gl(), &data, char_size));
                app.insert_font(&path, font);
                log(format!("font load complete - {}", path));
                Ok(())
            }),
        )
    }

    pub fn load_image(&mut self, image_path: &str) -> Result<(), LoadError> {
        let path = image_path.to_owned();
        self.load_file(
            image_path,
            Box::new(move |data, app| {
                let image = Rc::new(Image::new(&data));
                app.insert_image(&path, image);
                Ok(())
            }),
        )
    }

    pub fn insert_font(&mut self, name: &str, font: Rc<Font>) {
        self.fonts.insert(name.to_owned(), font);
    }

    pub fn get_font(&self, name: &str) -> Option<Rc<Font>> {
        match self.fonts.get(name) {
            None => None,
            Some(font) => Some(font.clone()),
        }
    }

    pub fn insert_image(&mut self, name: &str, image: Rc<Image>) {
        self.images.insert(name.to_owned(), image);
    }

    pub fn get_image(&self, name: &str) -> Option<Rc<Image>> {
        match self.images.get(name) {
            None => None,
            Some(image) => Some(image.clone()),
        }
    }

    pub fn send_message(&mut self, id: &str, value: Option<MsgData>) {
        if let Some(ref mut messages) = self.messages {
            messages.push((id.to_owned(), value));
        }
    }
}

pub struct Fps {
    counter: u32,
    start: f64,
    last: f64,
    total_frames: u64,
    fps: u32,
    average: u32,
    goal: u32,
}

impl Fps {
    pub fn new(goal: u32) -> Fps {
        let now = crate::app::now();
        Fps {
            counter: 0,
            total_frames: 0,
            start: now,
            last: now,
            fps: 0,
            average: 0,
            goal,
        }
    }

    pub fn goal(&self) -> u32 {
        self.goal
    }

    pub fn current(&self) -> u32 {
        self.fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.total_frames += 1;
        let curr = crate::app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            self.average = (self.total_frames as f64 / (self.last - self.start)) as u32;
        }
    }
    pub fn average(&self) -> u32 {
        self.average
    }
}
