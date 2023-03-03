#![warn(clippy::float_cmp)]
use std::{collections::HashMap, sync::Arc};

use crate::{color::RGBA, loader::LoadHandler, log};
// use std::cell::RefCell;
// use std::rc::Rc;

/// An easy way to load PNG images and blit them on the console
pub struct Image {
    // file_loader: FileLoader,
    pub(crate) img: image::RgbaImage,
}

impl Image {
    /// Create an empty image.
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            img: image::RgbaImage::new(width, height),
        }
    }

    pub fn new(buf: &[u8]) -> Self {
        Image {
            img: image::load_from_memory(buf).unwrap().to_rgba8(),
        }
    }

    /// Returns the image's width in pixels or 0 if the image has not yet been loaded
    pub fn width(&self) -> u32 {
        self.img.width()
    }
    /// Returns the image's height in pixels or 0 if the image has not yet been loaded
    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn img(&self) -> &image::RgbaImage {
        &self.img
    }

    /// get the color of a specific pixel inside the image
    pub fn pixel(&self, x: u32, y: u32) -> Option<RGBA> {
        let p = self.img.get_pixel(x, y);
        return Some(RGBA::rgba(p[0], p[1], p[2], p[3]));
    }
    /// sets the color of a specific pixel inside the image
    pub fn put_pixel(&mut self, x: u32, y: u32, color: RGBA) {
        self.img
            .put_pixel(x, y, image::Rgba([color.0, color.1, color.2, color.3]));
    }

    /// If the image has already been loaded, return its size, else return None
    pub fn size(&self) -> (u32, u32) {
        (self.img.width(), self.img.height())
    }
}

pub struct Images {
    cache: HashMap<String, Arc<Image>>,
}

impl Images {
    pub fn new() -> Self {
        Images {
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: &str, font: Arc<Image>) {
        self.cache.insert(id.to_string(), font);
    }

    pub fn get(&self, id: &str) -> Option<Arc<Image>> {
        match self.cache.get(id) {
            None => None,
            Some(f) => Some(f.clone()),
        }
    }
}

pub struct ImageFileLoader;

impl ImageFileLoader {
    pub fn new() -> ImageFileLoader {
        ImageFileLoader
    }
}

impl LoadHandler for ImageFileLoader {
    fn file_loaded(
        &mut self,
        path: &str,
        data: Vec<u8>,
        world: &mut crate::ecs::Ecs,
    ) -> Result<(), crate::loader::LoadError> {
        let image = Arc::new(Image::new(&data));

        let mut fonts = world.resources.get_mut::<Images>().unwrap();
        fonts.insert(path, image);
        log(format!("image load complete - {}", path));
        Ok(())
    }
}
