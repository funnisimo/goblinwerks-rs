use super::{static_color, RGBA};

/// Errors from parsing color strings
#[derive(Debug, Copy, Clone)]
pub enum ColorParseErr {
    /// Found a digit that is not a hex digit [0-9A-Fa-f]
    NonHexDigit,
    /// Found a digit that is not an ascii digit [0-9]
    NonAsciiDigit,
    /// Found text that is not valid hex length (3,4,6, or 8)
    WrongHexLen,
    /// Found text that does not have correct number of rgba components (3 or 4)
    WrongRgbLen,
    /// Unknown Color
    UnknownColor,
    /// Invalid name (uses _ incorrectly)
    InvalidName,
    /// Invalid Modifier
    InvalidModifier,
}

pub fn get_color_opt(name: &str) -> Option<RGBA> {
    match get_color(name) {
        Err(_) => None,
        Ok(c) => Some(c),
    }
}

pub fn get_color(name: &str) -> Result<RGBA, ColorParseErr> {
    let name = name.trim().to_lowercase();
    if name.len() == 0 {
        return Ok(RGBA::new());
    } else if name.starts_with("#") {
        return parse_color_hex(&name);
    } else if name.starts_with("(") || name.starts_with("rgb(") || name.starts_with("rgba(") {
        return parse_color_rgb(&name);
    } else if name.contains("_") {
        // light + 25, lighter + 50, lightest + 75
        // dark - 25, darker - 50, darkest - 75
        let mut parts = name.split("_");

        let level = parts.next();
        let name = parts.next();

        // println!(
        //     "get_color: {} {}",
        //     level.as_ref().unwrap(),
        //     name.as_ref().unwrap()
        // );
        match name {
            None => Err(ColorParseErr::InvalidName),
            Some(name) => match static_color(name) {
                None => Err(ColorParseErr::UnknownColor),
                Some(color) => {
                    let result = match level {
                        Some("light") => RGBA::lerp(&color, &(255, 255, 255, 255).into(), 0.25),
                        Some("lighter") => RGBA::lerp(&color, &(255, 255, 255, 255).into(), 0.50),
                        Some("lightest") => RGBA::lerp(&color, &(255, 255, 255, 255).into(), 0.75),
                        Some("dark") => RGBA::lerp(&color, &(0, 0, 0, 255).into(), 0.25),
                        Some("darker") => RGBA::lerp(&color, &(0, 0, 0, 255).into(), 0.50),
                        Some("darkest") => RGBA::lerp(&color, &(0, 0, 0, 255).into(), 0.75),
                        _ => return Err(ColorParseErr::InvalidModifier),
                    };

                    Ok(result)
                }
            },
        }
    } else {
        // try to lookup
        match static_color(&name) {
            // if lookup fails, it might just be a hex color - e.g. #[f00]
            None => match parse_color_hex(&name) {
                Err(_) => Err(ColorParseErr::UnknownColor),
                Ok(c) => Ok(c),
            },
            Some(c) => Ok(c),
        }
    }
}

/// Parses RGBA from hex string
///
/// Text can start with or without '#' e.g. '#fff' or 'FFF'
/// Hex can be any of the following formats:
/// - RGB
/// - RRGGBB
/// - RGBA
/// - RRGGBBAA
/// Where R,G,B,A are all hex values [0-9A-Fa-f]
pub fn parse_color_hex(text: &str) -> Result<RGBA, ColorParseErr> {
    let no_hash = match text.starts_with("#") {
        false => text,
        true => &text[1..],
    };

    let base = match no_hash.chars().position(|ch| ch == ' ') {
        None => no_hash,
        Some(pos) => &no_hash[..pos],
    };

    if !base.chars().all(|ch| ch.is_ascii_hexdigit()) {
        println!("NonHexDigit - {}", text);
        return Err(ColorParseErr::NonHexDigit);
    }

    let digits: Vec<u32> = base
        .chars()
        .map(|ch| ch.to_digit(16).unwrap_or(0))
        .collect();

    let (r, g, b, a) = match digits.len() {
        3 => (
            digits[0] as f32 / 15.0,
            digits[1] as f32 / 15.0,
            digits[2] as f32 / 15.0,
            1.0,
        ),
        4 => (
            digits[0] as f32 / 15.0,
            digits[1] as f32 / 15.0,
            digits[2] as f32 / 15.0,
            digits[3] as f32 / 15.0,
        ),
        6 => (
            (digits[0] as f32 * 16.0 + digits[1] as f32) / 255.0,
            (digits[2] as f32 * 16.0 + digits[3] as f32) / 255.0,
            (digits[4] as f32 * 16.0 + digits[5] as f32) / 255.0,
            1.0,
        ),
        8 => (
            (digits[0] as f32 * 16.0 + digits[1] as f32) / 255.0,
            (digits[2] as f32 * 16.0 + digits[3] as f32) / 255.0,
            (digits[4] as f32 * 16.0 + digits[5] as f32) / 255.0,
            (digits[6] as f32 * 16.0 + digits[7] as f32) / 255.0,
        ),
        _ => {
            return Err(ColorParseErr::WrongHexLen);
        }
    };

    Ok((r, g, b, a).into())
}

/// Parses RGBA from comma separated R,G,B,A values
///
/// Alpha is optional, values must be separated by comma
/// The text can optionally start with 'rgb(', 'rgba(', or '('
/// If the text starts with something that has an opening paren, it can end with one.
pub fn parse_color_rgb(text: &str) -> Result<RGBA, ColorParseErr> {
    let start = match text.chars().position(|ch| ch == '(') {
        None => text,
        Some(idx) => &text[idx + 1..],
    };

    let body = match start.chars().position(|ch| ch == ')') {
        None => start,
        Some(idx) => &start[..idx],
    };

    // println!("color guts = {}", &text[start..end + start]);

    let num_parts = body.split(",").map(|p| p.trim()).collect::<Vec<&str>>();

    if num_parts.len() != 3 && num_parts.len() != 4 {
        return Err(ColorParseErr::WrongRgbLen);
    }

    let mut nums: Vec<u8> = Vec::new();
    for part in num_parts {
        if !part.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(ColorParseErr::NonAsciiDigit);
        }
        match part.parse::<u8>() {
            Err(_) => return Err(ColorParseErr::NonAsciiDigit),
            Ok(v) => nums.push(v),
        }
    }

    match nums.len() {
        3 => return Ok((nums[0], nums[1], nums[2], 255).into()),
        4 => return Ok((nums[0], nums[1], nums[2], nums[3]).into()),
        _ => {
            return Err(ColorParseErr::WrongRgbLen);
        }
    }
}

/// Parses RGBA from either hex or rgb or rgba values
pub fn parse_color(name: &str) -> Result<RGBA, ColorParseErr> {
    let name = name.trim().to_lowercase();
    if name.starts_with("#") {
        // skip down...
    } else if name.starts_with("(")
        || name.starts_with("rgb(")
        || name.starts_with("rgba(")
        || name.contains(",")
    {
        return parse_color_rgb(&name);
    }
    parse_color_hex(&name)
}

// /// Returns RGBA if the text can parse successfully
// fn to_rgba(name: &str) -> Option<RGBA> {
//     match parse_color(name) {
//         Err(_) => {
//             // console(format!("{:?}", e));
//             None
//         }
//         Ok(c) => Some(c),
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    const WHITE: RGBA = RGBA::rgb(255, 255, 255);
    const RED: RGBA = RGBA::rgb(255, 0, 0);
    const GREEN: RGBA = RGBA::rgb(0, 255, 0);
    const BLUE: RGBA = RGBA::rgb(0, 0, 255);
    const _BLACK: RGBA = RGBA::rgb(0, 0, 0);

    #[test]
    fn parse_hex() {
        assert_eq!(parse_color_hex("#fff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffffff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffffffff").unwrap(), WHITE);

        assert_eq!(parse_color_hex("#f00").unwrap(), RED);
        assert_eq!(parse_color_hex("#0f0f").unwrap(), GREEN);
        assert_eq!(parse_color_hex("#0000ff").unwrap(), BLUE);
        assert_eq!(
            parse_color_hex("#80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert_eq!(parse_color_hex("F00").unwrap(), RED);
        assert_eq!(parse_color_hex("0F0F").unwrap(), GREEN);
        assert_eq!(parse_color_hex("0000FF").unwrap(), BLUE);
        assert_eq!(
            parse_color_hex("80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert!(parse_color_hex("white").is_err());
        assert!(parse_color_hex("12,34,56").is_err());
    }

    #[test]
    fn parse_rgb() {
        assert_eq!(parse_color_rgb("0,0,0").unwrap(), RGBA::rgba(0, 0, 0, 255));

        assert_eq!(
            parse_color_rgb("rgb(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(
            parse_color_rgb("(255,150,200,25)").unwrap(),
            RGBA::rgba(255, 150, 200, 25)
        );

        assert_eq!(
            parse_color_rgb("rgba(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert!(parse_color_rgb("FFF").is_err());
        assert!(parse_color_rgb("white").is_err());
    }

    #[test]
    fn parse_test() {
        assert_eq!(parse_color("0,0,0").unwrap(), RGBA::rgba(0, 0, 0, 255));

        assert_eq!(
            parse_color("rgb(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(
            parse_color("(255,150,200,25)").unwrap(),
            RGBA::rgba(255, 150, 200, 25)
        );

        assert_eq!(
            parse_color("rgba(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(parse_color("#f00").unwrap(), RED);
        assert_eq!(parse_color("#0f0f").unwrap(), GREEN);
        assert_eq!(parse_color("#0000ff").unwrap(), BLUE);
        assert_eq!(
            parse_color("#80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert_eq!(parse_color("F00").unwrap(), RED);
        assert_eq!(parse_color("0F0F").unwrap(), GREEN);
        assert_eq!(parse_color("0000FF").unwrap(), BLUE);
        assert_eq!(parse_color("0000FF # comment").unwrap(), BLUE);

        assert_eq!(
            parse_color("80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert!(parse_color("white").is_err());
        assert!(parse_color("WHITE").is_err());
    }
}
