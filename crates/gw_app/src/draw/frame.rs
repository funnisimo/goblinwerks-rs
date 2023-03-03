use super::plain;
use crate::console::{Buffer, Glyph};
use crate::{color::RGBA, TextAlign};

/// The border type - single, double, or colored
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BorderType {
    /// Just fill the bg of the border
    Color,
    /// Use the single line chars
    Single,
    /// Use the double line chars
    Double,
}

impl BorderType {
    /// Returns the glyphs needed to draw the frame for this border type
    pub fn glyphs(&self, to_glyph: &dyn Fn(char) -> Glyph) -> [Glyph; 8] {
        match self {
            BorderType::Color => [0; 8],
            BorderType::Single => ['│', '│', '─', '─', '┌', '┐', '└', '┘'].map(|c| to_glyph(c)),
            BorderType::Double => ['║', '║', '═', '═', '╔', '╗', '╚', '╝'].map(|c| to_glyph(c)),
        }
    }
}

/// Constructs a new frame for this buffer
pub fn frame<'a>(buffer: &'a mut Buffer) -> Frame {
    Frame::new(buffer)
}

/// A frame drawing object
pub struct Frame<'a> {
    buffer: &'a mut Buffer,
    border: BorderType,

    fg: Option<RGBA>,
    bg: Option<RGBA>,

    fill_fg: Option<RGBA>,
    fill_glyph: Option<Glyph>,
    fill_bg: Option<RGBA>,

    title: String,
    title_fg: Option<RGBA>,
    title_align: TextAlign,
    // to_glyph: &'a ToGlyphFn,
}

impl<'a> Frame<'a> {
    /// Construct a new Frame for this buffer
    pub fn new(buffer: &'a mut Buffer) -> Self {
        Frame {
            buffer,
            border: BorderType::Single,

            fg: None,
            bg: None,

            fill_fg: None,
            fill_bg: None,
            fill_glyph: None,

            title: "".to_owned(),
            title_fg: None,
            title_align: TextAlign::Center,
            // to_glyph: &default_to_glyph,
        }
    }

    /// Set the border type, default=[`BorderType::Single`]
    pub fn border(mut self, border: BorderType) -> Self {
        self.border = border;
        self
    }

    /// Sets the fg color of the frame glyphs, default=None
    pub fn fg(mut self, fg: RGBA) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Sets the bg color of the frame glyphs, default=None
    pub fn bg(mut self, bg: RGBA) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Sets the glyph, fg, bg of the fill to use inside the frame, default=None,None,None
    pub fn fill(mut self, glyph: Option<Glyph>, fg: Option<RGBA>, bg: Option<RGBA>) -> Self {
        self.fill_glyph = glyph;
        self.fill_fg = fg;
        self.fill_bg = bg;
        self
    }

    /// Sets the title of the frame (drawn in the top)
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    /// Sets the fg color of the frame title, default=same as frame
    pub fn title_fg(mut self, fg: RGBA) -> Self {
        self.title_fg = Some(fg);
        self
    }

    /// Sets the alignment of the title in the top of the frame, default=Center
    pub fn title_align(mut self, align: TextAlign) -> Self {
        self.title_align = align;
        self
    }

    // pub fn to_glyph(mut self, to_glyph: &'a dyn Fn(char) -> Glyph) -> Self {
    //     self.to_glyph = to_glyph;
    //     self
    // }

    /// Draws the frame, title, and any fill
    pub fn draw(&mut self, x: i32, y: i32, width: u32, height: u32) {
        if self.fill_bg.is_some() || self.fill_fg.is_some() || self.fill_glyph.is_some() {
            self.buffer.area(
                x,
                y,
                width,
                height,
                self.fill_glyph,
                self.fill_fg,
                self.fill_bg,
            );
        }

        let glyphs = self.border.glyphs(self.buffer.to_glyph_fn);
        let left = x;
        let top = y;
        let right = x + width as i32 - 1;
        let bottom = y + height as i32 - 1;

        for y in top..bottom {
            self.buffer
                .draw_opt(left, y, Some(glyphs[0]), self.fg, self.bg);
            self.buffer
                .draw_opt(right, y, Some(glyphs[1]), self.fg, self.bg);
        }
        for x in left..right {
            self.buffer
                .draw_opt(x, top, Some(glyphs[2]), self.fg, self.bg);
            self.buffer
                .draw_opt(x, bottom, Some(glyphs[3]), self.fg, self.bg);
        }

        self.buffer
            .draw_opt(left, top, Some(glyphs[4]), self.fg, self.bg);
        self.buffer
            .draw_opt(right, top, Some(glyphs[5]), self.fg, self.bg);
        self.buffer
            .draw_opt(left, bottom, Some(glyphs[6]), self.fg, self.bg);
        self.buffer
            .draw_opt(right, bottom, Some(glyphs[7]), self.fg, self.bg);

        let tw = self.title.len() as i32;
        if tw > 0 {
            let tx = match self.title_align {
                TextAlign::Left => x + 2,
                TextAlign::Right => x + width as i32 - tw - 2,
                TextAlign::Center => x + width as i32 / 2,
            };

            let printer = match self.title_fg {
                None => plain(self.buffer),
                Some(x) => plain(self.buffer).fg(x),
            };
            printer.align(self.title_align).print(tx, y, &self.title);
            // self.title.draw(dest);
        }
    }
}
