use super::{FovSource, FovTarget};
use gw_util::point::Point;
use num_rational::Rational32;
use std::{iter::Map, ops::RangeInclusive};

enum Cardinal {
    North,
    East,
    South,
    West,
}

struct Quadrant {
    cardinal: Cardinal,
    origin: Point,
}

impl Quadrant {
    pub fn new(cardinal: Cardinal, origin: Point) -> Self {
        Self { cardinal, origin }
    }

    pub fn transform(&self, tile: Tile) -> Point {
        match self.cardinal {
            Cardinal::North => Point::new(self.origin.x + tile.column, self.origin.y - tile.depth),
            Cardinal::East => Point::new(self.origin.x + tile.column, self.origin.y + tile.depth),
            Cardinal::South => Point::new(self.origin.x + tile.depth, self.origin.y + tile.column),
            Cardinal::West => Point::new(self.origin.x - tile.depth, self.origin.y + tile.column),
        }
    }
}

struct Scanline {
    pub depth: i32,
    pub start_slope: Rational32,
    pub end_slope: Rational32,
}

#[derive(Clone, Copy)]
struct Tile {
    pub depth: i32,
    pub column: i32,
}

impl Scanline {
    fn with_integers(depth: i32, start_slope: i32, end_slope: i32) -> Self {
        Self::new(
            depth,
            Rational32::from_integer(start_slope),
            Rational32::from_integer(end_slope),
        )
    }

    fn new(depth: i32, start_slope: Rational32, end_slope: Rational32) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    fn tiles(&self) -> Map<RangeInclusive<i32>, impl FnMut(i32) -> Tile> {
        let start_column = round_ties_up(Rational32::from_integer(self.depth) * self.start_slope);
        let end_column = round_ties_down(Rational32::from_integer(self.depth) * self.end_slope);
        let depth = self.depth;
        (start_column..=end_column)
            .into_iter()
            .map(move |column| Tile { depth, column })
    }

    fn next(&self) -> Self {
        Self::new(self.depth + 1, self.start_slope, self.end_slope)
    }
}

fn slope(tile: Tile) -> Rational32 {
    Rational32::new(2 * tile.column - 1, 2 * tile.depth)
}

fn round_ties_up(r: Rational32) -> i32 {
    (r + Rational32::new(1, 2)).floor().to_integer()
}

fn round_ties_down(r: Rational32) -> i32 {
    (r - Rational32::new(1, 2)).ceil().to_integer()
}

fn is_symmetric(scanline: &Scanline, tile: Tile) -> bool {
    let column = Rational32::from_integer(tile.column);
    let depth = Rational32::from_integer(scanline.depth);
    column >= depth * scanline.start_slope && column <= depth * scanline.end_slope
}

struct FovScanner<'a, S: FovSource, T: FovTarget> {
    radius_2: i32,
    radius_plus_half_2: Rational32,
    quadrant: Quadrant,
    source: &'a S,
    target: &'a mut T,
}

impl<S: FovSource, T: FovTarget> FovScanner<'_, S, T> {
    fn reveal(&mut self, tile: Tile) {
        let point = self.quadrant.transform(tile);
        self.target.set_visible(point.x, point.y, 1.0);
    }

    fn is_opaque(&mut self, tile: Tile) -> bool {
        let point = self.quadrant.transform(tile);
        if let Some((x, y)) = self.source.try_wrap_xy(point.x, point.y) {
            self.source.is_opaque(x, y)
        } else {
            true
        }
    }

    fn scan(&mut self, first_line: Scanline) {
        let mut stack = vec![first_line];
        let mut prev_tile;
        while let Some(mut scanline) = stack.pop() {
            if scanline.depth * scanline.depth > self.radius_2 {
                continue;
            }
            prev_tile = None;
            for tile in scanline.tiles() {
                let tile_point = self.quadrant.transform(tile);
                let dx = tile_point.x - self.quadrant.origin.x;
                let dy = tile_point.y - self.quadrant.origin.y;
                if Rational32::from_integer(dx * dx + dy * dy) <= self.radius_plus_half_2 {
                    if self.is_opaque(tile) || is_symmetric(&scanline, tile) {
                        self.reveal(tile);
                    }
                    if let Some(prev_tile) = prev_tile {
                        if self.is_opaque(prev_tile) && !self.is_opaque(tile) {
                            scanline.start_slope = slope(tile);
                        }
                        if !self.is_opaque(prev_tile) && self.is_opaque(tile) {
                            let mut next_line = scanline.next();
                            next_line.end_slope = slope(tile);
                            stack.push(next_line);
                        }
                    }
                    prev_tile = Some(tile);
                }
            }
            if let Some(prev_tile) = prev_tile {
                if !self.is_opaque(prev_tile) {
                    stack.push(scanline.next());
                }
            }
        }
    }
}

/// Calculates field-of-view (symmetric version) for a map that supports Algorithm2D, returning a HashSet.
/// This is a bit faster than coercing the results into a vector, since internally it uses the set for de-duplication.
pub fn calculate_fov<S: FovSource, T: FovTarget>(
    source: &S,
    origin: Point,
    radius: u32,
    target: &mut T,
) {
    // NOTE: Symmetric shadowcasting algorithm adapted from https://www.albertford.com/shadowcasting/ (CC0)

    let size = source.get_size();
    target.reset(size.0, size.1);
    target.set_visible(origin.x, origin.y, 1.0);

    let radius_plus_half = Rational32::from_integer(radius as i32) + Rational32::new(1, 2);
    let radius_plus_half_2 = radius_plus_half * radius_plus_half;
    let radius_2 = (radius * radius) as i32;

    for cardinal in [
        Cardinal::North,
        Cardinal::East,
        Cardinal::South,
        Cardinal::West,
    ] {
        let mut scanner = FovScanner {
            radius_2,
            radius_plus_half_2,
            quadrant: Quadrant::new(cardinal, origin),
            source,
            target,
        };
        let first_line = Scanline::with_integers(1, -1, 1);
        scanner.scan(first_line);
    }
}
