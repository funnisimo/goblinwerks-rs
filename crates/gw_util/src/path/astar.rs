use super::BlockedSource;
use super::PathfindingSource;
use super::SearchGrid;
use super::OBSTRUCTION;
use crate::point::Point;

/// Bail out if the A* search exceeds this many steps.

/// Request an A-Star search. The start and end are specified as index numbers (compatible with your
/// PathfindingSource implementation), and it requires access to your map so as to call distance and exit determinations.
pub fn a_star_search<S: PathfindingSource + BlockedSource>(
    start: Point,
    end: Point,
    map: &S,
    allow_diagonal: bool,
) -> Option<Vec<Point>> {
    AStar::new(map).search(start, end, allow_diagonal)
}

/// Holds the result of an A-Star navigation query.
/// `destination` is the index of the target tile.
/// `success` is true if it reached the target, false otherwise.
/// `steps` is a vector of each step towards the target, *including* the starting position.
// #[derive(Clone)]
// pub struct NavigationPath {
//     pub start: Point,
//     pub end: Point,
//     pub steps: Vec<Point>,
// }

// impl Default for NavigationPath {
//     fn default() -> Self {
//         NavigationPath::new()
//     }
// }

// impl NavigationPath {
//     /// Makes a new (empty) NavigationPath
//     pub fn new() -> NavigationPath {
//         NavigationPath {
//             end: Point::new(0, 0),
//             start: Point::new(0, 0),
//             steps: Vec::new(),
//         }
//     }
// }

/// Private structure for calculating an A-Star navigation path.
struct AStar<'a, S>
where
    S: PathfindingSource + BlockedSource,
{
    source: &'a S,
}

impl<'a, S> AStar<'a, S>
where
    S: PathfindingSource + BlockedSource,
{
    /// Creates a new path, with specified starting and ending indices.
    fn new(source: &'a S) -> Self {
        AStar { source }
    }

    /// Performs an A-Star search
    fn search(self, start: Point, end: Point, allow_diagonal: bool) -> Option<Vec<Point>> {
        // println!("astar: {} -> {}", start, end);
        let (width, height) = self.source.get_size();
        let mut cost_map = SearchGrid::new(width, height);

        cost_map.set_goal_xy(end.x, end.y);

        loop {
            let current_idx = cost_map.pop_todo();
            if current_idx.is_none() {
                break;
            }

            let node = match cost_map.get(current_idx.unwrap()) {
                None => break,
                Some(node) => node.clone(),
            };

            // println!(
            //     "astar - node {} : {:.1} + {:.1} = {:.1}",
            //     node.xy,
            //     node.cost_so_far,
            //     node.estimate_left,
            //     node.score()
            // );

            if node.xy.x == start.x && node.xy.y == start.y {
                // println!("astar done - {} nodes", cost_map.count);
                let path = Some(
                    cost_map
                        .path_from(start, self.source, allow_diagonal)
                        .collect(),
                );
                return path;
            }

            for (x, y, is_diagonal) in node.xy.neighbors(allow_diagonal) {
                // println!("   - {}x{}", x, y);
                if cost_map.to_idx(x, y).is_none() {
                    continue;
                }
                let mut mult = 1.0;
                if is_diagonal {
                    if !allow_diagonal {
                        continue;
                    }
                    mult = 1.4;
                    // check to see if diagonal obstructions block this move
                    if self.source.move_cost(x, node.xy.y).unwrap_or(OBSTRUCTION) >= OBSTRUCTION
                        || self.source.move_cost(node.xy.x, y).unwrap_or(OBSTRUCTION) >= OBSTRUCTION
                    {
                        continue;
                    }
                }
                let cost = self.source.move_cost(x, y).unwrap_or(OBSTRUCTION) * mult;

                let current = Point::new(x, y);
                let cost = node.cost_so_far + cost;
                let estimate = self.source.estimate_pathing_distance(&current, &start);
                if cost_map.update_node_xy(x, y, cost, estimate) {
                    // console.log('- add', x, y, current!.distance + cost);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::path::TestSource;

    #[test]
    fn calculate_5x5_middle() {
        let source = TestSource::new(5, 5);
        let path = a_star_search(Point::new(2, 2), Point::new(0, 0), &source, false);
        assert!(path.is_some());
        assert_eq!(
            path.unwrap(),
            [(2, 1).into(), (2, 0).into(), (1, 0).into(), (0, 0).into()]
        );
    }
}
