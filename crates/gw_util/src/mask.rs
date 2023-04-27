use crate::{
    grid::{wrapped_flood_replace, Grid},
    path::{BlockedSource, PathfindingSource},
    point::Point,
};

pub fn get_area_mask<T: PathfindingSource + BlockedSource>(source: &T) -> Grid<u8> {
    let size = source.size();

    let mut grid: Grid<u8> = Grid::new(size.0 as usize, size.1 as usize, 0);

    // Mark all the unblocked locations...
    for x in 0..size.0 {
        for y in 0..size.1 {
            if source.is_blocked(x as i32, y as i32) {
                grid.set(x as i32, y as i32, 0);
            } else {
                grid.set(x as i32, y as i32, u8::MAX);
            }
        }
    }

    // start to fill them
    let mut area_count: u8 = 0;
    for x in 0..size.0 {
        for y in 0..size.1 {
            if *grid.get_unchecked(x as i32, y as i32) == u8::MAX {
                area_count = area_count.saturating_add(1);

                wrapped_flood_replace(
                    &mut grid,
                    Point::new(x as i32, y as i32),
                    1,
                    area_count,
                    source.wrap(),
                );
            }
        }
    }

    grid
}
