use gw_app::color::get_color;
use gw_app::color::ColorParseErr;
use gw_app::Glyph;
use gw_app::RGBA;
use std::convert::From;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum SpriteParseError {
    WrongFormat,
    BadForeColor(ColorParseErr),
    BadBackColor(ColorParseErr),
}

impl Display for SpriteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpriteParseError::WrongFormat => write!(f, "Wrong format"),
            SpriteParseError::BadForeColor(err) => write!(f, "Bad Fore Color - {:?}", err),
            SpriteParseError::BadBackColor(err) => write!(f, "Bad Back Color - {:?}", err),
        }
    }
}

// #[derive(Component, Default, Clone, Copy, Debug)]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sprite {
    pub glyph: Glyph,
    pub fg: RGBA,
    pub bg: RGBA,
}

impl Sprite {
    pub fn new(glyph: Glyph, fg: RGBA, bg: RGBA) -> Sprite {
        Sprite { glyph, fg, bg }
    }
}

impl FromStr for Sprite {
    type Err = SpriteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.trim();

        let sep = s.chars().nth(1).unwrap_or('|');

        let parts: Vec<&str> = str.split(sep).map(|p| p.trim()).collect();
        let (ch, fg, bg) = match parts.len() {
            1 => match parts[0].len() {
                1 => (parts[0], "white", "none"),
                _ => (" ", "none", parts[0]),
            },
            2 => (parts[0], parts[1], "none"),
            3 => (parts[0], parts[1], parts[2]),
            _ => return Err(SpriteParseError::WrongFormat),
        };

        let glyph = match ch.chars().next() {
            None => 0,
            Some(ch) => ch as Glyph,
        };

        let fg = match get_color(fg) {
            Err(e) => return Err(SpriteParseError::BadForeColor(e)),
            Ok(c) => c,
        };

        let bg = match get_color(bg) {
            Err(e) => return Err(SpriteParseError::BadBackColor(e)),
            Ok(c) => c,
        };

        Ok(Sprite::new(glyph, fg, bg))
    }
}

impl From<&str> for Sprite {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(sprite) => sprite,
            Err(_) => panic!("Failed to parse Sprite: {}", s),
        }
    }
}

pub fn from_text(ch: &str, fg: &str, bg: &str) -> Result<Sprite, SpriteParseError> {
    let glyph = match ch.chars().next() {
        None => return Err(SpriteParseError::WrongFormat),
        Some(ch) => ch as Glyph,
    };

    let fg = match get_color(fg) {
        Err(e) => return Err(SpriteParseError::BadForeColor(e)),
        Ok(c) => c,
    };

    let bg = match get_color(bg) {
        Err(e) => return Err(SpriteParseError::BadBackColor(e)),
        Ok(c) => c,
    };

    Ok(Sprite::new(glyph, fg, bg))
}

#[cfg(test)]
mod test {
    use super::*;
    use gw_app::color::init_colors;
    use gw_app::color::named;

    #[test]
    fn from_text() {
        init_colors();

        let sprite = super::from_text("*", "red", "").unwrap();
        assert_eq!(sprite.glyph, '*' as Glyph);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());
    }

    #[test]
    fn from_str() {
        init_colors();

        let sprite: Sprite = "*|red".into();
        assert_eq!(sprite.glyph, '*' as Glyph);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());
    }
}