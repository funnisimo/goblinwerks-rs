use legion::Registry;

use crate::color::init_colors;
use crate::ecs::Ecs;
use crate::ecs::REGISTRY;
use crate::font::FromGlyphFn;
use crate::font::ToGlyphFn;
use crate::loader::BoxedLoadHandler;
use crate::AppConfig;
// use crate::AppContext;
use crate::Runner;

pub type StartupFn = dyn Fn(&mut Ecs) -> ();

/// Builds an application runner
pub struct AppBuilder {
    /// window configuration info
    pub(crate) config: AppConfig,
    /// fonts to load
    pub(crate) fonts: Vec<(String, Option<(&'static ToGlyphFn, &'static FromGlyphFn)>)>,
    /// images to load
    pub(crate) images: Vec<String>,
    /// files to load
    pub(crate) files: Vec<(String, BoxedLoadHandler)>,

    pub(crate) startup: Vec<Box<StartupFn>>,
}

impl AppBuilder {
    /// Starts building a [`Runner`] with the given screen width and height in pixels
    pub fn new(width: u32, height: u32) -> Self {
        init_colors();

        let mut options = AppConfig::new("application", (width, height));
        options.fps = 60;

        AppBuilder {
            config: options,
            fonts: Vec::new(),
            images: Vec::new(),
            files: Vec::new(),
            startup: Vec::new(),
        }
    }

    /// Sets the window title
    pub fn title(mut self, title: &str) -> Self {
        self.config.title = title.to_owned();
        self
    }

    /// Turns on/off the vsync
    pub fn vsync(mut self, val: bool) -> Self {
        self.config.vsync = val;
        self
    }

    // /// Makes the application run headless
    // pub fn headless(mut self, val: bool) -> Self {
    //     self.config.headless = val;
    //     self
    // }

    /// Run fullscreen?
    pub fn fullscreen(mut self, val: bool) -> Self {
        self.config.fullscreen = val;
        self
    }

    /// Resizable?
    pub fn resizable(mut self, val: bool) -> Self {
        self.config.resizable = val;
        self
    }

    /// Show the mouse cursor?
    pub fn show_cursor(mut self, val: bool) -> Self {
        self.config.show_cursor = val;
        self
    }

    /// If on, clicking the close button on the window creates an event that you can handle
    pub fn intercept_close_request(mut self, val: bool) -> Self {
        self.config.intercept_close_request = val;
        self
    }

    /// Sets a func that will get called when the app is fully ready
    pub fn startup(mut self, func: Box<StartupFn>) -> Self {
        self.startup.push(func);
        self
    }

    /// Loads a file on startup
    pub fn file(mut self, file_path: &str, func: BoxedLoadHandler) -> Self {
        self.files.push((file_path.to_owned(), func));
        self
    }

    /// Loads a font on startup
    pub fn font(mut self, font_path: &str) -> Self {
        self.fonts.push((font_path.to_string(), None));
        self
    }

    /// Loads a font on startup
    pub fn font_with_transform(
        mut self,
        font_path: &str,
        to_glyph: &'static ToGlyphFn,
        from_glyph: &'static FromGlyphFn,
    ) -> Self {
        self.fonts
            .push((font_path.to_string(), Some((to_glyph, from_glyph))));
        self
    }

    /// Loads a list of fonts on startup
    pub fn fonts(mut self, font_paths: &[&str]) -> Self {
        for font_path in font_paths {
            self.fonts.push((font_path.to_string(), None));
        }
        self
    }

    /// Loads an image on startup
    pub fn image(mut self, image_path: &str) -> Self {
        self.images.push(image_path.to_string());
        self
    }

    /// Loads an image on startup
    pub fn images(mut self, image_paths: &[&str]) -> Self {
        for image_path in image_paths {
            self.images.push((*image_path).to_owned());
        }
        self
    }

    pub fn register_components<F>(self, func: F) -> Self
    where
        F: FnOnce(&mut Registry<String>) -> (),
    {
        if let Ok(mut registry) = REGISTRY.lock() {
            (func)(&mut *registry);
        }
        self
    }

    /// Sets the fps goal
    pub fn fps(mut self, fps_goal: u32) -> Self {
        self.config.fps = fps_goal;
        self
    }

    /// Builds the [`Runner`]
    pub fn build(self) -> Runner {
        Runner::new(self)
        // for font in self.fonts {
        //     runner.load_font(&font).expect("Failed to load font.");
        // }
        // for image in self.images {
        //     runner.load_image(&image).expect("Failed to load image.");
        // }
        // for (path, func) in self.files {
        //     runner.load_file(&path, func).expect("Failed to load file.");
        // }
        // runner
    }
}
