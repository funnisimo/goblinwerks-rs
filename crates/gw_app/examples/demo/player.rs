use gw_app::{color::RGBA, AppInput, Buffer, VirtualKeyCode};

#[derive(Default)]
pub struct Player {
    pos: (f32, f32),
    speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Self {
            pos: (0.0, 0.0),
            speed,
        }
    }
    pub fn move_from_input(&self, input: &AppInput) -> (i32, i32) {
        let mut mov = (0, 0);
        if input.key(VirtualKeyCode::Left) || input.key(VirtualKeyCode::A) {
            mov.0 = -1;
        } else if input.key(VirtualKeyCode::Right) || input.key(VirtualKeyCode::D) {
            mov.0 = 1;
        }
        if input.key(VirtualKeyCode::Up) || input.key(VirtualKeyCode::W) {
            mov.1 = -1;
        } else if input.key(VirtualKeyCode::Down) || input.key(VirtualKeyCode::W) {
            mov.1 = 1;
        }
        mov
    }
    pub fn move_to(&mut self, pos: (i32, i32)) {
        self.pos = (pos.0 as f32, pos.1 as f32);
    }
    pub fn move_by(&mut self, mov: (i32, i32), coef: f32) -> bool {
        let oldx = self.pos.0 as i32;
        let oldy = self.pos.1 as i32;
        self.pos.0 += self.speed * mov.0 as f32 * coef;
        self.pos.1 += self.speed * mov.1 as f32 * coef;
        oldx == self.pos.0 as i32 && oldy == self.pos.1 as i32
    }
    pub fn next_pos(&self, mov: (i32, i32)) -> (i32, i32) {
        (self.pos.0 as i32 + mov.0, self.pos.1 as i32 + mov.1)
    }
    pub fn pos(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32)
    }
    pub fn render(&self, buffer: &mut Buffer, light: RGBA) {
        let pos = self.pos();
        buffer.glyph(pos.0, pos.1, '@' as u32);
        buffer.fore(pos.0, pos.1, light);
    }
}
