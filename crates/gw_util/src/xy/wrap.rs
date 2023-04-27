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
    pub fn try_wrap_x(&self, x: i32, left: i32, right: i32) -> Option<i32> {
        match self {
            Wrap::None | Wrap::Y => {
                if x < left || x > right {
                    return None;
                }
                Some(x)
            }
            _ => {
                let width = (right - left) + 1;
                let mut tx = x;
                while tx < left {
                    tx += width;
                }

                Some((tx - left) % width + left)
            }
        }
    }

    pub fn try_wrap_y(&self, y: i32, top: i32, bottom: i32) -> Option<i32> {
        match self {
            Wrap::None | Wrap::X => {
                if y < top || y > bottom {
                    return None;
                }
                Some(y)
            }
            _ => {
                let height = (bottom - top) + 1;
                let mut ty = y;
                while ty < top {
                    ty += height;
                }
                Some((ty - top) % height + top)
            }
        }
    }

    pub fn try_wrap_in_rect(&self, x: i32, y: i32, rect: &Rect) -> Option<(i32, i32)> {
        let x0 = match self.try_wrap_x(x, rect.left(), rect.right()) {
            None => return None,
            Some(x) => x,
        };

        let y0 = match self.try_wrap_y(y, rect.top(), rect.bottom()) {
            None => return None,
            Some(y) => y,
        };

        Some((x0, y0))
    }

    pub fn try_wrap(&self, x: i32, y: i32, width: u32, height: u32) -> Option<(i32, i32)> {
        match self.try_wrap_x(x, 0, width as i32 - 1) {
            None => None,
            Some(x) => match self.try_wrap_y(y, 0, height as i32 - 1) {
                None => None,
                Some(y) => Some((x, y)),
            },
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrap_rect() {
        let wrap = Wrap::XY;

        let rect = Rect::with_bounds(0, 0, 9, 9);

        assert_eq!(wrap.try_wrap_in_rect(5, 10, &rect).unwrap(), (5, 0));
        assert_eq!(wrap.try_wrap_in_rect(0, 9, &rect).unwrap(), (0, 9));
        assert_eq!(wrap.try_wrap_in_rect(10, 19, &rect).unwrap(), (0, 9));
        assert_eq!(wrap.try_wrap_in_rect(20, -9, &rect).unwrap(), (0, 1));
        assert_eq!(wrap.try_wrap_in_rect(-1, -19, &rect).unwrap(), (9, 1));
    }

    #[test]
    fn try_wrap() {
        let wrap = Wrap::XY;

        assert_eq!(wrap.try_wrap(5, 10, 10, 10).unwrap(), (5, 0));
        assert_eq!(wrap.try_wrap(0, 9, 10, 10).unwrap(), (0, 9));
        assert_eq!(wrap.try_wrap(10, 19, 10, 10).unwrap(), (0, 9));
        assert_eq!(wrap.try_wrap(20, -9, 10, 10).unwrap(), (0, 1));
        assert_eq!(wrap.try_wrap(-1, -19, 10, 10).unwrap(), (9, 1));
    }
}
