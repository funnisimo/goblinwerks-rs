#[derive(Debug, PartialEq)]
pub struct Extents(pub f32, pub f32, pub f32, pub f32);

impl Extents {
    pub fn new() -> Self {
        Extents(0.0, 0.0, 1.0, 1.0)
    }

    pub fn contains(&self, pt: (f32, f32)) -> bool {
        if self.0 > pt.0 || self.3 < pt.0 || self.1 > pt.1 || self.3 < pt.1 {
            return false;
        }
        true
    }
}

impl Default for Extents {
    fn default() -> Self {
        Extents::new()
    }
}

impl From<(f32, f32, f32, f32)> for Extents {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Extents(value.0, value.1, value.2, value.3)
    }
}
