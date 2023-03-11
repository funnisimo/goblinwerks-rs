use gw_app::color::get_color;
use gw_app::color::ColorParseErr;
use gw_app::log;
use gw_app::Glyph;
use gw_app::RGBA;
use gw_util::text::find_first_of;
use serde::{Deserialize, Serialize};
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
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub glyph: Glyph,
    pub fg: RGBA,
    pub bg: RGBA,
}

impl Sprite {
    pub fn new(glyph: Glyph, fg: RGBA, bg: RGBA) -> Sprite {
        Sprite { glyph, fg, bg }
    }

    pub fn mix(&mut self, glyph: Glyph, fg: RGBA, bg: RGBA) {
        if glyph > 0 {
            self.glyph = glyph;
        }
        self.fg = RGBA::alpha_mix(&self.fg, &fg);
        self.bg = RGBA::alpha_mix(&self.bg, &bg);
    }
}

impl FromStr for Sprite {
    type Err = SpriteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.trim();

        let sep = match find_first_of(&str[1..], vec![':', '|', '-', ';']) {
            None => return Err(SpriteParseError::WrongFormat),
            Some((_idx, ch)) => ch,
        };

        // log(format!("parse sprite - {}, sep={}", str, sep));

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

        // log(format!("- ch={}", ch));
        let glyph = match ch.chars().count() {
            0 => 0,
            1 => match ch.chars().next() {
                None => return Err(SpriteParseError::WrongFormat),
                Some(ch) => ch as Glyph,
            },
            _ => {
                if !ch.starts_with("0x") {
                    return Err(SpriteParseError::WrongFormat);
                }
                match u32::from_str_radix(&ch[2..], 16) {
                    Err(e) => {
                        log(format!("- ch parse error - {:?}", e));
                        return Err(SpriteParseError::WrongFormat);
                    }
                    Ok(val) => val,
                }
            }
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
    let glyph = match ch.chars().count() {
        0 => 0,
        1 => match ch.chars().next() {
            None => return Err(SpriteParseError::WrongFormat),
            Some(ch) => ch as Glyph,
        },
        _ => {
            if !ch.starts_with("0x") {
                return Err(SpriteParseError::WrongFormat);
            }
            match u32::from_str_radix(&ch[2..], 16) {
                Err(_) => return Err(SpriteParseError::WrongFormat),
                Ok(val) => val,
            }
        }
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
    fn default() {
        let sprite = Sprite::default();
        assert_eq!(sprite.glyph, 0);
        assert_eq!(sprite.fg, RGBA::rgba(0, 0, 0, 0));
        assert_eq!(sprite.bg, RGBA::rgba(0, 0, 0, 0));
    }

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

    #[test]
    fn from_str_code() {
        init_colors();

        // This will not work in a TOML file (0x7F is reserved)

        let sprite: Sprite = "\u{007F}|red".into();
        assert_eq!(sprite.glyph, 127);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());

        let sprite: Sprite = "\u{007F}|red".parse().unwrap();
        assert_eq!(sprite.glyph, 127);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());
    }

    #[test]
    fn from_str_glyph() {
        init_colors();

        // This will not work in a TOML file (0x7F is reserved)

        let sprite: Sprite = "0x7F|red".into();
        assert_eq!(sprite.glyph, 127);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());

        let sprite: Sprite = "0x7F|red".parse().unwrap();
        assert_eq!(sprite.glyph, 127);
        assert_eq!(sprite.fg, named::RED.into());
        assert_eq!(sprite.bg, named::NONE.into());
    }
}
