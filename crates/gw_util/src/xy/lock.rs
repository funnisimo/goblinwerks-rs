#[derive(Debug, Copy, Clone, Default)]
pub enum Lock {
    #[default]
    None,
    X,
    Y,
    XY,
}

impl Lock {
    pub fn lock_x(&self, x: i32, width: u32, map_width: u32) -> i32 {
        match self {
            Lock::None | Lock::Y => x,
            _ => {
                if x < 0 {
                    return 0;
                }
                if x + width as i32 >= map_width as i32 {
                    return map_width.saturating_sub(width) as i32;
                }
                x
            }
        }
    }

    pub fn lock_y(&self, y: i32, height: u32, map_height: u32) -> i32 {
        match self {
            Lock::None | Lock::X => y,
            _ => {
                if y < 0 {
                    return 0;
                }
                if y + height as i32 >= map_height as i32 {
                    return map_height.saturating_sub(height) as i32;
                }
                y
            }
        }
    }

    pub fn lock(
        &self,
        (x, y): (i32, i32),
        view_size: (u32, u32),
        map_size: (u32, u32),
    ) -> (i32, i32) {
        let x0 = self.lock_x(x, view_size.0, map_size.0);
        let y0 = self.lock_y(y, view_size.1, map_size.1);
        (x0, y0)
    }
}
