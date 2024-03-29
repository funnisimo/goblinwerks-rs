use crate::codepage437;
use crate::{loader::LoadHandler, panel::set_texture_params};
use crate::{log, Glyph};
use gw_ecs::prelude::Ecs;
use std::{collections::HashMap, sync::Arc};
use uni_gl::{WebGLRenderingContext, WebGLTexture};

pub static SUBCELL_BYTES: &[u8] = include_bytes!("../assets/subcell.png");
pub static TERMINAL_8X8_BYTES: &[u8] = include_bytes!("../assets/terminal_8x8.png");

pub type ToGlyphFn = dyn Fn(char) -> Glyph;
pub type FromGlyphFn = dyn Fn(Glyph) -> char;

pub(crate) fn default_to_glyph(ch: char) -> Glyph {
    ch as Glyph
}

pub(crate) fn default_from_glyph(glyph: Glyph) -> char {
    char::from_u32(glyph).unwrap()
}

pub struct Font {
    img_size: (u32, u32),
    char_size: (u32, u32),
    count: u32,
    pub(crate) texture: WebGLTexture,
    pub(crate) to_glyph_fn: &'static ToGlyphFn,
    pub(crate) from_glyph_fn: &'static FromGlyphFn,
}

impl Font {
    pub fn new(gl: &WebGLRenderingContext, bytes: &[u8], char_size: (u32, u32)) -> Self {
        let mut font = Font {
            img_size: (0, 0),
            char_size,
            count: 0,
            texture: create_font_texture(gl),
            to_glyph_fn: &default_to_glyph,
            from_glyph_fn: &default_from_glyph,
        };

        font.load_font_img(bytes, gl);
        font
    }

    pub fn with_transforms(
        mut self,
        to_glyph: &'static ToGlyphFn,
        from_glyph: &'static FromGlyphFn,
    ) -> Self {
        self.to_glyph_fn = to_glyph;
        self.from_glyph_fn = from_glyph;
        self
    }

    pub fn img_width(&self) -> u32 {
        self.img_size.0
    }
    pub fn img_height(&self) -> u32 {
        self.img_size.1
    }
    pub fn char_width(&self) -> u32 {
        self.char_size.0
    }
    pub fn char_height(&self) -> u32 {
        self.char_size.1
    }
    pub fn char_size(&self) -> (u32, u32) {
        self.char_size
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn set_transform(
        &mut self,
        to_glyph: &'static ToGlyphFn,
        from_glyph: &'static FromGlyphFn,
    ) {
        self.to_glyph_fn = to_glyph;
        self.from_glyph_fn = from_glyph;
    }

    pub fn to_glyph(&self, ch: char) -> Glyph {
        (self.to_glyph_fn)(ch)
    }

    pub fn from_glyph(&self, glyph: Glyph) -> char {
        (self.from_glyph_fn)(glyph)
    }

    fn load_font_img(&mut self, buf: &[u8], gl: &WebGLRenderingContext) {
        log(format!("load font image - {}", buf.len()));
        let mut img = image::load_from_memory(&buf).unwrap().to_rgba8();
        process_image(&mut img);

        self.img_size = (img.width() as u32, img.height() as u32);
        self.count =
            (self.img_width() / self.char_width()) * (self.img_height() / self.char_height());

        gl.bind_texture(&self.texture);

        gl.tex_image2d(
            uni_gl::TextureBindPoint::Texture2d, // target
            0,                                   // level
            img.width() as u16,                  // width
            img.height() as u16,                 // height
            uni_gl::PixelFormat::Rgba,           // format
            uni_gl::PixelType::UnsignedByte,     // type
            &*img,                               // data
        );
    }
}

pub fn parse_char_size(filepath: &str) -> (u32, u32) {
    let mut char_width = 0;
    let mut char_height = 0;

    let start = match filepath.rfind('_') {
        None => {
            panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
        }
        Some(idx) => idx,
    };
    let end = match filepath.rfind('.') {
        None => {
            panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
        }
        Some(idx) => idx,
    };
    if start > 0 && end > 0 {
        let subpath = &filepath[start + 1..end];
        let charsize: Vec<&str> = subpath.split('x').collect();
        char_width = match charsize[0].parse::<u32>() {
            Err(_) => {
                panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
            }
            Ok(val) => val,
        };
        char_height = match charsize[1].parse::<u32>() {
            Err(_) => {
                panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
            }
            Ok(val) => val,
        };
    }
    (char_width, char_height)
}

fn process_image(img: &mut image::RgbaImage) {
    let pixel = img.get_pixel(0, 0);
    let alpha = pixel[3];
    if alpha == 255 {
        let transparent_color = (pixel[0], pixel[1], pixel[2]);
        let greyscale = transparent_color == (0, 0, 0);
        log(&format!(
            "{}transparent color: {:?}",
            if greyscale { "greyscale " } else { "" },
            transparent_color
        ));
        let (width, height) = img.dimensions();
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel_mut(x, y);
                if (pixel[0], pixel[1], pixel[2]) == transparent_color {
                    pixel[3] = 0;
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                } else if greyscale && pixel[0] == pixel[1] && pixel[1] == pixel[2] {
                    let alpha = pixel[0];
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = alpha;
                }
            }
        }
    }
}

fn create_font_texture(gl: &WebGLRenderingContext) -> WebGLTexture {
    let tex = gl.create_texture();
    gl.bind_texture(&tex);
    set_texture_params(gl, true);
    tex
}

pub struct Fonts {
    cache: HashMap<String, Arc<Font>>,
}

impl Fonts {
    pub fn new(gl: &uni_gl::WebGLRenderingContext) -> Self {
        let mut cache = HashMap::new();
        let sub_cell_font = Arc::new(Font::new(&gl, SUBCELL_BYTES, (4, 4)));
        let default_font = Arc::new(
            Font::new(&gl, TERMINAL_8X8_BYTES, (8, 8))
                .with_transforms(&codepage437::to_glyph, &codepage437::from_glyph),
        );
        cache.insert("SUBCELL".to_string(), sub_cell_font);
        cache.insert("DEFAULT".to_string(), default_font);

        Fonts { cache }
    }

    pub fn insert(&mut self, id: &str, font: Arc<Font>) {
        self.cache.insert(id.to_string(), font);
    }

    pub fn get(&self, id: &str) -> Option<Arc<Font>> {
        match self.cache.get(id) {
            None => None,
            Some(f) => Some(f.clone()),
        }
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Fonts {
            cache: HashMap::new(),
        }
    }
}

pub struct FontFileLoader {
    transforms: Option<(&'static ToGlyphFn, &'static FromGlyphFn)>,
}

impl FontFileLoader {
    pub fn new() -> FontFileLoader {
        FontFileLoader { transforms: None }
    }

    pub fn with_transforms(
        mut self,
        to_glyph: &'static ToGlyphFn,
        from_glyph: &'static FromGlyphFn,
    ) -> Self {
        self.transforms = Some((to_glyph, from_glyph));
        self
    }
}

impl LoadHandler for FontFileLoader {
    fn file_loaded(
        &mut self,
        path: &str,
        data: Vec<u8>,
        ecs: &mut Ecs,
    ) -> Result<(), crate::loader::LoadError> {
        let char_size = parse_char_size(path);

        let font = {
            let gl = ecs.read_global::<uni_gl::WebGLRenderingContext>();

            let mut font = Font::new(&*gl, &data, char_size);
            if let Some((to_glyph, from_glyph)) = self.transforms {
                font.set_transform(to_glyph, from_glyph);
            }
            Arc::new(font)
        };

        let mut fonts = ecs.write_global::<Fonts>();
        fonts.insert(path, font);

        log(format!("font load complete - {}", path));
        Ok(())
    }
}
