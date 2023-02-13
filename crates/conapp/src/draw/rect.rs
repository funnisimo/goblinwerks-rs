use crate::color::RGBA;
use crate::simple::{Buffer, Glyph};

pub fn rect<'a>(buffer: &'a mut Buffer) -> RectPrinter {
    RectPrinter::new(buffer)
}

pub struct RectPrinter<'a> {
    buffer: &'a mut Buffer,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    glyph: Option<Glyph>,
}

impl<'a> RectPrinter<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        RectPrinter {
            buffer,
            fg: None,
            bg: None,
            glyph: None,
        }
    }

    pub fn fg(mut self, fg: RGBA) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: RGBA) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn glyph(mut self, glyph: Glyph) -> Self {
        self.glyph = Some(glyph);
        self
    }

    pub fn draw(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let right = x + (width as i32);
        let down = y + (height as i32);

        let (buf_width, buf_height) = self.buffer.size();
        let buf_pot_width = self.buffer.pot_size().0;

        if let Some(fillchar) = self.glyph {
            for iy in y.max(0)..down.min(buf_height as i32) {
                let off = iy * buf_pot_width as i32;
                for ix in x.max(0)..right.min(buf_width as i32) {
                    self.buffer.glyphs_mut()[(off + ix) as usize] = u32::from(fillchar);
                }
            }
        }
        if let Some(fore) = self.fg {
            for iy in y.max(0)..down.min(buf_height as i32) {
                let off = iy * buf_pot_width as i32;
                for ix in x.max(0)..right.min(buf_width as i32) {
                    self.buffer.foregrounds_mut()[(off + ix) as usize] = fore;
                }
            }
        }
        if let Some(back) = self.bg {
            for iy in y.max(0)..down.min(buf_height as i32) {
                let off = iy * buf_pot_width as i32;
                for ix in x.max(0)..right.min(buf_width as i32) {
                    self.buffer.backgrounds_mut()[(off + ix) as usize] = back;
                }
            }
        }
    }
}
