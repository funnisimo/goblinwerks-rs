use crate::map::Cell;
use crate::map::Map;
use gw_util::point::Point;
use std::cmp::min;

// pub mod bracket;
pub mod goblin;
pub mod symmetric;

mod mask;
pub use mask::*;

mod fov;
pub use fov::*;

// CREDIT - This is adapted from: http://roguebasin.roguelikedevelopment.org/index.php?title=Improved_Shadowcasting_in_Java

pub enum FovCalc {
    // Bracket,
    Goblin,
    Symmetric,
}

impl FovCalc {
    pub fn calculate<S: FovSource, T: FovTarget>(
        &self,
        source: &S,
        origin: Point,
        radius: u32,
        target: &mut T,
    ) {
        match self {
            // FovCalc::Bracket => bracket::calculate_fov(source, origin, radius, target),
            FovCalc::Goblin => goblin::calculate_fov(source, origin, radius, target),
            FovCalc::Symmetric => symmetric::calculate_fov(source, origin, radius, target),
        }
    }

    pub fn get_mask<S: FovSource>(&self, source: &S, origin: Point, radius: u32) -> FOVMask {
        let size = source.get_size();
        let mut mask = FOVMask::new(size.0 as i32, size.1 as i32);
        self.calculate(source, origin, radius, &mut mask);
        mask
    }
}

pub trait FovSource {
    fn get_size(&self) -> (u32, u32) {
        (0, 0)
    }

    #[allow(unused_variables)]
    fn is_opaque(&self, x: i32, y: i32) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn has_xy(&self, x: i32, y: i32) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn calc_radius(&self, x: i32, y: i32) -> f32 {
        let lo = min(x.abs(), y.abs());
        if x > lo {
            x.abs() as f32 + lo as f32 * 0.4
        } else {
            y.abs() as f32 + lo as f32 * 0.4
        }
    }
}

pub trait FovTarget {
    #[allow(unused_variables)]
    fn set_visible(&mut self, x: i32, y: i32, pct: f32) {}

    #[allow(unused_variables)]
    fn reset(&mut self, width: u32, height: u32) {}
}

impl FovSource for Map {
    fn is_opaque(&self, x: i32, y: i32) -> bool {
        let index = match self.get_wrapped_index(x, y) {
            None => return true,
            Some(idx) => idx,
        };

        self.get_cell(index).unwrap().blocks_vision()
    }

    fn has_xy(&self, x: i32, y: i32) -> bool {
        Map::try_wrap_xy(&self, x, y).is_some()
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
