use super::Cell;
use crate::effect::BoxedEffect;
use crate::tile::{Tile, Tiles};
use crate::{map::Map, tile::TileKind};
use gw_util::point::Point;
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
        self.map.size()
    }

    pub fn fill(&mut self, name: &str) -> &mut Self {
        match self.tiles.get(name) {
            None => panic!("Tile does not exist - {}", name),
            Some(tile) => self.map.fill(tile),
        }
        self
    }

    pub fn set_tile(&mut self, x: i32, y: i32, name: &str) {
        let idx = match self.map.get_wrapped_index(x, y) {
            None => return,
            Some(idx) => idx,
        };
        match self.tiles.get(name) {
            None => panic!("Tile does not exist - {}", name),
            Some(tile) => {
                // log(format!("set_tile({},{},{}", x, y, name));s
                self.map.reset_tiles(idx, tile);
            }
        };
    }

    pub fn place_tile(&mut self, x: i32, y: i32, name: &str) {
        let idx = match self.map.get_wrapped_index(x, y) {
            None => return,
            Some(idx) => idx,
        };
        match self.tiles.get(name) {
            None => panic!("Tile does not exist - {}", name),
            Some(tile) => {
                // log(format!("set_tile({},{},{}", x, y, name));s
                self.map.place_tile(idx, tile);
            }
        };
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Arc<Tile> {
        let idx = match self.map.get_wrapped_index(x, y) {
            None => panic!("Unknown x, y = {},{}", x, y),
            Some(idx) => idx,
        };
        self.map.get_cell(idx).unwrap().ground().clone()
    }

    pub fn rng_mut(&mut self) -> &mut RandomNumberGenerator {
        &mut self.rng
    }

    // pub fn set_portal(&mut self, point: Point, map_id: &str, location: &str) -> &mut Self {
    //     let idx = match self.map.get_wrapped_index(point.x, point.y) {
    //         None => return self,
    //         Some(idx) => idx,
    //     };

    //     // TODO - Set portal info on map cell
    //     // self.map.set_portal(idx, info);
    //     self
    // }

    pub fn set_location(&mut self, location: &str, point: Point) -> &mut Self {
        let idx = match self.map.get_wrapped_index(point.x, point.y) {
            None => return self,
            Some(idx) => idx,
        };
        self.map.set_location(location, idx);
        self
    }

    pub fn add_effect(&mut self, x: i32, y: i32, action: &str, effect: BoxedEffect) -> &mut Self {
        let idx = match self.map.get_wrapped_index(x, y) {
            None => return self,
            Some(idx) => idx,
        };

        self.map.add_effect(idx, action, effect);
        self
    }

    pub fn set_flavor(&mut self, x: i32, y: i32, text: &str) -> &mut Self {
        let idx = match self.map.get_wrapped_index(x, y) {
            None => return self,
            Some(idx) => idx,
        };

        self.map.set_flavor(idx, text.to_string());
        self
    }

    pub fn add_border(&mut self, name: &str) -> &mut Self {
        let (width, height) = (self.map.width, self.map.height);
        // Make the boundaries walls
        let bottom: i32 = (height - 1).try_into().unwrap();
        let right: i32 = (width - 1).try_into().unwrap();

        let wall = self.tiles.get(name).unwrap();
        let map = &mut self.map;

        for x in 0..=right {
            let idx = map.get_wrapped_index(x, 0).unwrap();
            map.reset_tiles(idx, wall.clone());
            let idx = map.get_wrapped_index(x, bottom).unwrap();
            map.reset_tiles(idx, wall.clone());
        }

        for y in 0..=bottom {
            let idx = map.get_wrapped_index(0, y).unwrap();
            map.reset_tiles(idx, wall.clone());
            let idx = map.get_wrapped_index(right, y).unwrap();
            map.reset_tiles(idx, wall.clone());
        }
        self
    }

    pub fn add_random_tiles(&mut self, name: &str, count: usize) -> &mut Self {
        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let map = &mut self.map;
        let tile = self.tiles.get(name).unwrap();

        for _i in 0..count {
            let x = self.rng.roll_dice(1, 79) as i32;
            let y = self.rng.roll_dice(1, 49) as i32;
            let index = map.get_wrapped_index(x, y).unwrap();
            map.reset_tiles(index, tile.clone());
        }
        self
    }

    pub fn add_horizontal_hall(&mut self, x1: i32, x2: i32, y: i32, name: &str) -> &mut Self {
        let hall = self.tiles.get(name).unwrap();
        for x in min(x1, x2)..=max(x1, x2) {
            if !self.get_tile(x, y).kind.contains(TileKind::FLOOR) {
                let map = &mut self.map;
                let index = map.get_wrapped_index(x, y).unwrap();
                map.reset_tiles(index, hall.clone());
            }
        }
        self
    }

    pub fn add_vertical_hall(&mut self, y1: i32, y2: i32, x: i32, name: &str) -> &mut Self {
        let hall = self.tiles.get(name).unwrap();
        for y in min(y1, y2)..=max(y1, y2) {
            if !self.get_tile(x, y).kind.contains(TileKind::FLOOR) {
                let map = &mut self.map;
                let index = map.get_wrapped_index(x, y).unwrap();
                map.reset_tiles(index, hall.clone());
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
                let index = map.get_wrapped_index(x, y).unwrap();
                map.reset_tiles(index, tile.clone());
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
            let x = self
                .rng
                .roll_dice(1, self.map.width.saturating_sub(w as u32).saturating_sub(1))
                as i32
                - 1;
            let y = self.rng.roll_dice(
                1,
                self.map.height.saturating_sub(h as u32).saturating_sub(1),
            ) as i32
                - 1;
            let room = Rect::with_size(x, y, w as u32, h as u32);

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
