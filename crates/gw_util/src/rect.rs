use crate::point::Point;
use std::ops;

/// Defines a two-dimensional rectangle.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    /// The X position of the first point (typically the left)
    pub x1: i32,
    /// The X position of the second point (typically the right)
    pub x2: i32,
    /// The Y position of the first point (typically the top)
    pub y1: i32,
    /// The Y position of the second point (typically the bottom)
    pub y2: i32,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect::zero()
    }
}

impl Rect {
    /// Create a new rectangle, specifying X/Y Width/Height
    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w - 1,
            y2: y + h - 1,
        }
    }

    /// Create a new rectangle, specifying exact dimensions
    pub fn with_bounds(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect {
            x1: left,
            y1: top,
            x2: right,
            y2: bottom,
        }
    }

    /// Creates a zero rectangle
    pub fn zero() -> Rect {
        Rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }

    /// Returns true if this overlaps with other
    #[must_use]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center of the rectangle
    #[must_use]
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Returns true if a point is inside the rectangle
    #[must_use]
    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.x1 <= x && self.x2 > x && self.y1 <= y && self.y2 > y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f(Point::new(x, y));
            }
        }
    }

    /// Returns the rectangle's width
    #[must_use]
    pub fn width(&self) -> i32 {
        i32::abs(self.x2 - self.x1) + 1
    }

    /// Returns the rectangle's height
    #[must_use]
    pub fn height(&self) -> i32 {
        i32::abs(self.y2 - self.y1) + 1
    }
}

impl ops::Add<Rect> for Rect {
    type Output = Rect;
    fn add(mut self, rhs: Rect) -> Rect {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        let rect = Rect::with_size(0, 0, 10, 10);
        assert!(rect.width() == 10);
        assert!(rect.height() == 10);
    }

    #[test]
    fn test_add() {
        let rect = Rect::with_size(0, 0, 10, 10) + Rect::with_size(1, 1, 1, 1);
        assert!(rect.x1 == 1 && rect.y1 == 1);
        assert!(rect.x2 == 11 && rect.y2 == 11);
    }

    #[test]
    fn test_intersect() {
        let r1 = Rect::with_size(0, 0, 10, 10);
        let r2 = Rect::with_size(5, 5, 10, 10);
        let r3 = Rect::with_size(100, 100, 5, 5);
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_center() {
        let r1 = Rect::with_size(0, 0, 10, 9);
        let center = r1.center();
        assert_eq!(center, Point::new(4, 4)); // floor

        let r1 = Rect::with_size(0, 0, 11, 10);
        let center = r1.center();
        assert_eq!(center, Point::new(5, 4)); // floor
    }

    #[test]
    fn test_contains_point() {
        let r1 = Rect::with_size(0, 0, 10, 10);
        assert!(r1.contains_point(&Point::new(5, 5)));
        assert!(!r1.contains_point(&Point::new(100, 100)));
    }

    #[test]
    fn test_rect_callback() {
        use std::collections::HashSet;

        let r1 = Rect::with_size(0, 0, 1, 1);
        let mut points: HashSet<Point> = HashSet::new();
        r1.for_each(|p| {
            points.insert(p);
        });
        assert!(points.contains(&Point::new(0, 0)));
        assert!(!points.contains(&Point::new(1, 0)));
        assert!(!points.contains(&Point::new(0, 1)));
        assert!(!points.contains(&Point::new(1, 1)));
    }
}
