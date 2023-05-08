use super::{Cell, Map};
use gw_ecs::World;
use gw_util::{
    grid::Grid,
    mask::get_area_mask,
    path::{BlockedSource, PathfindingSource},
};

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

    fn size(&self) -> (u32, u32) {
        Map::size(self)
    }

    fn wrap(&self) -> gw_util::xy::Wrap {
        self.wrap
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

pub struct AreaGrid(Grid<u8>);

impl AreaGrid {
    pub fn new(grid: Grid<u8>) -> Self {
        AreaGrid(grid)
    }

    pub fn grid(&self) -> &Grid<u8> {
        &self.0
    }

    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        match self.0.get(x, y) {
            None => None,
            Some(v) => Some(*v),
        }
    }
}

pub fn ensure_area_grid(world: &mut World) {
    if !world.has_resource::<AreaGrid>() {
        let map = world.read_resource::<Map>();
        let grid = get_area_mask(&*map);
        drop(map);
        world.insert_resource(AreaGrid(grid));
    }
}
