use crate::color::RGBA;
use crate::simple::Buffer;

pub fn progress<'a>(buffer: &'a mut Buffer) -> Progress {
    Progress::new(buffer)
}

pub struct Progress<'a> {
    buffer: &'a mut Buffer,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    width: i32,
}

impl<'a> Progress<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        Progress {
            buffer,
            fg: None,
            bg: None,
            width: 0,
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

    pub fn width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    // TODO - Add some alpha blending in transition cell
    pub fn draw(&mut self, x: i32, y: i32, val: i32, max: i32) {
        let percent = val as f32 / max as f32;
        let fill_width = (percent * self.width as f32) as i32;
        for cx in 0..self.width {
            if cx <= fill_width {
                if let Some(ref fg) = self.fg {
                    self.buffer.back(x + cx, y, *fg);
                }
            } else {
                if let Some(ref bg) = self.bg {
                    self.buffer.back(x + cx, y, *bg);
                }
            }
        }
    }
}
