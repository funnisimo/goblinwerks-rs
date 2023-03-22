use serde::{Deserialize, Serialize};

// use crate::console;
use super::{get_color, ColorParseErr};
use std::{ops, str::FromStr};

/// White color
pub const WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);

/// Black color
pub const BLACK: RGBA = RGBA::rgba(0, 0, 0, 255);

/// RGB tuple
pub type RGB = (u8, u8, u8);

/// Tuple of Red,Green,Blue,Alpha components 0-255
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl RGBA {
    /// Constucts an empty RGBA (0,0,0,0)
    pub const fn new() -> Self {
        RGBA(0, 0, 0, 0)
    }

    /// Constructs an RGBA from R,G,B components
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        RGBA(r, g, b, 255)
    }

    /// Constructs and RGBA from R,G,B,A components
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA(r, g, b, a)
    }

    pub fn is_empty(&self) -> bool {
        self.a() == 0
    }

    /// The red component
    pub fn r(&self) -> u8 {
        self.0
    }

    /// The green component
    pub fn g(&self) -> u8 {
        self.1
    }

    /// The blue component
    pub fn b(&self) -> u8 {
        self.2
    }

    /// The alpha component
    pub fn a(&self) -> u8 {
        self.3
    }

    /// As a tuple of floats (0.0-1.0)
    pub fn to_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
            self.3 as f32 / 255.0,
        )
    }

    /// Mixes RGB of c2 into c1 using pct
    pub fn blend(c1: &RGBA, c2: &RGBA, pct: f32) -> RGBA {
        let alpha = pct * c2.3 as f32 / 255.0;
        RGBA::rgba(
            ((1.0 - alpha) * f32::from(c1.0) + alpha * f32::from(c2.0)) as u8,
            ((1.0 - alpha) * f32::from(c1.1) + alpha * f32::from(c2.1)) as u8,
            ((1.0 - alpha) * f32::from(c1.2) + alpha * f32::from(c2.2)) as u8,
            255, // TODO - c1.3.saturating_add((alpha * f32::from(c2.2)) as u8)
        )
    }

    /// Slides between c1 and c2 using pct
    pub fn lerp(c1: &RGBA, c2: &RGBA, pct: f32) -> RGBA {
        RGBA::rgba(
            ((1.0 - pct) * f32::from(c1.0) + pct * f32::from(c2.0)) as u8,
            ((1.0 - pct) * f32::from(c1.1) + pct * f32::from(c2.1)) as u8,
            ((1.0 - pct) * f32::from(c1.2) + pct * f32::from(c2.2)) as u8,
            ((1.0 - pct) * f32::from(c1.3) + pct * f32::from(c2.3)) as u8,
        )
    }

    /// Multiplies R,G,B components by coef
    pub fn scale(c: &RGBA, coef: f32) -> RGBA {
        // RGBA::rgba(
        //     (f32::from(c.0) * coef).min(255.0) as u8,
        //     (f32::from(c.1) * coef).min(255.0) as u8,
        //     (f32::from(c.2) * coef).min(255.0) as u8,
        //     c.3,
        // )
        c * coef
    }

    /// multiplies c1 * c2 using c2 as percent
    pub fn multiply(c1: &RGBA, c2: &RGBA) -> RGBA {
        RGBA::rgba(
            (f32::from(c1.0) * f32::from(c2.0) / 255.0) as u8,
            (f32::from(c1.1) * f32::from(c2.1) / 255.0) as u8,
            (f32::from(c1.2) * f32::from(c2.2) / 255.0) as u8,
            255,
        )
    }

    /// Returns 50% c1 + 50% c2
    pub fn mix(c1: &RGBA, c2: &RGBA) -> RGBA {
        RGBA::rgba(
            (0.5 * f32::from(c1.0) + 0.5 * f32::from(c2.0)) as u8,
            (0.5 * f32::from(c1.1) + 0.5 * f32::from(c2.1)) as u8,
            (0.5 * f32::from(c1.2) + 0.5 * f32::from(c2.2)) as u8,
            (0.5 * f32::from(c1.3) + 0.5 * f32::from(c2.3)) as u8,
        )
    }

    pub fn alpha_mix(base: &RGBA, with: &RGBA) -> RGBA {
        if base.a() <= 0 {
            return with.clone();
        }
        if with.a() <= 0 {
            return base.clone();
        }
        if with.a() >= 255 {
            return with.clone();
        }
        let with_pct = with.a() as f32 / 255.0;
        let base_pct = 1.0 - with_pct;

        let mut res = base * base_pct + with * with_pct;
        res.3 = base.a().saturating_add(with.a());
        res
    }

    /// Computes squared distance between colors
    pub fn distance(c1: &RGBA, c2: &RGBA) -> i32 {
        let dr = i32::from(c1.0) - i32::from(c2.0);
        let dg = i32::from(c1.1) - i32::from(c2.1);
        let db = i32::from(c1.2) - i32::from(c2.2);
        dr * dr + dg * dg + db * db
    }

    /// Removes pct of color (RGB) from c1
    pub fn darken(c1: &RGBA, pct: f32) -> RGBA {
        let mut to_sub = c1 * pct;
        to_sub.3 = 0;
        *c1 - to_sub
    }

    /// Moves c1 color pct closer to white
    pub fn lighten(c1: &RGBA, pct: f32) -> RGBA {
        let mut to_add = (WHITE - *c1) * pct;
        to_add.3 = 0;
        c1 + to_add
    }

    pub fn invert(base: &RGBA) -> RGBA {
        RGBA::rgba(255 - base.r(), 255 - base.g(), 255 - base.b(), base.a())
    }

    pub fn binary_inverse(base: &RGBA) -> RGBA {
        let r = if base.r() < 128 { 255 } else { 0 };
        let g = if base.g() < 128 { 255 } else { 0 };
        let b = if base.b() < 128 { 255 } else { 0 };
        RGBA::rgba(r, g, b, base.a())
    }
}

/// Convert from RGB to RGBA
impl From<RGB> for RGBA {
    fn from(d: RGB) -> Self {
        RGBA::rgb(d.0, d.1, d.2)
    }
}

impl From<&RGBA> for RGBA {
    fn from(d: &RGBA) -> Self {
        RGBA::rgba(d.0, d.1, d.2, d.3)
    }
}

/// Convert from tuple of u8 to RGBA
impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(d: (u8, u8, u8, u8)) -> Self {
        RGBA::rgba(d.0, d.1, d.2, d.3)
    }
}

/// Convert from tuple of floats to RGBA
impl From<(f32, f32, f32)> for RGBA {
    fn from(d: (f32, f32, f32)) -> Self {
        let r = (d.0 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let g = (d.1 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let b = (d.2 * 255.0).floor().clamp(0.0, 255.0) as u8;
        RGBA::rgb(r, g, b)
    }
}

/// Convert from tuple of floats to RGBA
impl From<(f32, f32, f32, f32)> for RGBA {
    fn from(d: (f32, f32, f32, f32)) -> Self {
        let r = (d.0 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let g = (d.1 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let b = (d.2 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let a = (d.3 * 255.0).floor().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, a)
    }
}

impl From<RGBA> for (u8, u8, u8, u8) {
    fn from(d: RGBA) -> (u8, u8, u8, u8) {
        (d.0, d.1, d.2, d.3)
    }
}

impl From<RGBA> for (f32, f32, f32, f32) {
    fn from(d: RGBA) -> (f32, f32, f32, f32) {
        d.to_f32()
    }
}

/// Multiples R,G,B components, keeps A
impl ops::Mul<f32> for RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        let r = (self.0 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let g = (self.1 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let b = (self.2 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, self.3)
    }
}

/// Multiples R,G,B components, keeps A
impl ops::Mul<f32> for &RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        let r = (self.0 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let g = (self.1 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let b = (self.2 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, self.3)
    }
}

/// Adds all components
impl ops::Add<RGBA> for RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);
        let a = self.3.saturating_add(rhs.3); // hmmm?
        RGBA::rgba(r, g, b, a)
    }
}

/// Adds all components
impl ops::Add<RGBA> for &RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);
        let a = self.3.saturating_add(rhs.3); // hmmm?
        RGBA::rgba(r, g, b, a)
    }
}

/// Adds all components
impl ops::Sub<RGBA> for RGBA {
    type Output = RGBA;

    fn sub(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_sub(rhs.0);
        let g = self.1.saturating_sub(rhs.1);
        let b = self.2.saturating_sub(rhs.2);
        let a = self.3.saturating_sub(rhs.3); // hmmm?
        RGBA::rgba(r, g, b, a)
    }
}

/// Converts from text to RGBA
///
/// Panics if the conversion fails.
/// Uses [`get_color`]
impl From<&str> for RGBA {
    fn from(t: &str) -> Self {
        match get_color(t) {
            Err(_) => panic!("Invalid color: {}", t),
            Ok(rgba) => rgba,
        }
    }
}

/// Converts from text to RGBA
///
/// Uses [`get_color`]
impl FromStr for RGBA {
    type Err = ColorParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_color(s)
    }
}
