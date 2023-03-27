use crate::rect::Rect;

#[derive(Debug, Copy, Clone, Default)]
pub enum Wrap {
    #[default]
    None,
    X,
    Y,
    XY,
}

impl Wrap {
    pub fn try_wrap_x(&self, x: i32, rect: &Rect) -> Option<i32> {
        match self {
            Wrap::None | Wrap::Y => {
                if x < rect.left() || x > rect.right() {
                    return None;
                }
                Some(x)
            }
            _ => {
                let mut tx = x;
                while tx < rect.left() {
                    tx += rect.width() as i32;
                }

                Some((tx - rect.left()) % rect.width() as i32 + rect.left())
            }
        }
    }

    pub fn try_wrap_y(&self, y: i32, rect: &Rect) -> Option<i32> {
        match self {
            Wrap::None | Wrap::X => {
                if y < rect.top() || y > rect.bottom() {
                    return None;
                }
                Some(y)
            }
            _ => {
                let mut ty = y;
                while ty < rect.top() {
                    ty += rect.height() as i32;
                }
                Some((ty - rect.top()) % rect.height() as i32 + rect.top())
            }
        }
    }

    pub fn try_wrap(&self, x: i32, y: i32, rect: &Rect) -> Option<(i32, i32)> {
        let x0 = match self.try_wrap_x(x, rect) {
            None => return None,
            Some(x) => x,
        };

        let y0 = match self.try_wrap_y(y, rect) {
            None => return None,
            Some(y) => y,
        };

        Some((x0, y0))
    }

    pub fn wrap_x(&self, x: i32, width: u32) -> i32 {
        match self {
            Wrap::None | Wrap::Y => x,
            _ => {
                let mut tx = x;
                while tx < 0 {
                    tx += width as i32;
                }
                tx % width as i32
            }
        }
    }

    pub fn wrap_y(&self, y: i32, height: u32) -> i32 {
        match self {
            Wrap::None | Wrap::X => y,
            _ => {
                let mut ty = y;
                while ty < 0 {
                    ty += height as i32;
                }
                ty % height as i32
            }
        }
    }

    pub fn wrap(&self, x: i32, y: i32, width: u32, height: u32) -> (i32, i32) {
        let x0 = self.wrap_x(x, width);
        let y0 = self.wrap_y(y, height);
        (x0, y0)
    }
}
