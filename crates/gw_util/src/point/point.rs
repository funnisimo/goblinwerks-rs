use super::distance;
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Display, ops};

pub static DIRS: [Point; 8] = [
    Point::new(0, -1), // Up
    Point::new(1, 0),  // Right
    Point::new(0, 1),  // Down
    Point::new(-1, 0), // Left
    Point::new(1, -1),
    Point::new(1, 1),
    Point::new(-1, 1),
    Point::new(-1, -1),
];

// TODO - Should this be in CSS order -> left, top, right, bottom?
pub static DIRS4: [Point; 4] = [
    Point::new(0, -1), // Up
    Point::new(1, 0),  // Right
    Point::new(0, 1),  // Down
    Point::new(-1, 0), // Left
];

pub struct NeighborIterator {
    x: i32,
    y: i32,
    idx: usize,
    with_diagonals: bool,
}

impl NeighborIterator {
    pub fn new(x: i32, y: i32, with_diagonals: bool) -> Self {
        NeighborIterator {
            x,
            y,
            idx: 0,
            with_diagonals,
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = (i32, i32, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx > 3 && !self.with_diagonals) || self.idx >= DIRS.len() {
            return None;
        }
        let dir = DIRS[self.idx];
        self.idx += 1;
        Some((self.x + dir.x, self.y + dir.y, self.idx > 4))
    }
}

#[derive(Debug)]
pub struct RingIterator {
    x: i32,
    y: i32,
    radius: u32,
    len: u32,
    idx: u32,
}

impl RingIterator {
    pub fn new(x: i32, y: i32, radius: u32) -> Self {
        let len = (1 + radius + radius).pow(2);
        RingIterator {
            x,
            y,
            radius,
            len,
            idx: 0,
        }
    }
}

impl Iterator for RingIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let size = 1 + self.radius + self.radius;

        if self.radius == 0 {
            self.idx += 1;
            return match self.idx {
                1 => Some((self.x, self.y)),
                _ => None,
            };
        }

        let min_len = (self.radius - 1).pow(2) as f32;
        let max_len = self.radius.pow(2) as f32;

        // println!(
        //     "iter next : size={}, min_len={}, max_len={}",
        //     size, min_len, max_len
        // );

        loop {
            if self.idx >= self.len {
                // println!("iter idx = {} :: DONE", self.idx);
                return None;
            }

            let dx: i32 = (self.idx % size) as i32 - self.radius as i32;
            let dy: i32 = (self.idx / size) as i32 - self.radius as i32;

            // println!("- idx={}, dx={},dy={}", self.idx, dx, dy);

            self.idx += 1;

            let radius = Point::new(dx, dy).len_squared();
            if radius > min_len && radius <= max_len {
                let x = dx as i32 + self.x;
                let y = dy as i32 + self.y;
                // println!("iter idx = {} :: {},{}", self.idx, dx, dy);
                return Some((x, y));
            } else {
                // println!(
                //     "iter idx !! {} :: {},{} = {} vs {}<->{}",
                //     self.idx - 1,
                //     dx,
                //     dy,
                //     radius,
                //     min_len,
                //     max_len
                // );
            }
        }
    }
}

pub const ZERO_POINT: Point = Point { x: 0, y: 0 };

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Default, Serialize, Deserialize)]
/// Helper struct defining a 2D point in space.
pub struct Point {
    /// The point's X location
    pub x: i32,
    /// The point's Y location
    pub y: i32,
}

impl Point {
    /// Create a new point from an x/y coordinate.
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// Create a zero point
    #[inline]
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    /// Get a point from a row-major index - (x,y).
    pub fn from_index_xy(index: usize, width: usize, height: usize) -> Point {
        let _ = height;
        let x = index % width;
        let y = index / width;
        Point::new(x as i32, y as i32)
    }

    /// Get a point from a column-major index - (y,x).
    pub fn from_index_yx(index: usize, width: usize, height: usize) -> Point {
        let _ = width;
        let x = index / height;
        let y = index % height;
        Point::new(x as i32, y as i32)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn as_dir(&self) -> Self {
        if self.x < -1 || self.x > 1 || self.y < -1 || self.y > 1 {
            if self.x.abs() > self.y.abs() {
                return Self::new(self.x.signum(), 0);
            } else if self.y.abs() > self.x.abs() {
                return Self::new(0, self.y.signum());
            } else {
                return Self::new(self.x.signum(), self.y.signum());
            }
        }
        Self::new(self.x, self.y)
    }

    pub fn len(&self) -> f32 {
        distance::simple(&ZERO_POINT, self)
    }

    pub fn len_squared(&self) -> f32 {
        distance::squared(&ZERO_POINT, self)
    }

    pub fn len_precise(&self) -> f32 {
        distance::precise(&ZERO_POINT, self)
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_point(&mut self, other: &Point) {
        self.set(other.x, other.y);
    }

    pub fn distance(&self, other: &Point) -> f32 {
        distance::simple(self, other)
    }

    pub fn neighbors(&self, with_diagonals: bool) -> NeighborIterator {
        NeighborIterator::new(self.x, self.y, with_diagonals)
    }

    pub fn ring(&self, radius: u32) -> RingIterator {
        RingIterator::new(self.x, self.y, radius)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(item: (i32, i32)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(item: (f32, f32)) -> Self {
        Self {
            x: item.0 as i32,
            y: item.1 as i32,
        }
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Point {
        Point::new(self.x + rhs, self.y + rhs)
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point {
    type Output = Point;
    fn sub(self, rhs: i32) -> Point {
        Point::new(self.x - rhs, self.y - rhs)
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point> for Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

/// Support multiplying a point by an f32
impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Point {
        Point::new((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
    }
}

/// Support dividing a point by a point
impl ops::Div<Point> for Point {
    type Output = Point;
    fn div(self, rhs: Point) -> Point {
        Point::new(self.x / rhs.x, self.y / rhs.y)
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point {
    type Output = Point;
    fn div(self, rhs: i32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

/// Support dividing a point by an f32
impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(self, rhs: f32) -> Point {
        Point::new((self.x as f32 / rhs) as i32, (self.y as f32 / rhs) as i32)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl ops::DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_neighbors() {
        let p = Point::new(3, 6);
        let mut it = p.neighbors(true);
        assert_eq!((3, 5, false), it.next().unwrap());
        assert_eq!((4, 6, false), it.next().unwrap());
        assert_eq!((3, 7, false), it.next().unwrap());
        assert_eq!((2, 6, false), it.next().unwrap());
        assert_eq!((4, 5, true), it.next().unwrap());
        assert_eq!((4, 7, true), it.next().unwrap());
        assert_eq!((2, 7, true), it.next().unwrap());
        assert_eq!((2, 5, true), it.next().unwrap());
        assert_eq!(None, it.next());

        let mut it = p.neighbors(false);
        assert_eq!((3, 5, false), it.next().unwrap());
        assert_eq!((4, 6, false), it.next().unwrap());
        assert_eq!((3, 7, false), it.next().unwrap());
        assert_eq!((2, 6, false), it.next().unwrap());
        assert_eq!(None, it.next());
    }

    #[test]
    fn point_ring_0() {
        let p = Point::new(0, 0);

        let mut count = 0;
        for _ in p.ring(0) {
            count += 1;
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn point_ring_1() {
        let p = Point::new(0, 0);

        let iter = p.ring(1);
        println!("iter = {:?}", iter);

        let mut count = 0;
        for _ in iter {
            count += 1;
        }
        assert_eq!(count, 4);
    }

    #[test]
    fn point_ring_2() {
        let p = Point::new(0, 0);

        let iter = p.ring(2);
        println!("iter = {:?}", iter);

        // --X--
        // -X-X-
        // X-0-X
        // -X-X-
        // --X--

        let mut count = 0;
        for _ in iter {
            count += 1;
        }
        assert_eq!(count, 8);
    }

    #[test]
    fn point_ring_3() {
        let p = Point::new(0, 0);

        let iter = p.ring(3);
        println!("iter = {:?}", iter);

        // --XXX--
        // -X---X-
        // X-----X
        // X--0--X
        // X-----X
        // -X---X-
        // --XXX--

        let mut count = 0;
        for _ in iter {
            count += 1;
        }
        assert_eq!(count, 16);
    }

    #[test]
    fn new_point() {
        let pt = Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn add_point_to_point() {
        let pt = Point::new(0, 0);
        let p2 = pt + Point::new(1, 2);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn add_assign_point_to_point() {
        let mut pt = Point::new(0, 0);
        pt += Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn add_point_to_int() {
        let pt = Point::new(0, 0);
        let p2 = pt + 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn sub_point_to_point() {
        let pt = Point::new(0, 0);
        let p2 = pt - Point::new(1, 2);
        assert_eq!(p2.x, -1);
        assert_eq!(p2.y, -2);
    }

    #[test]
    fn sub_assign_point_to_point() {
        let mut pt = Point::new(0, 0);
        pt -= Point::new(1, 2);
        assert_eq!(pt.x, -1);
        assert_eq!(pt.y, -2);
    }

    #[test]
    fn sub_point_to_int() {
        let pt = Point::new(0, 0);
        let p2 = pt - 2;
        assert_eq!(p2.x, -2);
        assert_eq!(p2.y, -2);
    }

    #[test]
    fn mul_point_to_point() {
        let pt = Point::new(1, 1);
        let p2 = pt * Point::new(1, 2);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn mul_assign_point_to_point() {
        let mut pt = Point::new(1, 1);
        pt *= Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn mul_point_to_int() {
        let pt = Point::new(1, 1);
        let p2 = pt * 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn mul_point_to_float() {
        let pt = Point::new(1, 1);
        let p2 = pt * 4.0;
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 4);
    }

    #[test]
    fn div_point_to_point() {
        let pt = Point::new(4, 4);
        let p2 = pt / Point::new(2, 4);
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 1);
    }

    #[test]
    fn div_assign_point_to_point() {
        let mut pt = Point::new(4, 4);
        pt /= Point::new(2, 4);
        assert_eq!(pt.x, 2);
        assert_eq!(pt.y, 1);
    }

    #[test]
    fn div_point_to_int() {
        let pt = Point::new(4, 4);
        let p2 = pt / 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn div_point_to_float() {
        let pt = Point::new(4, 4);
        let p2 = pt / 2.0;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }
}
