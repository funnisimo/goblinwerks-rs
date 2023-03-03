use crate::color::RGBA;
use crate::console::Buffer;
use crate::img::Image;

// sub-pixel resolution kit

pub fn subcell<'a>(buffer: &'a mut Buffer) -> SubCell<'a> {
    SubCell::new(buffer)
}

// const FLAG_TO_ASCII: [i32; 8] = [
//     0,
//     CHAR_SUBP_NE as i32,
//     CHAR_SUBP_SW as i32,
//     CHAR_SUBP_DIAG as i32,
//     CHAR_SUBP_SE as i32,
//     CHAR_SUBP_E as i32,
//     -(CHAR_SUBP_N as i32),
//     -(CHAR_SUBP_NW as i32),
// ];

// These are the values from the SUBCELL font
const CHAR_NONE: i32 = 0;
const CHAR_SUBP_NW: i32 = 1;
const CHAR_SUBP_NE: i32 = 2;
const CHAR_SUBP_N: i32 = 3;
const CHAR_SUBP_SE: i32 = 4;
const CHAR_SUBP_DIAG: i32 = 5;
const CHAR_SUBP_E: i32 = 6;
const CHAR_SUBP_SW: i32 = 7;

/*
    pixels have following flag values :
        X 1
        2 4
    flag indicates which pixels uses foreground color (top left pixel always uses background color except if all pixels have the same color)
    negative values mean flip foreground and background when drawing glyph
*/
pub fn to_subcell_glyph(flag: u8) -> i32 {
    match flag {
        0 => CHAR_NONE,
        1 => CHAR_SUBP_NE,
        2 => CHAR_SUBP_SW,
        3 => CHAR_SUBP_DIAG,
        4 => CHAR_SUBP_SE,
        5 => CHAR_SUBP_E,
        6 => -CHAR_SUBP_N,
        7 => -CHAR_SUBP_NW,
        _ => CHAR_NONE,
    }
}

pub struct SubCell<'a> {
    buffer: &'a mut Buffer,
    transparent: Option<RGBA>,
    to_glyph: &'a dyn Fn(u8) -> i32,
}

impl<'a> SubCell<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        SubCell {
            buffer,
            transparent: None,
            to_glyph: &to_subcell_glyph,
        }
    }

    pub fn transparent(mut self, color: RGBA) -> Self {
        self.transparent = Some(color);
        self
    }

    pub fn to_glyph(mut self, func: &'a dyn Fn(u8) -> i32) -> Self {
        self.to_glyph = func;
        self
    }

    /// blit an image on the console, using the subcell characters to achieve twice the normal resolution.
    /// This uses the CHAR_SUBCELL_* ascii codes (from 226 to 232):
    ///
    /// ![subcell_chars](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell.png)
    ///
    /// Comparison before/after subcell in the chronicles of Doryen :
    ///
    /// ![subcell_comp](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_comp.png)
    ///
    /// Pyromancer! screenshot, making full usage of subcell resolution:
    ///
    /// ![subcell_pyro](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_pyro.png)
    pub fn blit(
        &mut self,
        img: &Image,
        dx: i32,
        dy: i32,
        sx: i32,
        sy: i32,
        w: Option<i32>,
        h: Option<i32>,
    ) {
        let img = img.img();
        let mut grid: [RGBA; 4] = [
            (0, 0, 0, 0).into(),
            (0, 0, 0, 0).into(),
            (0, 0, 0, 0).into(),
            (0, 0, 0, 0).into(),
        ];
        let mut back: RGBA = (0, 0, 0, 0).into();
        let mut front: Option<RGBA> = None;
        let mut ascii: i32 = ' ' as i32;
        let width = img.width() as i32;
        let height = img.height() as i32;
        let con_width = self.buffer.width() as i32;
        let con_height = self.buffer.height() as i32;
        let mut blit_w = w.unwrap_or(width);
        let mut blit_h = h.unwrap_or(height);
        let minx = sx.max(0);
        let miny = sy.max(0);
        blit_w = blit_w.min(width - minx);
        blit_h = blit_h.min(height - miny);
        let mut maxx = if dx + blit_w / 2 <= con_width {
            blit_w
        } else {
            (con_width - dx) * 2
        };
        let mut maxy = if dy + blit_h / 2 <= con_height {
            blit_h
        } else {
            (con_height - dy) * 2
        };
        maxx += minx;
        maxy += miny;
        let mut cx = minx;
        while cx < maxx {
            let mut cy = miny;
            while cy < maxy {
                // get the 2x2 super pixel colors from the image
                let conx = dx + (cx - minx) / 2;
                let cony = dy + (cy - miny) / 2;
                let console_back = self.buffer.get_back(conx, cony).unwrap().clone();
                let pixel = img.get_pixel(cx as u32, cy as u32);
                grid[0] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                if let Some(ref t) = self.transparent {
                    if grid[0] == *t {
                        grid[0] = console_back;
                    }
                }
                if cx < maxx - 1 {
                    let pixel = img.get_pixel(cx as u32 + 1, cy as u32);
                    grid[1] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                    if let Some(ref t) = self.transparent {
                        if grid[1] == *t {
                            grid[1] = console_back;
                        }
                    }
                } else {
                    grid[1] = console_back;
                }
                if cy < maxy - 1 {
                    let pixel = img.get_pixel(cx as u32, cy as u32 + 1);
                    grid[2] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                    if let Some(ref t) = self.transparent {
                        if grid[2] == *t {
                            grid[2] = console_back;
                        }
                    }
                } else {
                    grid[2] = console_back;
                }
                if cx < maxx - 1 && cy < maxy - 1 {
                    let pixel = img.get_pixel(cx as u32 + 1, cy as u32 + 1);
                    grid[3] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                    if let Some(ref t) = self.transparent {
                        if grid[3] == *t {
                            grid[3] = console_back;
                        }
                    }
                } else {
                    grid[3] = console_back;
                }
                // analyse color, posterize, get pattern
                compute_pattern(self.to_glyph, &grid, &mut back, &mut front, &mut ascii);
                if let Some(front) = front {
                    if ascii >= 0 {
                        let glyph = ascii as u32;
                        self.buffer.back(conx, cony, back);
                        self.buffer.fore(conx, cony, front);
                        self.buffer.glyph(conx, cony, glyph);
                    } else {
                        let glyph = -ascii as u32;
                        self.buffer.back(conx, cony, front);
                        self.buffer.fore(conx, cony, back);
                        self.buffer.glyph(conx, cony, glyph);
                    }
                } else {
                    // single color
                    self.buffer.back(conx, cony, back);
                    self.buffer.glyph(conx, cony, ascii as u32);
                }
                cy += 2;
            }
            cx += 2;
        }
    }
}

fn compute_pattern(
    to_glyph: &dyn Fn(u8) -> i32,
    desired: &[RGBA; 4],
    back: &mut RGBA,
    front: &mut Option<RGBA>,
    ascii: &mut i32,
) {
    // adapted from Jeff Lait's code posted on r.g.r.d
    let mut flag = 0;
    /*
        pixels have following flag values :
            X 1
            2 4
        flag indicates which pixels uses foreground color (top left pixel always uses foreground color except if all pixels have the same color)
    */
    let mut weight: [f32; 2] = [0.0, 0.0];
    // First colour trivial.
    *back = desired[0];

    // Ignore all duplicates...
    let mut i = 1;
    while i < 4 {
        if desired[i].0 != back.0 || desired[i].1 != back.1 || desired[i].2 != back.2 {
            break;
        }
        i += 1;
    }

    // All the same.
    if i == 4 {
        *front = None;
        *ascii = ' ' as i32;
        return;
    }
    weight[0] = i as f32;

    // Found a second colour...
    let mut tmp_front = desired[i];
    weight[1] = 1.0;
    flag |= 1 << (i - 1);
    // remaining colours
    i += 1;
    while i < 4 {
        if desired[i].0 == back.0 && desired[i].1 == back.1 && desired[i].2 == back.2 {
            weight[0] += 1.0;
        } else if desired[i].0 == tmp_front.0
            && desired[i].1 == tmp_front.1
            && desired[i].2 == tmp_front.2
        {
            flag |= 1 << (i - 1);
            weight[1] += 1.0;
        } else {
            // Bah, too many colours,
            // merge the two nearest
            let dist0i = RGBA::distance(&desired[i], back);
            let dist1i = RGBA::distance(&desired[i], &tmp_front);
            let dist01 = RGBA::distance(back, &tmp_front);
            if dist0i < dist1i {
                if dist0i <= dist01 {
                    // merge 0 and i
                    *back = RGBA::blend(&desired[i], back, weight[0] / (1.0 + weight[0]));
                    weight[0] += 1.0;
                } else {
                    // merge 0 and 1
                    *back = RGBA::blend(back, &tmp_front, weight[1] / (weight[0] + weight[1]));
                    weight[0] += 1.0;
                    tmp_front = desired[i];
                    flag = 1 << (i - 1);
                }
            } else if dist1i <= dist01 {
                // merge 1 and i
                tmp_front = RGBA::blend(&desired[i], &tmp_front, weight[1] / (1.0 + weight[1]));
                weight[1] += 1.0;
                flag |= 1 << (i - 1);
            } else {
                // merge 0 and 1
                *back = RGBA::blend(back, &tmp_front, weight[1] / (weight[0] + weight[1]));
                weight[0] += 1.0;
                tmp_front = desired[i];
                flag = 1 << (i - 1);
            }
        }
        i += 1;
    }
    *front = Some(tmp_front);
    *ascii = to_glyph(flag as u8);
}
