use conapp::{color::RGBA, Buffer};

use crate::level::Level;
use crate::light::{Light, LIGHT_COEF};

pub struct Entity {
    /// ascii character for this entity
    ch: u32,
    /// position on the map (cell coordinate)
    pub pos: (i32, i32),
    pub name: String,
    color: RGBA,
    light: bool,
}

impl Entity {
    pub fn new_goblin(pos: (i32, i32)) -> Self {
        Self {
            ch: 'g' as u32,
            pos,
            name: "a petrified goblin".to_owned(),
            color: (80, 150, 70, 255).into(),
            light: false,
        }
    }
    pub fn new_light(pos: (i32, i32)) -> Self {
        Self {
            ch: 15,
            pos,
            name: "a flickering torch".to_owned(),
            color: (150, 174, 27, 255).into(),
            light: true,
        }
    }
    pub fn render(&self, buffer: &mut Buffer, level: &Level) {
        let (color, penumbra) = if self.light {
            (self.color, false)
        } else {
            let light = level.light_at(self.pos);
            let penumbra = Light::is_penumbra(light, 100);
            let mut color = RGBA::multiply(self.color, light);
            if penumbra {
                color = RGBA::scale(color, LIGHT_COEF);
            }
            (color, penumbra)
        };
        buffer.glyph(
            self.pos.0,
            self.pos.1,
            if penumbra { '?' as u32 } else { self.ch },
        );
        buffer.fore(self.pos.0, self.pos.1, color);
    }
}
