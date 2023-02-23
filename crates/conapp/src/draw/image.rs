use crate::color::RGBA;
use crate::console::Buffer;
use crate::img::Image;

/// Constructs a new [`Blitter`] for this buffer
pub fn image(buffer: &mut Buffer) -> Blitter {
    Blitter::new(buffer)
}

/// An object that can draw images into a buffer
pub struct Blitter<'a> {
    buffer: &'a mut Buffer,
    transparent: Option<RGBA>,
}

impl<'a> Blitter<'a> {
    /// Constructs a new Blitter for this buffer
    pub fn new(buffer: &'a mut Buffer) -> Self {
        Blitter {
            buffer,
            transparent: None,
        }
    }

    /// Sets the color of the image that should be transparent when blitted, default=None
    pub fn transparent(mut self, color: RGBA) -> Self {
        self.transparent = Some(color);
        self
    }

    /// blit an image on a console
    ///
    /// x,y are the coordinate of the top left image pixel in the console
    ///
    /// image pixels using the transparent color will be ignored
    pub fn blit(&mut self, x: i32, y: i32, image: &Image) {
        let buf_size = self.buffer.size();

        let img = image.img();
        let width = img.width() as i32;
        let height = img.height() as i32;
        let minx = x.max(0);
        let miny = y.max(0);
        let maxx = (x + width).min(buf_size.0 as i32);
        let maxy = (y + height).min(buf_size.1 as i32);
        let offx = if x < 0 { -x } else { 0 };
        let offy = if y < 0 { -y } else { 0 };
        let con_width = self.buffer.pot_size().0;
        let back = self.buffer.backgrounds_mut();
        for cx in minx..maxx {
            for cy in miny..maxy {
                let pixel = img.get_pixel((cx - minx + offx) as u32, (cy - miny + offy) as u32);
                let color = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
                if let Some(ref t) = self.transparent {
                    if color == *t {
                        continue;
                    }
                }
                let offset = (cx as u32 + cy as u32 * con_width) as usize;
                back[offset] = color;
            }
        }
    }

    /// blit an image on a console
    ///
    /// x,y are the coordinate of the image center in the console
    /// image can be scaled and rotated (angle is in radians)
    /// image pixels using the transparent color will be ignored
    pub fn blit_ex(&mut self, x: f32, y: f32, scalex: f32, scaley: f32, angle: f32, image: &Image) {
        if scalex == 0.0 || scaley == 0.0 {
            return;
        }
        let size = image.size();
        let rx = x - size.0 as f32 * 0.5;
        let ry = y - size.1 as f32 * 0.5;
        if scalex == 1.0 && scaley == 1.0 && angle == 0.0 && rx.floor() == rx && ry.floor() == ry {
            let ix = rx as i32;
            let iy = ry as i32;
            self.blit(ix, iy, image);
            return;
        }

        let iw = (size.0 / 2) as f32 * scalex;
        let ih = (size.1 / 2) as f32 * scaley;
        // get the coordinates of the image corners in the console
        let newx_x = angle.cos();
        let newx_y = -angle.sin();
        let newy_x = newx_y;
        let newy_y = -newx_x;
        // image corners coordinates
        // 0 = P - w/2 x' +h/2 y'
        let x0 = x - iw * newx_x + ih * newy_x;
        let y0 = y - iw * newx_y + ih * newy_y;
        // 1 = P + w/2 x' + h/2 y'
        let x1 = x + iw * newx_x + ih * newy_x;
        let y1 = y + iw * newx_y + ih * newy_y;
        // 2 = P + w/2 x' - h/2 y'
        let x2 = x + iw * newx_x - ih * newy_x;
        let y2 = y + iw * newx_y - ih * newy_y;
        // 3 = P - w/2 x' - h/2 y'
        let x3 = x - iw * newx_x - ih * newy_x;
        let y3 = y - iw * newx_y - ih * newy_y;
        // get the affected rectangular area in the console
        let rx = x0.min(x1).min(x2).min(x3) as i32;
        let ry = y0.min(y1).min(y2).min(y3) as i32;
        let rw = x0.max(x1).max(x2).max(x3) as i32 - rx;
        let rh = y0.max(y1).max(y2).max(y3) as i32 - ry;
        // clip it
        let minx = rx.max(0);
        let miny = ry.max(0);
        let maxx = (rx + rw).min(self.buffer.width() as i32);
        let maxy = (ry + rh).min(self.buffer.height() as i32);
        let invscalex = 1.0 / scalex;
        let invscaley = 1.0 / scaley;
        let con_width = self.buffer.pot_size().0;
        let back = self.buffer.backgrounds_mut();
        let img = image.img();
        for cx in minx..maxx {
            for cy in miny..maxy {
                // map the console pixel to the image world
                let ix = (iw + (cx as f32 - x) * newx_x + (cy as f32 - y) * (-newy_x)) * invscalex;
                let iy = (ih + (cx as f32 - x) * (newx_y) - (cy as f32 - y) * newy_y) * invscaley;
                let color = if ix as i32 >= size.0 as i32
                    || ix < 0.0
                    || iy as i32 >= size.1 as i32
                    || iy < 0.0
                {
                    RGBA::rgba(0, 0, 0, 255)
                } else {
                    let pixel = img.get_pixel(ix as u32, iy as u32);
                    RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3])
                };
                if let Some(ref t) = self.transparent {
                    if color == *t {
                        continue;
                    }
                }
                let offset = (cx as u32 + cy as u32 * con_width) as usize;
                if scalex < 1.0 || scaley < 1.0 {
                    // todo mipmap
                }
                back[offset] = color;
            }
        }
    }
}
