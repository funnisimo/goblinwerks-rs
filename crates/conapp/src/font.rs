// use crate::Buffer;
use uni_gl::{WebGLRenderingContext, WebGLTexture};

use crate::{console, simple::set_texture_params};

pub struct Font {
    img_size: (u32, u32),
    char_size: (u32, u32),
    count: u32,
    pub(crate) texture: WebGLTexture,
}

impl Font {
    pub fn new(gl: &WebGLRenderingContext, bytes: &[u8], char_size: (u32, u32)) -> Self {
        let mut font = Font {
            img_size: (0, 0),
            char_size,
            count: 0,
            texture: create_font_texture(gl),
        };

        font.load_font_img(bytes, gl);
        font
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

    fn load_font_img(&mut self, buf: &[u8], gl: &WebGLRenderingContext) {
        console(format!("load font image - {}", buf.len()));
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
        crate::console(&format!(
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
