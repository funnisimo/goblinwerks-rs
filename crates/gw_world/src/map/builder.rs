use super::Cell;
use crate::tile::{Tile, Tiles};
use crate::{map::Map, tile::TileKind};
use gw_util::rect::Rect;
use gw_util::rng::RandomNumberGenerator;
use std::cmp::{max, min};
use std::sync::Arc;

pub struct Builder<'t> {
    map: Map,
    rng: RandomNumberGenerator,
    tiles: &'t Tiles,
}

impl<'t> Builder<'t> {
    pub fn new(tiles: &'t Tiles, width: u32, height: u32) -> Builder {
        Builder {
            map: Map::new(width, height),
            rng: RandomNumberGenerator::new(),
            tiles,
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(&mut self, seed: u64) -> &mut Self {
        self.rng = RandomNumberGenerator::seeded(seed);
        self
    }

    pub fn build(self) -> Map {
        self.map
    }

    pub fn size(&self) -> (u32, u32) {
        self.map.get_size()
    }

    pub fn fill(&mut self, name: &str) -> &mut Self {
        match self.tiles.get(name) {
            None => panic!("Tile does not exist - {}", name),
            Some(tile) => self.map.fill(tile),
        }
        self
    }

    pub fn set_tile(&mut self, x: i32, y: i32, name: &str) {
        match self.tiles.get(name) {
            None => panic!("Tile does not exist - {}", name),
            Some(tile) => {
                // log(format!("set_tile({},{},{}", x, y, name));s
                self.map.reset_tiles(x, y, tile);
            }
        };
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Arc<Tile> {
        self.map.get_cell(x, y).unwrap().ground().clone()
    }

    pub fn rng_mut(&mut self) -> &mut RandomNumberGenerator {
        &mut self.rng
    }

    pub fn add_border(&mut self, name: &str) -> &mut Self {
        let (width, height) = (self.map.width, self.map.height);
        // Make the boundaries walls
        let bottom: i32 = (height - 1).try_into().unwrap();
        let right: i32 = (width - 1).try_into().unwrap();

        let wall = self.tiles.get(name).unwrap();
        let map = &mut self.map;

        for x in 0..=right {
            map.reset_tiles(x, 0, wall.clone());
            map.reset_tiles(x, bottom, wall.clone());
        }

        for y in 0..=bottom {
            map.reset_tiles(0, y, wall.clone());
            map.reset_tiles(right, y, wall.clone());
        }
        self
    }

    pub fn add_random_tiles(&mut self, name: &str, count: usize) -> &mut Self {
        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let map = &mut self.map;
        let tile = self.tiles.get(name).unwrap();

        for _i in 0..count {
            let x = self.rng.roll_dice(1, 79);
            let y = self.rng.roll_dice(1, 49);
            map.reset_tiles(x, y, tile.clone());
        }
        self
    }

    pub fn add_horizontal_hall(&mut self, x1: i32, x2: i32, y: i32, name: &str) -> &mut Self {
        let hall = self.tiles.get(name).unwrap();
        for x in min(x1, x2)..=max(x1, x2) {
            if !self.get_tile(x, y).kind.contains(TileKind::FLOOR) {
                let map = &mut self.map;
                map.reset_tiles(x, y, hall.clone());
            }
        }
        self
    }

    pub fn add_vertical_hall(&mut self, y1: i32, y2: i32, x: i32, name: &str) -> &mut Self {
        let hall = self.tiles.get(name).unwrap();
        for y in min(y1, y2)..=max(y1, y2) {
            if !self.get_tile(x, y).kind.contains(TileKind::FLOOR) {
                let map = &mut self.map;
                map.reset_tiles(x, y, hall.clone());
            }
        }
        self
    }

    pub fn room_fits(&self, room: Rect) -> bool {
        let left = room.x1 - 1;
        let right = room.x2 + 1;
        let top = room.y1 - 1;
        let bottom = room.y2 + 1;

        for y in top..=bottom {
            for x in left..=right {
                if self.get_tile(x, y).kind.contains(TileKind::FLOOR) {
                    return false;
                }
            }
        }
        true
    }

    pub fn try_add_room(&mut self, room: Rect, name: &str) -> bool {
        if !self.room_fits(room) {
            return false;
        }
        let map = &mut self.map;
        let tile = self.tiles.get(name).unwrap();
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                map.reset_tiles(x, y, tile.clone());
            }
        }

        true
    }

    pub fn add_connected_rooms(
        &mut self,
        max_count: i32,
        min_size: i32,
        max_size: i32,
        floor: &str,
        hall: &str,
    ) -> Vec<Rect> {
        let mut rooms: Vec<Rect> = vec![];

        for _ in 0..max_count {
            let w = self.rng.range(min_size, max_size);
            let h = self.rng.range(min_size, max_size);
            let x = self.rng.roll_dice(1, self.map.width as i32 - w - 1) - 1;
            let y = self.rng.roll_dice(1, self.map.height as i32 - h - 1) - 1;
            let room = Rect::with_size(x, y, w, h);

            if self.try_add_room(room, floor) {
                if !rooms.is_empty() {
                    let new_pos = room.center();
                    let prev_pos = rooms[rooms.len() - 1].center();
                    if self.rng.range(0, 2) == 1 {
                        self.add_horizontal_hall(prev_pos.x, new_pos.x, prev_pos.y, hall);
                        self.add_vertical_hall(prev_pos.y, new_pos.y, new_pos.x, hall);
                    } else {
                        self.add_vertical_hall(prev_pos.y, new_pos.y, prev_pos.x, hall);
                        self.add_horizontal_hall(prev_pos.x, new_pos.x, new_pos.y, hall);
                    }
                }

                rooms.push(room);
            }
        }

        rooms
    }
}

pub fn dig_random_level(tiles: &Tiles, width: u32, height: u32) -> Map {
    let mut builder = Builder::new(tiles, width, height);
    builder.add_border("WALL").add_random_tiles("WALL", 400);
    builder.build()
}

pub fn dig_room_level(tiles: &Tiles, width: u32, height: u32) -> Map {
    let mut builder = Builder::new(tiles, width, height);
    builder.fill("WALL");
    builder.add_connected_rooms(30, 6, 10, "FLOOR", "HALL");
    builder.build()
}
