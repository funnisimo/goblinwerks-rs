use crate::fov::{FovFlags, FOV};
use crate::sprite::Sprite;
use gw_app::Buffer;
use gw_app::{Glyph, RGBA};

pub struct MapMemory {
    buffer: Buffer,
    flags: Vec<FovFlags>,
}

impl MapMemory {
    pub fn new(width: u32, height: u32) -> Self {
        MapMemory {
            buffer: Buffer::new(width, height),
            flags: vec![FovFlags::empty(); (width * height) as usize],
        }
    }

    pub fn set_sprite(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: Glyph) {
        self.buffer.draw_opt(x, y, Some(glyph), Some(fg), Some(bg));
    }

    pub fn get_sprite(&self, x: i32, y: i32) -> Option<Sprite> {
        if !self.buffer.has_xy(x, y) {
            return None;
        }

        let g = match self.buffer.get_glyph(x, y) {
            None => 0,
            Some(val) => *val,
        };
        let f = match self.buffer.get_fore(x, y) {
            None => RGBA::new(),
            Some(c) => *c,
        };
        let b = match self.buffer.get_back(x, y) {
            None => RGBA::new(),
            Some(c) => *c,
        };

        Some(Sprite::new(g, f, b))
    }

    pub fn store_flags(&mut self, fov: &FOV) {
        self.flags.copy_from_slice(&fov.flags);
    }

    pub fn restore_flags(&self, fov: &mut FOV) {
        fov.flags.copy_from_slice(&self.flags);
        fov.set_dirty();
    }
}
