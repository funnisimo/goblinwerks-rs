use super::{Cell, Map};
use gw_util::path::{BlockedSource, PathfindingSource};

impl PathfindingSource for Map {
    // Handled in default for Trait
    // fn estimate_pathing_distance(&self, a: Point, b: Point) -> f32 {
    //     DistanceAlg::Pythagoras.distance2d(a, b)
    // }

    fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
        let idx = match self.get_wrapped_index(x, y) {
            None => return None,
            Some(idx) => idx,
        };

        let cell = self.get_cell(idx).unwrap();

        if cell.blocks() {
            return None;
        }

        if self.blocked[idx] {
            // TODO - Allies?  Enemies?  Fixture?
            return Some(5.0);
        }

        Some(1.0)
    }

    fn get_size(&self) -> (u32, u32) {
        self.size()
    }
}

impl BlockedSource for Map {
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        let idx = match self.get_wrapped_index(x, y) {
            None => return true,
            Some(idx) => idx,
        };

        let cell = self.get_cell(idx).unwrap();

        if cell.blocks() {
            return true;
        }

        if self.blocked[idx] {
            // TODO - Allies?  Enemies?  Fixture?
            return true;
        }
        false
    }
}

// impl Algorithm2D for Map {
//     fn dimensions(&self) -> Point {
//         Point::new(self.width, self.height)
//     }
// }
