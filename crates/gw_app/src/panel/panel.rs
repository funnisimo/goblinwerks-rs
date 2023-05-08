use super::Buffer;
use super::PanelProgram;
use crate::font::{Font, Fonts};
use crate::log;
use gw_ecs::Ecs;
use gw_util::extents::Extents;
use gw_util::point::Point;
use std::sync::Arc;
use uni_gl::WebGLRenderingContext;

/// This contains the data for a console (including the one displayed on the screen) and methods to draw on it.
pub struct Panel {
    buffer: Buffer,
    extents: Extents,
    font_name: String,
    font: Option<Arc<Font>>,
    zpos: i8,
}

impl Panel {
    /// create a new offscreen console that you can draw to the screen with a font
    /// width and height are in cells (characters), not pixels.
    pub fn new(width: u32, height: u32, font_name: &str) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            extents: (0.0, 0.0, 1.0, 1.0).into(),
            font_name: font_name.to_owned(),
            font: None,
            zpos: 0,
        }
    }

    pub fn with_extents(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.set_extents(left, top, right, bottom);
        self
    }

    pub fn set_extents(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> &mut Self {
        println!("console extents = {},{} - {},{}", left, top, right, bottom);

        self.extents = (left, top, right, bottom).into();
        self
    }

    pub fn extents(&self) -> &Extents {
        &self.extents
    }

    pub fn is_full_screen(&self) -> bool {
        self.extents.0 == 0.0
            && self.extents.1 == 0.0
            && self.extents.2 == 1.0
            && self.extents.3 == 1.0
    }

    pub fn with_zpos(mut self, zpos: i8) -> Self {
        self.zpos = zpos;
        self
    }

    pub fn set_zpos(&mut self, zpos: i8) -> &mut Self {
        self.zpos = zpos;
        self
    }

    pub fn ready(&self) -> bool {
        self.font.is_some()
    }

    pub fn font_name(&self) -> &String {
        &self.font_name
    }

    pub fn set_font(&mut self, font: Arc<Font>) {
        self.buffer.set_to_glyph(font.to_glyph_fn);
        self.font = Some(font.clone());
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }
    pub fn height(&self) -> u32 {
        self.buffer.height()
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    // pub(crate) fn pot_width(&self) -> u32 {
    //     self.buffer.pot_size().0
    // }
    // pub(crate) fn pot_height(&self) -> u32 {
    //     self.buffer.pot_size().1
    // }

    pub fn font_char_size(&self) -> (u32, u32) {
        match self.font {
            None => (0, 0),
            Some(ref f) => f.char_size(),
        }
    }

    /// resizes the console
    pub fn resize(&mut self, width: u32, height: u32) {
        self.buffer.resize(width, height);
    }

    pub fn render(&mut self, ecs: &Ecs) {
        let fonts = ecs.read_global::<Fonts>();
        let gl = ecs.read_global::<WebGLRenderingContext>();
        let mut program = ecs.write_global::<PanelProgram>();

        if self.font.is_none() {
            let font = fonts.get(self.font_name.as_ref());
            if font.is_some() {
                log(format!("Got font - {}", self.font_name));
                self.buffer.set_to_glyph(font.as_ref().unwrap().to_glyph_fn);
                self.font = font;
            } else {
                log("Still missing font");
            }
        }

        match self.font {
            None => {}
            Some(ref font) => {
                program.use_font(&gl, &font);
                program.set_extents(&gl, &self.extents, self.zpos);
                program.render_buffer(&gl, &self.buffer);
            }
        }
    }

    pub fn contains_screen_pct(&self, screen_pct: (f32, f32)) -> bool {
        self.extents.contains(screen_pct)
    }

    /// returns the cell that the screen pos converts to for this console [0.0-1.0]
    pub fn mouse_pos(&self, screen_pct: (f32, f32)) -> Option<(f32, f32)> {
        if !self.extents.contains(screen_pct) {
            return None;
        }

        let cell_pct = (
            (screen_pct.0 - self.extents.0) / (self.extents.2 - self.extents.0),
            (screen_pct.1 - self.extents.1) / (self.extents.3 - self.extents.1),
        );

        Some((
            (cell_pct.0) * self.buffer.width() as f32,
            (cell_pct.1) * self.buffer.height() as f32,
        ))
    }

    pub fn mouse_point(&self, screen_pct: (f32, f32)) -> Option<Point> {
        match self.mouse_pos(screen_pct) {
            None => None,
            Some((x, y)) => Some(Point::new(x.floor() as i32, y.floor() as i32)),
        }
    }
}

impl From<(u32, u32)> for Panel {
    fn from(size: (u32, u32)) -> Self {
        Panel::new(size.0, size.1, "DEFAULT")
    }
}

impl From<(u32, u32, &str)> for Panel {
    fn from(config: (u32, u32, &str)) -> Self {
        Panel::new(config.0, config.1, config.2)
    }
}

pub fn subcell_console(width: u32, height: u32) -> Panel {
    Panel::new(width, height, "SUBCELL")
}

pub fn default_console(width: u32, height: u32) -> Panel {
    Panel::new(width, height, "DEFAULT")
}

pub fn calc_window_pct(
    screen_size_px: (u32, u32),
    char_size_px: (u32, u32),
    window_size_cells: (u32, u32),
) -> (f32, f32) {
    let window_size_px = (
        window_size_cells.0 * char_size_px.0,
        window_size_cells.1 * char_size_px.1,
    );
    (
        (window_size_px.0 as f32 / screen_size_px.0 as f32).min(1.0),
        (window_size_px.1 as f32 / screen_size_px.1 as f32).min(1.0),
    )
}

pub fn calc_center_offset(window_size_pct: (f32, f32)) -> (f32, f32) {
    (
        (1.0 - window_size_pct.0).max(0.0) / 2.0,
        (1.0 - window_size_pct.1).max(0.0) / 2.0,
    )
}

pub fn calc_center_extents(window_size_pct: (f32, f32)) -> (f32, f32, f32, f32) {
    let offset = calc_center_offset(window_size_pct);
    (
        offset.0,
        offset.1,
        offset.0 + window_size_pct.0,
        offset.1 + window_size_pct.1,
    )
}
