use crate::app::File;
use crate::ecs::Ecs;
use crate::font::{FontFileLoader, FromGlyphFn, ToGlyphFn};
use crate::img::ImageFileLoader;
use crate::log;

#[derive(Debug)]
pub enum LoadError {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
    ParseError(String),
    ProcessError(String),
}

pub trait LoadHandler {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, world: &mut Ecs) -> Result<(), LoadError>;
}

pub type BoxedLoadHandler = Box<dyn LoadHandler>;

impl<F> LoadHandler for F
where
    F: Fn(&str, Vec<u8>, &mut Ecs) -> Result<(), LoadError>,
{
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, world: &mut Ecs) -> Result<(), LoadError> {
        self(path, data, world)
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

pub struct Loader {
    files_to_load: Vec<LoadInfo>,
    ready: bool,
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            files_to_load: Vec::new(),
            ready: true,
        }
    }

    pub fn has_files_to_load(&self) -> bool {
        !self.files_to_load.is_empty()
    }

    pub fn load_files(&mut self, ecs: &mut Ecs) -> bool {
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
                        let mut cb = info.cb.take().unwrap();
                        match cb.file_loaded(&info.path, data, ecs) {
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

    pub fn load_file(&mut self, path: &str, cb: BoxedLoadHandler) -> Result<(), LoadError> {
        log(format!("loading file - {}", path));
        // check to see if we are already loading it... ???

        match crate::app::FileSystem::open(path) {
            Ok(f) => {
                log(format!("file open - {}", path));
                log(format!("loading async file {}", path));
                self.files_to_load.push(LoadInfo::new(path, f, cb));
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
