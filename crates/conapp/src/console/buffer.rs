use crate::color::RGBA;

pub type Glyph = u32;

// #[derive(Copy, Clone)]
// pub enum TextAlign {
//     Left,
//     Right,
//     Center,
// }

// rectangle drawing kit
// pub const CHAR_CORNER_NW: u32 = 218;
// pub const CHAR_CORNER_SW: u32 = 192;
// pub const CHAR_CORNER_SE: u32 = 217;
// pub const CHAR_CORNER_NE: u32 = 191;
// pub const CHAR_LINE_H: u32 = 196;
// pub const CHAR_LINE_V: u32 = 179;

/// This contains the data for a console (including the one displayed on the screen) and methods to draw on it.
pub struct Buffer {
    width: u32,
    height: u32,
    // power of 2 size (for textures)
    pot_width: u32,
    pot_height: u32,
    glyph: Vec<u32>,
    back: Vec<RGBA>,
    fore: Vec<RGBA>,
    // colors: HashMap<String, RGBA>,
    // color_stack: Vec<RGBA>,
}

impl Buffer {
    /// create a new offscreen console that you can blit on another console
    /// width and height are in cells (characters), not pixels.
    pub fn new(width: u32, height: u32) -> Self {
        let mut back = Vec::new();
        let mut fore = Vec::new();
        let mut glyph = Vec::new();
        let mut pot_width = 1;
        let mut pot_height = 1;
        while pot_width < width {
            pot_width *= 2;
        }
        while pot_height < height {
            pot_height *= 2;
        }
        for _ in 0..(pot_width * pot_height) as usize {
            back.push(RGBA::rgba(0, 0, 0, 0));
            fore.push(RGBA::rgba(255, 255, 255, 0));
            glyph.push(0);
        }

        Self {
            width,
            height,
            glyph,
            back,
            fore,
            pot_width,
            pot_height,
            // colors: HashMap::new(),
            // color_stack: Vec::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pot_size(&self) -> (u32, u32) {
        (self.pot_width, self.pot_height)
    }

    /// resizes the console
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let mut pot_width = 1;
        let mut pot_height = 1;
        while pot_width < width {
            pot_width *= 2;
        }
        while pot_height < height {
            pot_height *= 2;
        }
        self.pot_height = pot_height;
        self.pot_width = pot_width;
        self.back.clear();
        self.fore.clear();
        self.glyph.clear();
        for _ in 0..(pot_width * pot_height) as usize {
            self.back.push(RGBA::rgba(0, 0, 0, 255));
            self.fore.push(RGBA::rgba(255, 255, 255, 255));
            self.glyph.push(' ' as u32);
        }
    }

    /// for fast reading of the characters values
    pub fn glyphs(&self) -> &Vec<u32> {
        &self.glyph
    }
    /// for fast reading of the characters colors
    pub fn foregrounds(&self) -> &Vec<RGBA> {
        &self.fore
    }
    /// for fast reading of the background colors
    pub fn backgrounds(&self) -> &Vec<RGBA> {
        &self.back
    }
    /// for fast writing of the characters values
    pub(crate) fn glyphs_mut(&mut self) -> &mut Vec<u32> {
        &mut self.glyph
    }
    /// for fast writing of the characters colors
    pub(crate) fn foregrounds_mut(&mut self) -> &mut Vec<RGBA> {
        &mut self.fore
    }
    /// for fast writing of the background colors
    pub(crate) fn backgrounds_mut(&mut self) -> &mut Vec<RGBA> {
        &mut self.back
    }

    /// get the background color of a cell (if x,y inside the console)
    pub fn get_back(&self, x: i32, y: i32) -> Option<&RGBA> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => self.back.get(idx),
        }
    }
    /// get the foreground color of a cell (if x,y inside the console)
    pub fn get_fore(&self, x: i32, y: i32) -> Option<&RGBA> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => self.fore.get(idx),
        }
    }
    /// get the glyph code of a cell (if x,y inside the console)
    pub fn get_glyph(&self, x: i32, y: i32) -> Option<&u32> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => self.glyph.get(idx),
        }
    }

    fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32 {
            return None;
        }
        Some(x as usize + y as usize * self.pot_size().0 as usize)
    }

    pub fn has_xy(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(_) => true,
        }
    }

    /// set the character at a specific position (doesn't change the color).
    pub fn glyph(&mut self, x: i32, y: i32, glyph: Glyph) {
        if let Some(idx) = self.to_idx(x, y) {
            self.glyph[idx] = glyph;
        }
    }
    /// set the character color at a specific position
    pub fn fore(&mut self, x: i32, y: i32, col: RGBA) {
        if let Some(idx) = self.to_idx(x, y) {
            self.fore[idx] = col;
        }
    }
    /// set the background color at a specific position
    pub fn back(&mut self, x: i32, y: i32, col: RGBA) {
        if let Some(idx) = self.to_idx(x, y) {
            self.back[idx] = col;
        }
    }

    pub fn update<F>(&mut self, func: F)
    where
        F: Fn(i32, i32, &mut Glyph, &mut RGBA, &mut RGBA) -> (),
    {
        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                let idx = self.to_idx(x, y).unwrap();
                let g = self.glyph.get_mut(idx).unwrap();
                let fg = self.fore.get_mut(idx).unwrap();
                let bg = self.back.get_mut(idx).unwrap();
                func(x, y, g, fg, bg);
            }
        }
    }

    pub fn clear(&mut self, glyph: bool, fore: bool, back: bool) {
        let fg = match fore {
            true => Some(RGBA::new()),
            false => None,
        };
        let bg = match back {
            true => Some(RGBA::new()),
            false => None,
        };
        let gl = match glyph {
            true => Some(0),
            false => None,
        };
        self.fill(gl, fg, bg);
    }

    /// fill the whole console with values
    pub fn fill(&mut self, fillglyph: Option<Glyph>, fore: Option<RGBA>, back: Option<RGBA>) {
        let w = self.width();
        let h = self.height();
        self.area(0, 0, w, h, fillglyph, fore, back);
    }

    // /// write a multi-color string. Foreground color is defined by #[color_name] patterns inside the string.
    // /// color_name must have been registered with [`Console::register_color`] before.
    // /// Default foreground color is white, at the start of the string.
    // /// When an unknown color name is used, the color goes back to its previous value.
    // /// You can then use an empty name to end a color span.
    // /// Example
    // /// ```
    // /// use doryen_rs::{Console, TextAlign};
    // /// let mut con=Console::new(80,25);
    // /// con.register_color("pink", (255, 0, 255, 255));
    // /// con.register_color("blue", (0, 0, 255, 255));
    // /// con.print_color(5, 5, "#[blue]This blue text contains a #[pink]pink#[] word", TextAlign::Left, None);
    // /// ```
    // pub fn print_color(
    //     &mut self,
    //     x: i32,
    //     y: i32,
    //     text: &str,
    //     align: TextAlign,
    //     back: Option<RGBA>,
    // ) {
    //     let mut cury = y;
    //     for line in text.to_owned().split('\n') {
    //         self.print_line_color(x, cury, line, align, back);
    //         cury += 1;
    //     }
    // }

    // pub fn get_color_spans(&mut self, text: &str, text_len: &mut i32) -> Vec<(RGBA, String)> {
    //     let mut spans: Vec<(RGBA, String)> = Vec::new();
    //     let mut color_stack: Vec<RGBA> = Vec::new();

    //     *text_len = 0;
    //     let mut fore = *color_stack
    //         .last()
    //         .unwrap_or(&RGBA::rgba(255, 255, 255, 255));
    //     for color_span in text.to_owned().split("#[") {
    //         if color_span.is_empty() {
    //             continue;
    //         }
    //         let mut col_text = color_span.splitn(2, ']');
    //         let col_name = col_text.next().unwrap();
    //         if let Some(text_span) = col_text.next() {
    //             if let Some(color) = get_color(col_name) {
    //                 fore = color;
    //                 color_stack.push(fore);
    //             } else {
    //                 color_stack.pop();
    //                 fore = *color_stack
    //                     .last()
    //                     .unwrap_or(&RGBA::rgba(255, 255, 255, 255));
    //             }
    //             spans.push((fore, text_span.to_owned()));
    //             *text_len += text_span.chars().count() as i32;
    //         } else {
    //             spans.push((fore, col_name.to_owned()));
    //             *text_len += col_name.chars().count() as i32;
    //         }
    //     }
    //     spans
    // }

    // pub fn print_line_color(
    //     &mut self,
    //     x: i32,
    //     y: i32,
    //     text: &str,
    //     align: TextAlign,
    //     back: Option<RGBA>,
    // ) {
    //     let mut str_len = 0;
    //     let spans = self.get_color_spans(text, &mut str_len);
    //     let mut ix = match align {
    //         TextAlign::Left => x,
    //         TextAlign::Right => x - str_len + 1,
    //         TextAlign::Center => x - str_len / 2,
    //     };
    //     for (color, span) in spans {
    //         self.print_line(ix, y, &span, TextAlign::Left, Some(color), back);
    //         ix += span.chars().count() as i32;
    //     }
    // }
    // /// write a string. If the string reaches the border of the console, it's truncated.
    // /// If the string contains carriage return `"\n"`, multiple lines are printed.
    // pub fn print(
    //     &mut self,
    //     x: i32,
    //     y: i32,
    //     text: &str,
    //     align: TextAlign,
    //     fore: Option<RGBA>,
    //     back: Option<RGBA>,
    // ) {
    //     let mut cury = y;
    //     for line in text.to_owned().split('\n') {
    //         self.print_line(x, cury, line, align, fore, back);
    //         cury += 1;
    //     }
    // }

    // pub fn print_line(
    //     &mut self,
    //     x: i32,
    //     y: i32,
    //     text: &str,
    //     align: TextAlign,
    //     fore: Option<RGBA>,
    //     back: Option<RGBA>,
    // ) {
    //     let stext = text.to_owned();
    //     let mut str_len = stext.chars().count() as i32;
    //     let mut start = 0;
    //     let mut ix = match align {
    //         TextAlign::Left => x,
    //         TextAlign::Right => x - str_len + 1,
    //         TextAlign::Center => x - str_len / 2,
    //     };
    //     if ix < 0 {
    //         str_len += ix;
    //         start -= ix;
    //         ix = 0;
    //     }
    //     if ix + str_len > self.width() as i32 {
    //         str_len = self.width() as i32 - ix;
    //     }
    //     let mut chars = stext.chars().skip(start as usize);
    //     for _ in 0..str_len {
    //         let ch = chars.next();
    //         self.draw(ix, y, Some(to_glyph(ch.unwrap())), fore, back);
    //         ix += 1;
    //     }
    // }
    // /// draw a rectangle, possibly filling it with a character.
    // pub fn rectangle(
    //     &mut self,
    //     x: i32,
    //     y: i32,
    //     w: u32,
    //     h: u32,
    //     fore: Option<RGBA>,
    //     back: Option<RGBA>,
    //     fill: Option<Glyph>,
    // ) {
    //     let right = x + (w as i32) - 1;
    //     let down = y + (h as i32) - 1;
    //     self.draw(x, y, Some(CHAR_CORNER_NW), fore, back);
    //     self.draw(right, down, Some(CHAR_CORNER_SE), fore, back);
    //     self.draw(right, y, Some(CHAR_CORNER_NE), fore, back);
    //     self.draw(x, down, Some(CHAR_CORNER_SW), fore, back);
    //     if (y as u32) < self.height() {
    //         self.area(x + 1, y, w - 2, 1, fore, back, Some(CHAR_LINE_H));
    //     }
    //     if (down as u32) < self.height() {
    //         self.area(x + 1, down, w - 2, 1, fore, back, Some(CHAR_LINE_H));
    //     }
    //     if (x as u32) < self.width() {
    //         self.area(x, y + 1, 1, h - 2, fore, back, Some(CHAR_LINE_V));
    //     }
    //     if (right as u32) < self.width() {
    //         self.area(right, y + 1, 1, h - 2, fore, back, Some(CHAR_LINE_V));
    //     }
    //     if fill.is_some() {
    //         self.area(x + 1, y + 1, w - 2, h - 2, fore, back, fill);
    //     }
    // }

    /// fill an area with values
    pub fn area(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fillglyph: Option<Glyph>,
        fore: Option<RGBA>,
        back: Option<RGBA>,
    ) {
        let right = x + (w as i32);
        let down = y + (h as i32);
        if let Some(fillchar) = fillglyph {
            for iy in y.max(0)..down.min(self.height() as i32) {
                let off = iy * self.pot_size().0 as i32;
                for ix in x.max(0)..right.min(self.width() as i32) {
                    self.glyphs_mut()[(off + ix) as usize] = u32::from(fillchar);
                }
            }
        }
        if let Some(fore) = fore {
            for iy in y.max(0)..down.min(self.height() as i32) {
                let off = iy * self.pot_size().0 as i32;
                for ix in x.max(0)..right.min(self.width() as i32) {
                    self.foregrounds_mut()[(off + ix) as usize] = fore;
                }
            }
        }
        if let Some(back) = back {
            for iy in y.max(0)..down.min(self.height() as i32) {
                let off = iy * self.pot_size().0 as i32;
                for ix in x.max(0)..right.min(self.width() as i32) {
                    self.backgrounds_mut()[(off + ix) as usize] = back;
                }
            }
        }
    }

    /// can change all properties of a console cell at once
    pub fn draw(&mut self, x: i32, y: i32, glyph: Glyph, fore: RGBA, back: RGBA) {
        if let Some(idx) = self.to_idx(x, y) {
            self.glyph[idx] = glyph;
            self.fore[idx] = fore;
            self.back[idx] = back;
        }
    }

    /// can change all properties of a console cell at once
    pub fn draw_opt(
        &mut self,
        x: i32,
        y: i32,
        glyph: Option<Glyph>,
        fore: Option<RGBA>,
        back: Option<RGBA>,
    ) {
        if let Some(idx) = self.to_idx(x, y) {
            if let Some(code) = glyph {
                self.glyph[idx] = code;
            }
            if let Some(fore) = fore {
                self.fore[idx] = fore;
            }
            if let Some(back) = back {
                self.back[idx] = back;
            }
        }
    }
    /// blit (draw) a console onto another one
    /// You can use fore_alpha and back_alpha to blend this console with existing background on the destination.
    /// If you define a key color, the cells using this color as background will be ignored. This makes it possible to blit
    /// non rectangular zones.
    pub fn blit(
        &self,
        x: i32,
        y: i32,
        destination: &mut Buffer,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<RGBA>,
    ) {
        self.blit_ex(
            0,
            0,
            self.width() as i32,
            self.height() as i32,
            destination,
            x,
            y,
            fore_alpha,
            back_alpha,
            key_color,
        );
    }
    /// blit a region of this console onto another one.
    /// see [`crate::draw::Blitter::blit`]
    pub fn blit_ex(
        &self,
        xsrc: i32,
        ysrc: i32,
        wsrc: i32,
        hsrc: i32,
        destination: &mut Buffer,
        xdst: i32,
        ydst: i32,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<RGBA>,
    ) {
        for y in 0..hsrc - ysrc {
            let off = (y + ysrc) * self.pot_size().0 as i32;
            let doff = (y + ydst) * destination.pot_size().0 as i32;
            for x in 0..wsrc - xsrc {
                if self.to_idx(xsrc + x, ysrc + y).is_some()
                    && destination.to_idx(xdst + x, ydst + y).is_some()
                {
                    let src_idx = (off + x + xsrc) as usize;
                    let dest_idx = (doff + x + xdst) as usize;
                    let src_back = self.backgrounds()[src_idx];
                    let dst_back = destination.backgrounds()[dest_idx];
                    if back_alpha > 0.0 {
                        let back = self.backgrounds()[src_idx];
                        if let Some(key) = key_color {
                            if key == back {
                                continue;
                            }
                        }
                        destination.backgrounds_mut()[dest_idx] =
                            RGBA::blend(dst_back, src_back, back_alpha);
                    }
                    if fore_alpha > 0.0 {
                        let src_fore = self.foregrounds()[src_idx];
                        let dst_fore = destination.foregrounds()[dest_idx];
                        let src_char = self.glyphs()[src_idx];
                        let dst_char = destination.glyphs()[dest_idx];
                        let dst_back = destination.backgrounds()[dest_idx];
                        if fore_alpha < 1.0 {
                            if src_char == ' ' as u32 || src_char == 0 {
                                destination.foregrounds_mut()[dest_idx] =
                                    RGBA::blend(dst_fore, src_back, back_alpha);
                            } else if dst_char == ' ' as u32 || dst_char == 0 {
                                destination.glyphs_mut()[dest_idx] = src_char;
                                destination.foregrounds_mut()[dest_idx] =
                                    RGBA::blend(dst_back, src_fore, fore_alpha);
                            } else if dst_char == src_char {
                                destination.foregrounds_mut()[dest_idx] =
                                    RGBA::blend(dst_fore, src_fore, fore_alpha);
                            } else if fore_alpha < 0.5 {
                                destination.foregrounds_mut()[dest_idx] =
                                    RGBA::blend(dst_fore, dst_back, fore_alpha * 2.0);
                            } else {
                                destination.glyphs_mut()[dest_idx] = src_char;
                                destination.foregrounds_mut()[dest_idx] =
                                    RGBA::blend(dst_back, src_fore, (fore_alpha - 0.5) * 2.0);
                            }
                        } else {
                            destination.foregrounds_mut()[dest_idx] = src_fore;
                            destination.glyphs_mut()[dest_idx] = src_char;
                        }
                    }
                }
            }
        }
    }
}

/// compute the length of a string containing color codes.
/// Example :
/// ```
/// use doryen_rs::Console;
/// let len = Console::text_color_len("#[red]red text with a #[blue]blue#[] word");
/// assert_eq!(len, 25); // actual string : "red text with a blue word"
/// let len = Console::text_color_len("#[red]a\nb");
/// assert_eq!(len, 3); // actual string : "a\nb"
/// let len = Console::text_color_len("normal string");
/// assert_eq!(len, 13);
/// ```
#[allow(dead_code)]
fn text_color_len(text: &str) -> usize {
    let mut text_len = 0;
    for color_span in text.to_owned().split("#[") {
        if color_span.is_empty() {
            continue;
        }
        let mut col_text = color_span.split(']');
        let col_name = col_text.next().unwrap();
        if let Some(text_span) = col_text.next() {
            text_len += text_span.chars().count();
        } else {
            text_len += col_name.chars().count();
        }
    }
    text_len
}
