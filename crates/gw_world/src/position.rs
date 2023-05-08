use gw_ecs::{Component, DenseVecStorage};
use gw_util::point::Point;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::convert::Into;

// #[derive(Component, Default, Clone, Copy, Debug)]
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub blocks_move: bool, // TODO - Is this still used?
    pub map_id: u32,
}

impl Position {
    pub fn new<X: Into<i32>, Y: Into<i32>>(x: X, y: Y) -> Self {
        Position {
            x: x.into(),
            y: y.into(),
            blocks_move: true,
            map_id: 0,
        }
    }

    pub fn with_blocking(mut self, blocks_move: bool) -> Self {
        self.blocks_move = blocks_move;
        self
    }

    pub fn with_map(mut self, map_id: u32) -> Self {
        self.map_id = map_id;
        self
    }

    // pub fn from_point(point: Point) -> Position {
    //     Position {
    //         x: point.x,
    //         y: point.y,
    //         blocks_move: true,
    //         map_id: 0,
    //     }
    // }

    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_level(&mut self, x: i32, y: i32, level: u32) {
        self.x = x;
        self.y = y;
        self.map_id = level;
    }

    pub fn is_xy(&self, x: i32, y: i32, map_id: u32) -> bool {
        self.x == x && self.y == y && self.map_id == map_id
    }
}

impl Into<Point> for Position {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl From<Point> for Position {
    fn from(p: Point) -> Self {
        Position::new(p.x, p.y)
    }
}

impl From<&Point> for Position {
    fn from(p: &Point) -> Self {
        Position::new(p.x, p.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from(p: (i32, i32)) -> Self {
        Position::new(p.0, p.1)
    }
}

impl From<(u32, u32)> for Position {
    fn from(p: (u32, u32)) -> Self {
        Position::new(p.0 as i32, p.1 as i32)
    }
}

impl From<(i32, i32, bool)> for Position {
    fn from(data: (i32, i32, bool)) -> Self {
        let mut p = Position::new(data.0, data.1);
        p.blocks_move = data.2;
        p
    }
}

impl PartialEq<Point> for Position {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<Position> for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}
