#![allow(dead_code)]

use super::*;
use crate::point::Point;
// use std::ops::{Add, AddAssign};
use super::{BlockedSource, PathfindingSource};

pub fn get_cost_map<S: PathfindingSource + BlockedSource>(
    goal: Point,
    source: &S,
    allow_diagonal: bool,
) -> SearchGrid {
    let (width, height) = source.get_size();
    let mut grid = SearchGrid::new(width, height);
    grid.set_goal_xy(goal.x, goal.y);
    calculate_costs(&mut grid, source, allow_diagonal);
    grid
}

// pub fn calculate_map_to_me(grid: &mut Box<SearchGrid>, _world: &World, entity: EntityId) {
//     let source = EntitySource::new(entity);
//     calculate_costs(grid, &source, source.allow_diagonal);
// }

pub fn calculate_costs<S: PathfindingSource>(grid: &mut SearchGrid, src: &S, allow_diagonal: bool) {
    loop {
        let current = grid.pop_todo();
        if current.is_none() {
            break;
        }

        let node = match grid.get(current.unwrap()) {
            None => break,
            Some(node) => node.clone(),
        };

        // println!(
        //     "todo: {} @ {} + {}",
        //     node.xy, node.cost_so_far, node.estimate_left
        // );
        for (x, y, is_diagonal) in node.xy.neighbors(allow_diagonal) {
            // println!("   - {}x{}", x, y);
            if grid.to_idx(x, y).is_none() {
                continue;
            }
            let mut mult = 1.0;
            if is_diagonal {
                mult = 1.4;
                // check to see if obstruction blocks this move
                if src.move_cost(x, node.xy.y).unwrap_or(OBSTRUCTION) >= OBSTRUCTION
                    || src.move_cost(node.xy.x, y).unwrap_or(OBSTRUCTION) >= OBSTRUCTION
                {
                    continue;
                }
            }
            let cost = src.move_cost(x, y).unwrap_or(OBSTRUCTION) * mult;

            if grid.update_node_xy(x, y, node.cost_so_far + cost, 0.0) {
                // println!("   - add: {},{} : {}", x, y, node.cost_so_far + cost);
            }
        }
    }
}

pub fn rescan<S: PathfindingSource>(grid: &mut SearchGrid, src: &S, allow_diagonal: bool) {
    grid.clear_todo();

    for idx in 0..grid.len() {
        let (x, y, distance) = {
            let node = &grid.get(idx).unwrap();
            (node.xy.x, node.xy.y, node.cost_so_far)
        };

        if distance < BLOCKED {
            if let Some(idx) = grid.to_idx(x, y) {
                grid.insert_node(idx);
            }
        }
    }

    calculate_costs(grid, src, allow_diagonal)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::path::TestSource;
    use crate::point::DIRS;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn calculate_5x5_top_left() {
        let source = TestSource::new(5, 5);
        let mut grid = SearchGrid::new(5, 5);
        grid.set_goal_xy(0, 0);
        calculate_costs(&mut grid, &source, true);

        println!("{}", grid);
        assert_approx_eq!(grid.distance_xy(0, 0), 0.0, 0.001);
        assert_approx_eq!(grid.distance_xy(0, 1), 1.0, 0.001);
        assert_approx_eq!(grid.distance_xy(1, 0), 1.0, 0.001);
        assert_approx_eq!(grid.distance_xy(1, 1), 1.4, 0.001);
        assert_approx_eq!(grid.distance_xy(0, 2), 2.0, 0.001);
        assert_approx_eq!(grid.distance_xy(2, 0), 2.0, 0.001);
        assert_approx_eq!(grid.distance_xy(2, 2), 2.8, 0.001);
        assert_approx_eq!(grid.distance_xy(0, 3), 3.0, 0.001);
        assert_approx_eq!(grid.distance_xy(3, 0), 3.0, 0.001);
        assert_approx_eq!(grid.distance_xy(3, 3), 4.2, 0.001);
        assert_approx_eq!(grid.distance_xy(0, 4), 4.0, 0.001);
        assert_approx_eq!(grid.distance_xy(1, 4), 4.4, 0.001);
        assert_approx_eq!(grid.distance_xy(2, 4), 4.8, 0.001);
        assert_approx_eq!(grid.distance_xy(3, 4), 5.2, 0.001);
        assert_approx_eq!(grid.distance_xy(4, 4), 5.6, 0.001);
        assert_approx_eq!(grid.distance_xy(4, 3), 5.2, 0.001);
        assert_approx_eq!(grid.distance_xy(4, 2), 4.8, 0.001);
        assert_approx_eq!(grid.distance_xy(4, 1), 4.4, 0.001);
        assert_approx_eq!(grid.distance_xy(4, 0), 4.0, 0.001);
    }

    #[test]
    fn calculate_5x5_top_right() {
        let source = TestSource::new(5, 5);
        let a = get_cost_map(Point::new(4, 0), &source, true);
        println!("{}", a);
        assert_approx_eq!(a.distance_xy(4, 0), 0.0, 0.001);
        assert_approx_eq!(a.distance_xy(4, 1), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 0), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 1), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 2), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 0), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 2), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 3), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 0), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 3), 4.2, 0.001);
        assert_approx_eq!(a.distance_xy(4, 4), 4.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 4), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(2, 4), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(1, 4), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(0, 4), 5.6, 0.001);
        assert_approx_eq!(a.distance_xy(0, 3), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(0, 2), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 1), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(0, 0), 4.0, 0.001);
    }

    #[test]
    fn calculate_5x5_bottom_right() {
        let source = TestSource::new(5, 5);
        let a = get_cost_map(Point::new(4, 4), &source, true);
        println!("{}", a);
        assert_approx_eq!(a.distance_xy(4, 4), 0.0, 0.001);
        assert_approx_eq!(a.distance_xy(4, 3), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 4), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 3), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 2), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 4), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 2), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 1), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 4), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 1), 4.2, 0.001);
        assert_approx_eq!(a.distance_xy(4, 0), 4.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 0), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(2, 0), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(1, 0), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(0, 0), 5.6, 0.001);
        assert_approx_eq!(a.distance_xy(0, 1), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(0, 2), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 3), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(0, 4), 4.0, 0.001);
    }

    #[test]
    fn calculate_5x5_bottom_left() {
        let source = TestSource::new(5, 5);
        let a = get_cost_map(Point::new(0, 4), &source, true);
        println!("{}", a);
        assert_approx_eq!(a.distance_xy(0, 4), 0.0, 0.001);
        assert_approx_eq!(a.distance_xy(0, 3), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 4), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 3), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(0, 2), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 4), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 2), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 1), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 4), 3.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 1), 4.2, 0.001);
        assert_approx_eq!(a.distance_xy(0, 0), 4.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 0), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(2, 0), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(3, 0), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(4, 0), 5.6, 0.001);
        assert_approx_eq!(a.distance_xy(4, 1), 5.2, 0.001);
        assert_approx_eq!(a.distance_xy(4, 2), 4.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 3), 4.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 4), 4.0, 0.001);
    }

    #[test]
    fn calculate_5x5_middle() {
        let source = TestSource::new(5, 5);
        let a = get_cost_map(Point::new(2, 2), &source, true);
        println!("{}", a);
        assert_approx_eq!(a.distance_xy(2, 2), 0.0, 0.001);

        assert_approx_eq!(a.distance_xy(1, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 1), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 3), 1.0, 0.001);

        assert_approx_eq!(a.distance_xy(1, 1), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(3, 3), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(3, 1), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(1, 3), 1.4, 0.001);

        assert_approx_eq!(a.distance_xy(0, 2), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 0), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(4, 2), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 4), 2.0, 0.001);

        assert_approx_eq!(a.distance_xy(0, 0), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 0), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 4), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 4), 2.8, 0.001);
    }

    #[test]
    fn calculate_5x5_off_center() {
        let source = TestSource::new(5, 5);
        let a = get_cost_map(Point::new(1, 2), &source, true);
        println!("{}", a);
        assert_approx_eq!(a.distance_xy(1, 2), 0.0, 0.001);

        assert_approx_eq!(a.distance_xy(0, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 1), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(1, 3), 1.0, 0.001);

        assert_approx_eq!(a.distance_xy(0, 0), 2.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 0), 3.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 4), 2.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 4), 3.8, 0.001);

        assert_approx_eq!(a.distance_xy(0, 1), 1.4, 0.001);
        assert_approx_eq!(a.distance_xy(1, 0), 2.0, 0.001);

        assert_approx_eq!(a.distance_xy(4, 1), 3.4, 0.001);
        assert_approx_eq!(a.distance_xy(3, 0), 2.8, 0.001);

        assert_approx_eq!(a.distance_xy(1, 4), 2.0, 0.001);
        assert_approx_eq!(a.distance_xy(0, 3), 1.4, 0.001);

        assert_approx_eq!(a.distance_xy(3, 4), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 3), 3.4, 0.001);
    }

    #[test]
    fn calculate_5x5_with_block() {
        let mut source = TestSource::new(5, 5);
        source.cost_func = Some(Box::new(|x, y| {
            if x == 1 && y == 3 {
                return Some(BLOCKED);
            }
            if x == 2 && y == 3 {
                return Some(BLOCKED);
            }
            Some(1.0)
        }));
        let a = get_cost_map(Point::new(2, 2), &source, true);

        assert_approx_eq!(a.distance_xy(2, 2), 0.0, 0.001);

        assert_approx_eq!(a.distance_xy(1, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 1), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(3, 2), 1.0, 0.001);
        assert_approx_eq!(a.distance_xy(2, 3), BLOCKED, 0.001);

        assert_approx_eq!(a.distance_xy(0, 0), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(4, 0), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(0, 4), 3.4, 0.001);
        assert_approx_eq!(a.distance_xy(4, 4), 2.8, 0.001);

        assert_approx_eq!(a.distance_xy(0, 3), 2.4, 0.001);
        assert_approx_eq!(a.distance_xy(1, 3), BLOCKED, 0.001);

        assert_approx_eq!(a.distance_xy(0, 4), 3.4, 0.001);
        assert_approx_eq!(a.distance_xy(1, 4), 3.8, 0.001);
        assert_approx_eq!(a.distance_xy(2, 4), 2.8, 0.001);
        assert_approx_eq!(a.distance_xy(3, 4), 2.4, 0.001);

        let is_blocked = |_, _| false;
        assert_eq!(a.next_dir(0, 4, &is_blocked, true), Some(&DIRS[0]));
        assert_eq!(a.next_dir(0, 3, &is_blocked, true), Some(&DIRS[4]));
        assert_eq!(a.next_dir(1, 2, &is_blocked, true), Some(&DIRS[1]));
        assert_eq!(a.next_dir(2, 2, &is_blocked, true), None);

        assert_eq!(
            a.path_from_xy(0, 4, &is_blocked, true)
                .collect::<Vec<Point>>(),
            vec![Point::new(0, 3), Point::new(1, 2), Point::new(2, 2)]
        );

        assert_eq!(
            a.path_from_xy(0, 4, &is_blocked, false)
                .collect::<Vec<Point>>(),
            vec![
                Point::new(0, 3),
                Point::new(0, 2),
                Point::new(1, 2),
                Point::new(2, 2)
            ]
        );
    }

    // #[test]
    // fn add_assign() {
    //     let mut a = CostMap::alloc(10, 10);
    //     let b = CostMap::alloc(10, 10);
    //     a += &b;
    //     SearchGrid::free(b);
    //
    // }

    // #[test]
    // fn add() {
    //     let a = CostMap::alloc(10, 10);
    //     let b = CostMap::alloc(10, 10);
    //     let c = &a + &b;
    //     SearchGrid::free(b);
    //
    //     SearchGrid::free(c);
    // }
}
