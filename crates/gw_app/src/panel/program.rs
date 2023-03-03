use super::Buffer;
use crate::color::RGBA;
use crate::font::Font;
// use image::{ImageBuffer, Rgba};
use crate::log;
use gw_util::extents::Extents;
use std::collections::HashMap;
use std::mem::size_of;
use std::slice;
use uni_gl::{
    AttributeSize, BufferKind, DataType, DrawMode, PixelFormat, PixelType, Primitives, ShaderKind,
    TextureBindPoint, TextureKind, TextureMagFilter, TextureMinFilter, TextureParameter,
    TextureWrap, WebGLBuffer, WebGLProgram, WebGLRenderingContext, WebGLShader, WebGLTexture,
    WebGLUniformLocation, WebGLVertexArray, IS_GL_ES,
};

// shaders
pub const DORYEN_VS: &str = include_str!("doryen_vs.glsl");
pub const DORYEN_FS: &str = include_str!("doryen_fs.glsl");

const FONT_TEXTURE: u32 = 0;
const GLYPH_TEXTURE: u32 = 1;
const FG_TEXTURE: u32 = 2;
const BG_TEXTURE: u32 = 3;

#[derive(Debug)]
pub struct PrimitiveData {
    pub count: usize,
    pub data_per_primitive: usize,
    pub pos_data: Vec<f32>,
    pub tex_data: Vec<f32>,
    pub draw_mode: Primitives,
}

impl PrimitiveData {
    pub fn new() -> PrimitiveData {
        PrimitiveData {
            count: 0,
            data_per_primitive: 0,
            pos_data: Vec::new(),
            tex_data: Vec::new(),
            draw_mode: Primitives::Triangles,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum DoryenUniforms {
    Font,
    Ascii,
    Foreground,
    Background,
    FontCharsPerLine,
    FontCoef,
    TermSize,
    TermCoef,
    ZPos,
}

pub struct PanelProgram {
    pub(crate) program: WebGLProgram,
    pub(crate) vao: WebGLVertexArray,
    pub(crate) vertex_pos_location: Option<u32>,
    pub(crate) vertex_uv_location: Option<u32>,
    pub(crate) vertex_pos_buffer: Option<WebGLBuffer>,
    pub(crate) vertex_uv_buffer: Option<WebGLBuffer>,
    // pub(crate) font: WebGLTexture,
    pub(crate) ascii: WebGLTexture,
    pub(crate) foreground: WebGLTexture,
    pub(crate) background: WebGLTexture,
    pub(crate) uniform_locations: HashMap<DoryenUniforms, Option<WebGLUniformLocation>>,
    pub(crate) data: PrimitiveData,
}

trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl<T> IntoBytes for Vec<T> {
    fn into_bytes(self) -> Vec<u8> {
        let len = size_of::<T>() * self.len();
        unsafe {
            let slice = self.into_boxed_slice();
            Vec::<u8>::from_raw_parts(Box::into_raw(slice) as _, len, len)
        }
    }
}

fn compile_shader(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    let shader = gl.create_shader(shader_kind);
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    shader
}

fn compile_shader_wasm_native(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    if IS_GL_ES {
        compile_shader(gl, shader_kind, &("#version 300 es\n".to_string() + source))
    } else {
        compile_shader(gl, shader_kind, &("#version 150\n".to_string() + source))
    }
}

fn create_program(
    gl: &WebGLRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> WebGLProgram {
    log("compiling VS");
    let vert_shader = compile_shader_wasm_native(gl, ShaderKind::Vertex, vertex_source);
    log("compiling FS");
    let frag_shader = compile_shader_wasm_native(gl, ShaderKind::Fragment, fragment_source);
    log("linking Program");
    let shader_program = gl.create_program();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);
    shader_program
}

impl PanelProgram {
    pub fn new(gl: &WebGLRenderingContext) -> PanelProgram {
        println!("Create program");
        let data = create_primitive();
        let shader_program = create_program(gl, DORYEN_VS, DORYEN_FS);
        let vao = gl.create_vertex_array();
        let vertex_pos_location = gl.get_attrib_location(&shader_program, "aVertexPosition");
        let vertex_pos_buffer = vertex_pos_location.and(Some(gl.create_buffer()));
        let vertex_uv_location = gl.get_attrib_location(&shader_program, "aTextureCoord");
        let vertex_uv_buffer = vertex_uv_location.and(Some(gl.create_buffer()));
        let mut uniform_locations = HashMap::new();
        for uniform in [
            (DoryenUniforms::Font, "uFont"),
            (DoryenUniforms::Ascii, "uAscii"),
            (DoryenUniforms::Background, "uBack"),
            (DoryenUniforms::Foreground, "uFront"),
            (DoryenUniforms::FontCharsPerLine, "uFontCharsPerLine"),
            (DoryenUniforms::FontCoef, "uFontCoef"),
            (DoryenUniforms::TermCoef, "uTermCoef"),
            (DoryenUniforms::TermSize, "uTermSize"),
            (DoryenUniforms::ZPos, "uZPos"), // If using ZPos
        ]
        .iter()
        {
            uniform_locations.insert(
                uniform.0,
                gl.get_uniform_location(&shader_program, uniform.1),
            );
        }

        PanelProgram {
            program: shader_program,
            vao,
            vertex_pos_location,
            vertex_uv_location,
            vertex_pos_buffer,
            vertex_uv_buffer,
            // font: create_font_texture(gl),
            ascii: gl.create_texture(),
            foreground: gl.create_texture(),
            background: gl.create_texture(),
            uniform_locations,
            data,
        }
    }

    pub fn set_extents(&mut self, gl: &WebGLRenderingContext, extents: &Extents, zpos: i8) {
        let left = (extents.0 * 2.0) - 1.0;
        let top = 1.0 - (extents.1 * 2.0);
        let right = (extents.2 * 2.0) - 1.0;
        let bottom = 1.0 - (extents.3 * 2.0);

        self.data.pos_data[0] = left;
        self.data.pos_data[1] = bottom;
        self.data.pos_data[2] = left;
        self.data.pos_data[3] = top;
        self.data.pos_data[4] = right;
        self.data.pos_data[5] = top;
        self.data.pos_data[6] = right;
        self.data.pos_data[7] = bottom;

        // println!("{:?}", self.data.pos_data);

        gl.use_program(&self.program);
        gl.bind_vertex_array(&self.vao);

        if let Some(ref buf) = self.vertex_pos_buffer {
            if let Some(ref loc) = self.vertex_pos_location {
                // println!("set extents - {:?}", self.data.pos_data);
                set_buffer_data(gl, buf, &self.data.pos_data, *loc, AttributeSize::Two);
            }
        }

        if let Some(ref buf) = self.vertex_uv_buffer {
            if let Some(ref loc) = self.vertex_uv_location {
                set_buffer_data(gl, buf, &self.data.tex_data, *loc, AttributeSize::Two);
            }
        }

        // If using ZPos
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::ZPos) {
            let zpos: f32 = zpos as f32 / 128.0;
            // println!("zpos = {}", zpos);
            gl.uniform_1f(location, zpos);
        }
    }

    pub(crate) fn use_font(&mut self, gl: &WebGLRenderingContext, font: &Font) {
        gl.use_program(&self.program);
        gl.active_texture(FONT_TEXTURE);
        gl.bind_texture(&font.texture);

        if let Some(&Some(ref location)) = self
            .uniform_locations
            .get(&DoryenUniforms::FontCharsPerLine)
        {
            gl.uniform_1f(
                location,
                (font.img_width() as f32) / (font.char_width() as f32),
            );
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::FontCoef) {
            gl.uniform_2f(
                location,
                (
                    (font.char_width() as f32) / (font.img_width() as f32),
                    (font.char_height() as f32) / (font.img_height() as f32),
                ),
            );
        }

        if let Some(&Some(ref sampler_location)) = self.uniform_locations.get(&DoryenUniforms::Font)
        {
            gl.uniform_1i(sampler_location, FONT_TEXTURE as i32);
        }
    }

    pub fn render_buffer(&mut self, gl: &WebGLRenderingContext, buffer: &Buffer) {
        gl.use_program(&self.program);
        self.set_uniforms(gl, buffer);

        // bind font texture
        // gl.active_texture(FONT_TEXTURE);
        // gl.bind_texture(&self.font);

        gl.bind_vertex_array(&self.vao);
        if let Some(ref buf) = self.vertex_pos_buffer {
            if let Some(ref loc) = self.vertex_pos_location {
                // println!("render primitive - {:?}", self.data.pos_data);
                set_buffer_data(gl, buf, &self.data.pos_data, *loc, AttributeSize::Two);
            }
        }

        let (pot_width, pot_height) = buffer.pot_size();
        let con_width = buffer.width();
        let con_height = buffer.height();

        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermSize) {
            gl.uniform_2f(location, (con_width as f32, con_height as f32));
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermCoef) {
            gl.uniform_2f(
                location,
                (1.0 / (pot_width as f32), 1.0 / (pot_height as f32)),
            );
        }

        gl.draw_arrays(
            self.data.draw_mode,
            self.data.count * self.data.data_per_primitive,
        );
    }

    fn update_uniform_texture(
        &mut self,
        gl: &WebGLRenderingContext,
        uniform: DoryenUniforms,
        tex_num: u32,
        tex: &WebGLTexture,
        data: &[u8],
        pot_width: u32,
        pot_height: u32,
    ) {
        if let Some(&Some(ref location)) = self.uniform_locations.get(&uniform) {
            // let index = self.index * 4 + tex_num;
            gl.active_texture(tex_num);
            gl.bind_texture(tex);
            gl.tex_image2d(
                TextureBindPoint::Texture2d, // target
                0,                           // level
                pot_width as u16,            // width
                pot_height as u16,           // height
                PixelFormat::Rgba,           // format
                PixelType::UnsignedByte,     // type
                data,                        // data
            );
            set_texture_params(gl, true);
            gl.uniform_1i(location, tex_num as i32);
        }
    }

    pub fn set_uniforms(&mut self, gl: &WebGLRenderingContext, buffer: &Buffer) {
        gl.use_program(&self.program);
        let (pot_width, pot_height) = buffer.pot_size();
        let ascii_tex = WebGLTexture(self.ascii.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Ascii,
            GLYPH_TEXTURE,
            &ascii_tex,
            u32_to_u8(&buffer.glyphs()[..]),
            pot_width,
            pot_height,
        );
        let fore_tex = WebGLTexture(self.foreground.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Foreground,
            FG_TEXTURE,
            &fore_tex,
            color_to_u8(&buffer.foregrounds()[..]),
            pot_width,
            pot_height,
        );
        let back_tex = WebGLTexture(self.background.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Background,
            BG_TEXTURE,
            &back_tex,
            color_to_u8(&buffer.backgrounds()[..]),
            pot_width,
            pot_height,
        );
    }
}

fn set_buffer_data(
    gl: &WebGLRenderingContext,
    buffer: &WebGLBuffer,
    data: &Vec<f32>,
    attribute_location: u32,
    count_per_vertex: AttributeSize,
) {
    gl.bind_buffer(BufferKind::Array, buffer);
    gl.enable_vertex_attrib_array(attribute_location);
    gl.buffer_data(
        BufferKind::Array,
        &data.clone().into_bytes(),
        DrawMode::Stream,
    );
    gl.vertex_attrib_pointer(
        attribute_location,
        count_per_vertex,
        DataType::Float,
        false,
        0,
        0,
    );
}

fn u32_to_u8(v: &[u32]) -> &[u8] {
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * size_of::<u32>()) }
}

fn color_to_u8(v: &[RGBA]) -> &[u8] {
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * size_of::<RGBA>()) }
}

pub fn set_texture_params(gl: &WebGLRenderingContext, nearest: bool) {
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMagFilter,
        if nearest {
            TextureMagFilter::Nearest
        } else {
            TextureMagFilter::Linear
        } as i32,
    );
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMinFilter,
        if nearest {
            TextureMinFilter::Nearest
        } else {
            TextureMinFilter::Linear
        } as i32,
    );
    let wrap = TextureWrap::ClampToEdge as i32;
    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapS, wrap);
    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapT, wrap);
}

fn _get_pot_value(value: u32) -> u32 {
    let mut pot_value = 1;
    while pot_value < value {
        pot_value *= 2;
    }
    pot_value
}

fn create_primitive() -> PrimitiveData {
    let left = -1.0;
    let top = -1.0;
    let right = 1.0;
    let bottom = 1.0;

    // println!(
    //     "- create primitive at - {},{} - {},{}",
    //     left, top, right, bottom
    // );

    let mut data = PrimitiveData::new();
    data.pos_data.push(left);
    data.pos_data.push(top);
    data.pos_data.push(left);
    data.pos_data.push(bottom);
    data.pos_data.push(right);
    data.pos_data.push(bottom);
    data.pos_data.push(right);
    data.pos_data.push(top);

    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(0.0);
    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(1.0);

    data.count = 4;
    data.data_per_primitive = 1;
    data.draw_mode = Primitives::TriangleFan;

    data
}
