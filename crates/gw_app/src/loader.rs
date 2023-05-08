use crate::app::File;
use crate::font::{FontFileLoader, FromGlyphFn, ToGlyphFn};
use crate::img::ImageFileLoader;
use crate::log;
use gw_ecs::Ecs;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum LoadError {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
    ParseError(String),
    ProcessError(String),
}

pub trait LoadHandler {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError>;
}

pub type BoxedLoadHandler = Box<dyn LoadHandler>;

impl<F> LoadHandler for F
where
    F: Fn(&str, Vec<u8>, &mut Ecs) -> Result<(), LoadError>,
{
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        self(path, data, ecs)
    }
}

pub struct LoadInfo {
    path: String,
    file: File,
    cb: Option<BoxedLoadHandler>,
}

impl LoadInfo {
    fn new(path: &str, file: File, cb: BoxedLoadHandler) -> Self {
        LoadInfo {
            path: path.to_owned(),
            cb: Some(cb),
            file,
        }
    }
}

#[derive(Default)]
pub struct Loader {
    files_to_load: VecDeque<LoadInfo>,
    ready: bool,
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            files_to_load: VecDeque::new(),
            ready: true,
        }
    }

    pub fn has_files_to_load(&self) -> bool {
        !self.files_to_load.is_empty()
    }

    fn pop(&mut self) -> Option<LoadInfo> {
        self.files_to_load.pop_front()
    }

    fn unpop(&mut self, info: LoadInfo) {
        self.files_to_load.push_front(info)
    }

    pub fn load_file(&mut self, path: &str, cb: BoxedLoadHandler) -> Result<(), LoadError> {
        log(format!("loading file - {}", path));
        // check to see if we are already loading it... ???

        match crate::app::FileSystem::open(path) {
            Ok(f) => {
                log(format!("file open - {}", path));
                log(format!("loading async file {}", path));
                self.files_to_load.push_back(LoadInfo::new(path, f, cb));
                self.ready = false;
                Ok(())
            }
            Err(e) => Err(LoadError::OpenError(e)),
        }
    }

    pub fn load_font(&mut self, font_path: &str) -> Result<(), LoadError> {
        self.load_file(font_path, Box::new(FontFileLoader::new()))
    }

    pub fn load_font_with_transform(
        &mut self,
        font_path: &str,
        to_glyph: &'static ToGlyphFn,
        from_glyph: &'static FromGlyphFn,
    ) -> Result<(), LoadError> {
        self.load_file(
            font_path,
            Box::new(FontFileLoader::new().with_transforms(to_glyph, from_glyph)),
        )
    }

    pub fn load_image(&mut self, image_path: &str) -> Result<(), LoadError> {
        self.load_file(image_path, Box::new(ImageFileLoader::new()))
    }
}

/// returns true when no more files to process
pub(crate) fn load_files(ecs: &mut Ecs) -> bool {
    // get the next file to load
    let mut load_info = match ecs.try_write_global::<Loader>() {
        None => return true,
        Some(mut loader) => loader.pop(),
    };

    while load_info.is_some() {
        let mut info = load_info.unwrap();
        if info.file.is_ready() {
            match info.file.read_binary() {
                Err(e) => {
                    panic!("\x1b[31mFailed to read file\x1b[0m - {:?}", e);
                }
                Ok(data) => {
                    let mut cb = info.cb.take().unwrap();
                    match cb.file_loaded(&info.path, data, ecs) {
                        Err(e) => {
                            panic!(
                                "\x1b[31mError processing file({})\x1b[0m - {:?}",
                                &info.path, e
                            );
                        }
                        Ok(_) => {
                            println!("Processed file({})", &info.path);
                        }
                    }
                }
            }

            // finished that one, get the next one
            load_info = match ecs.try_write_global::<Loader>() {
                None => return true,
                Some(mut loader) => loader.pop(),
            };
        } else {
            // File not ready, push back on queue
            let mut loader = ecs.write_global::<Loader>();
            loader.unpop(info);
            return false;
        }
    }

    true
}
