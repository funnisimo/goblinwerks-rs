use super::TextAlign;
use crate::color::{get_color_opt, RGBA};
use crate::console::Buffer;
use crate::console::Glyph;
use crate::text::parse_colored_lines;
use crate::text::wrap_colored;
use crate::text::ColoredLine;
use crate::text::ColoredSpan;
use std::cmp::{max, min};
use std::fmt::Debug;

/// Creates a [`ColoredPrinter`]
pub fn colored<'a>(buffer: &'a mut Buffer) -> ColoredPrinter {
    ColoredPrinter::new(buffer)
}

/// Prints color encoded text to the buffer
pub struct ColoredPrinter<'a> {
    buffer: &'a mut Buffer,
    width: Option<i32>,
    height: Option<i32>,
    align: TextAlign,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    to_glyph: &'a dyn Fn(char) -> Glyph,
}

impl<'a> ColoredPrinter<'a> {
    /// Creates a `ColoredPrinter` for the buffer
    pub fn new(buffer: &'a mut Buffer) -> Self {
        ColoredPrinter {
            buffer,
            width: None,
            height: None,
            align: TextAlign::Left,
            fg: Some(RGBA::rgb(255, 255, 255)),
            bg: None,
            to_glyph: &|ch| ch as u32,
        }
    }

    /// Sets the width of the printing
    ///
    /// If the width is > than the text, will print glyph 0 and fill any bg
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the printing
    ///
    /// If the height is > than the text, will print glyph 0 and fill any bg
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the alignment for the printing
    ///
    /// Center Align means center of text is on the given x for draw calls
    /// Right Align means the right edge of the text is on the given x
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the fg (default=WHITE)
    pub fn fg(mut self, fg: RGBA) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Sets the bg (default=None)
    pub fn bg(mut self, bg: RGBA) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Sets the char->Glyph conversion function, default=(ch as u32)
    pub fn to_glyph(mut self, to_glyph: &'a dyn Fn(char) -> Glyph) -> Self {
        self.to_glyph = to_glyph;
        self
    }

    /// Prints the given text at the given location, returns the length printed
    pub fn print(&mut self, x: i32, y: i32, text: &str) -> i32 {
        // let width = self.width.unwrap_or(self.buffer.width() as i32 - x);
        let mut widest = 0;

        let mut cy = y;
        for line in parse_colored_lines(text).iter().take(1) {
            let w = self.print_line(x, cy, &line);
            widest = max(widest, w);
            cy += 1;
        }

        if let Some(height) = self.height {
            for _ in 1..height {
                for ix in 0..widest {
                    self.buffer.draw_opt(x + ix, cy, Some(0), self.fg, self.bg);
                }
                cy += 1;
            }
        }

        widest
    }

    /// Prints all the lines in the given text, truncates at width (if any), returns the (width,height) printed
    pub fn print_lines(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        // let width = self.width.unwrap_or(self.buffer.width() as i32 - x);
        let max_height = self.height.unwrap_or(999);

        let mut widest = 0;

        let mut cy = y;
        for (i, line) in parse_colored_lines(text).iter().enumerate() {
            if i as i32 >= max_height {
                break;
            }
            let w = self.print_line(x, cy, &line);
            widest = max(widest, w);
            cy += 1;
        }

        if let Some(height) = self.height {
            for _ in (cy - y)..height {
                for ix in 0..widest {
                    self.buffer.draw_opt(x + ix, cy, Some(0), self.fg, self.bg);
                }
                cy += 1;
            }
        }

        (widest, cy - y)
    }

    /// Performs word wrapping of the given text at the setup width (or buffer width) and prints the lines
    pub fn wrap(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let width = self.width.unwrap_or(self.buffer.width() as i32 - x);
        let max_height = self.height.unwrap_or(999);

        let mut widest = self.width.unwrap_or(0);

        let mut cy = y;
        for (i, line) in wrap_colored(width as usize, text).iter().enumerate() {
            if i as i32 >= max_height {
                break;
            }
            let w = self.print_line(x, cy, &line);
            widest = max(widest, w);
            cy += 1;
        }

        if let Some(height) = self.height {
            for _ in (cy - y)..height {
                for ix in 0..widest {
                    self.buffer.draw_opt(x + ix, cy, Some(0), self.fg, self.bg);
                }
                cy += 1;
            }
        }

        (widest, cy - y)
    }

    /// Prints the line, handles width, bg, and align
    fn print_line(&mut self, x: i32, y: i32, line: &ColoredLine) -> i32 {
        let width = self.width.unwrap_or(line.char_len() as i32);
        let self_len = min(width, line.char_len() as i32);
        let spaces = width.saturating_sub(self_len);

        let (x, pre, post) = match self.align {
            TextAlign::Left => (x, 0, spaces),
            TextAlign::Center => {
                let half = spaces / 2;
                (x - half - self_len / 2, half, spaces - half)
            }
            TextAlign::Right => (x - self_len + 1, spaces, 0),
        };

        let mut cx = x;
        let fg = self.fg;
        let bg = self.bg;

        // let mut output = "[".to_string();
        for _ in 0..pre {
            self.buffer.draw_opt(cx, y, Some(0), fg, bg);
            cx += 1;
        }

        // output += line.0;
        let mut left = self_len as u32;
        for span in line.spans() {
            let w = self.print_span(cx, y, span, left);
            left = left.saturating_sub(w as u32);
            cx += w;
            if left == 0 {
                // post = 0;
                break;
            }
        }

        for _ in 0..post {
            self.buffer.draw_opt(cx, y, Some(0), fg, bg);
            cx += 1;
        }

        // output.push(']');

        // println!("{} [{}]", output, output.len() - 2);
        width
    }

    /// just print the text - nothing more, nothing less
    /// decisions about padding, alignment, etc... need to be in ColoredLine::print
    fn print_span(&mut self, x: i32, y: i32, span: &ColoredSpan, width: u32) -> i32 {
        let mut cx = x;
        let fg = match span.color() {
            None => self.fg,
            Some(txt) => get_color_opt(txt),
        };
        let bg = self.bg;

        let mut left = width;
        for char in span.as_str().chars() {
            let glyph = (self.to_glyph)(char);
            self.buffer.draw_opt(cx, y, Some(glyph), fg, bg);
            cx += 1;
            left = left.saturating_sub(1);
            if left == 0 {
                break;
            }
        }

        cx - x
    }
}

impl<'a> Debug for ColoredPrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ColoredPrinter");

        s.field("buffer", &self.buffer.size());
        if let Some(width) = self.width {
            s.field("width", &width);
        }
        if let Some(height) = self.height {
            s.field("height", &height);
        }

        s.field("align", &self.align);

        if let Some(ref fg) = self.fg {
            s.field("fg", fg);
        }

        if let Some(ref bg) = self.bg {
            s.field("bg", bg);
        }

        s.finish()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const _WHITE: RGBA = RGBA::rgb(255, 255, 255);
    const _RED: RGBA = RGBA::rgb(255, 0, 0);
    const _GREEN: RGBA = RGBA::rgb(0, 255, 0);
    const _BLUE: RGBA = RGBA::rgb(0, 0, 255);
    const _BLACK: RGBA = RGBA::rgb(0, 0, 0);

    // #[test]
    // fn no_color() {
    //     let mut iter = TextIterator::new(&|_| Some(WHITE), "Text");

    //     assert_eq!(iter.next().unwrap(), (None, 'T'));
    //     assert_eq!(iter.next().unwrap(), (None, 'e'));
    //     assert_eq!(iter.next().unwrap(), (None, 'x'));
    //     assert_eq!(iter.next().unwrap(), (None, 't'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn start_color() {
    //     fn to_rgba(_: &str) -> Option<RGBA> {
    //         Some(BLUE)
    //     }

    //     let mut iter = TextIterator::new(&to_rgba, "#[blue]Text");

    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'T'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'e'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'x'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 't'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn mid_color() {
    //     fn to_rgba(t: &str) -> Option<RGBA> {
    //         match t {
    //             "blue" => Some(BLUE),
    //             "white" => Some(WHITE),
    //             _ => None,
    //         }
    //     }

    //     let mut iter = TextIterator::new(&to_rgba, "a #[blue]b#[] c");

    //     assert_eq!(iter.next().unwrap(), (None, 'a'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, 'c'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn escape_color() {
    //     let mut iter = TextIterator::new(&|_| Some(RED), "a #[[blue]b#[[] c");

    //     assert_eq!(iter.next().unwrap(), (None, 'a'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, '#'));
    //     assert_eq!(iter.next().unwrap(), (None, '['));
    //     assert_eq!(iter.next().unwrap(), (None, 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, 'l'));
    //     assert_eq!(iter.next().unwrap(), (None, 'u'));
    //     assert_eq!(iter.next().unwrap(), (None, 'e'));
    //     assert_eq!(iter.next().unwrap(), (None, ']'));
    //     assert_eq!(iter.next().unwrap(), (None, 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, '#'));
    //     assert_eq!(iter.next().unwrap(), (None, '['));
    //     assert_eq!(iter.next().unwrap(), (None, ']'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, 'c'));
    //     assert_eq!(iter.next(), None);
    // }

    fn extract_line(buf: &Buffer, x: i32, y: i32, width: i32) -> String {
        let mut output = "".to_string();
        for cx in x..x + width {
            if let Some(g) = buf.get_glyph(cx, y) {
                output.push(char::from_u32(*g).unwrap());
            }
        }
        output
    }

    #[test]
    fn wrap_basic() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        assert_eq!(printer.wrap(0, 0, "taco casa"), (10, 1));
        assert_eq!(extract_line(&buffer, 0, 0, 10), "taco casa\0");
    }

    #[test]
    fn trunc_basic() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        assert_eq!(
            printer.print(0, 0, "This is a longer text that will be truncated."),
            10
        );
        assert_eq!(extract_line(&buffer, 0, 0, 11), "This is a \0");
    }

    #[test]
    fn wrap_multi_plain() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(0, 1, "#[red]taco casa#[] is a great fast food place");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "taco casa\0\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "is a great\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "fast food\0\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "place\0\0\0\0\0\0");
        assert_eq!(r, (10, 4));
    }

    #[test]
    fn wrap_multi_height() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10).height(3);

        let r = printer.wrap(0, 1, "#[red]taco casa#[] is a great fast food place");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "taco casa\0\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "is a great\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "fast food\0\0");
        assert_eq!(extract_line(&buffer, 0, 4, 5), "\0\0\0\0\0");
        assert_eq!(r, (10, 3));
    }

    #[test]
    fn wrap_breakword() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(0, 1, "supercalafragalisticexpialadocious");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "supercala-\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "fragalist-\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "icexpiala-\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "docious\0\0\0\0");
        assert_eq!(r, (10, 4));
    }

    #[test]
    fn wrap_multi_hyphen() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(
            0,
            1,
            "the conflaguration exponentially #[#f00]deteriorated#[] the stonemasons' monuments",
        );
        assert_eq!(extract_line(&buffer, 0, 1, 11), "the confl-\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "aguration\0\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "exponenti-\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "ally dete-\0");
        assert_eq!(extract_line(&buffer, 0, 5, 11), "riorated\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 6, 11), "the stone-\0");
        assert_eq!(extract_line(&buffer, 0, 7, 11), "masons'\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 8, 11), "monuments\0\0");
        assert_eq!(r, (10, 8));
    }

    #[test]
    fn wrap_lines() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(20);

        let r = printer.wrap(
            0,
            1,
            "the conflaguration\nexponentially\ndeteriorated the\nstonemasons' monuments",
        );
        assert_eq!(extract_line(&buffer, 0, 1, 21), "the conflaguration\0\0\0");
        assert_eq!(
            extract_line(&buffer, 0, 2, 21),
            "exponentially\0\0\0\0\0\0\0\0"
        );
        assert_eq!(
            extract_line(&buffer, 0, 3, 21),
            "deteriorated the\0\0\0\0\0"
        );
        assert_eq!(extract_line(&buffer, 0, 4, 21), "stonemasons' monume-\0");
        assert_eq!(
            extract_line(&buffer, 0, 5, 21),
            "nts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
        assert_eq!(r, (20, 5));
    }
}
