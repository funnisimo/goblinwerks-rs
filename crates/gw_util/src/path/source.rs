// use crate::world::{global_world, EntityId};
use crate::point::{distance, Point};

/// Implement this trait to support path-finding functions.
pub trait PathfindingSource {
    /// Return Some(#) if move is possible, None means blocked
    #[allow(unused_variables)]
    fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
        Some(1.0)
    }

    fn get_size(&self) -> (u32, u32);

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    /// Default implementation returns Pythagorean distance, which works for most implementations where you can move diagonally.
    fn estimate_pathing_distance(&self, a: &Point, b: &Point) -> f32 {
        distance::precise(a, b)
    }
}

// impl<F> PathfindingSource for F
// where
//     F: Fn(i32, i32) -> Option<f32>,
// {
//     fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
//         self(x, y)
//     }
// }

// Implement this trait to support path-finding functions.
pub trait BlockedSource {
    /// Return Some(#) if move is possible, None means blocked
    #[allow(unused_variables)]
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        false
    }
}

impl<F> BlockedSource for F
where
    F: Fn(i32, i32) -> bool,
{
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        self(x, y)
    }
}

// pub struct EntitySource {
//     pub allow_diagonal: bool,
// }

// impl EntitySource {
//     pub fn new(_entity: EntityId) -> Self {
//         EntitySource {
//             allow_diagonal: true,
//         }
//     }
// }

// impl PathfindingSource for EntitySource {
//     fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
//         match global_world().map().borrow().get_tile_at_xy(x, y) {
//             None => return None,
//             Some(tile) => {
//                 if tile.is_obstruction() {
//                     return Some(OBSTRUCTION);
//                 } else if tile.blocks() {
//                     return Some(BLOCKED);
//                 }
//             }
//         }
//         if global_world().map().borrow().blocked_xy(x, y) {
//             return Some(AVOIDED);
//         }
//         Some(OK)
//     }

//     fn get_size(&self) -> (u32, u32) {
//         let map = global_world().map().borrow();
//         (map.width, map.height)
//     }
// }

// impl BlockedSource for EntitySource {
//     fn is_blocked(&self, x: i32, y: i32) -> bool {
//         if !global_world().map().borrow().has_xy(x, y) {
//             return false;
//         }
//         let idx = global_world().map().borrow().to_idx(x, y);
//         global_world().map().borrow().blocked[idx]
//     }
// }

// pub struct HeroSource {
//     pub allow_diagonal: bool,
//     start: Point,
// }

// impl HeroSource {
//     pub fn new() -> Self {
//         HeroSource {
//             allow_diagonal: true,
//             start: global_world().hero_point(),
//         }
//     }
// }

// impl PathfindingSource for HeroSource {
//     fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
//         let is_visible = match global_world().get_fov(global_world().hero_entity()) {
//             None => false,
//             Some(fov) => {
//                 let fov = fov.borrow();
//                 if !fov.is_revealed(x, y) && !fov.is_mapped(x, y) {
//                     return None;
//                 }
//                 fov.is_visible(x, y)
//             }
//         };

//         if self.start.x == x && self.start.y == y {
//             return Some(OK);
//         }

//         let map = global_world().map().borrow();
//         match map.get_tile_at_xy(x, y) {
//             None => return None,
//             Some(tile) => {
//                 if tile.is_obstruction() {
//                     return Some(OBSTRUCTION);
//                 } else if tile.blocks() {
//                     return Some(BLOCKED);
//                 }
//             }
//         }
//         if map.has_blocker_xy(x, y) && is_visible {
//             return Some(BLOCKED);
//         }
//         Some(OK)
//     }

//     fn get_size(&self) -> (u32, u32) {
//         let map = global_world().map().borrow();
//         (map.width, map.height)
//     }
// }

// impl BlockedSource for HeroSource {
//     fn is_blocked(&self, x: i32, y: i32) -> bool {
//         let map = global_world().map().borrow();
//         if let Some(fov) = global_world().get_fov(global_world().hero_entity()) {
//             let fov = fov.borrow();
//             if !fov.is_revealed(x, y) && !fov.is_mapped(x, y) {
//                 return true;
//             }
//             if !fov.is_visible(x, y) {
//                 return false;
//             }
//         }
//         map.blocked_xy(x, y)
//     }
// }

#[cfg(test)]

pub struct TestSource {
    width: u32,
    height: u32,
    pub cost_func: Option<Box<dyn Fn(i32, i32) -> Option<f32>>>,
    pub block_func: Option<Box<dyn Fn(i32, i32) -> bool>>,
}

#[cfg(test)]
impl TestSource {
    pub fn new(width: u32, height: u32) -> Self {
        TestSource {
            width,
            height,
            cost_func: None,
            block_func: None,
        }
    }
}

#[cfg(test)]
impl PathfindingSource for TestSource {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
        match self.cost_func.as_ref() {
            None => Some(1.0),
            Some(func) => func(x, y),
        }
    }
}

#[cfg(test)]
impl BlockedSource for TestSource {
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        match self.block_func.as_ref() {
            None => false,
            Some(func) => func(x, y),
        }
    }
}
