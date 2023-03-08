use super::goblin::calculate_fov;
use super::FovTarget;
use crate::map::Map;
use crate::position::Position;
use gw_app::ecs::world::EntityStore;
use gw_app::ecs::Entity;
use gw_app::log;
use gw_app::Ecs;
use gw_util::point::Point;
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<Vec<Vec<bool>>> = Mutex::new(Vec::new());
}

pub struct FOVMask {
    width: i32,
    height: i32,
    data: Option<Vec<bool>>,
    // active: bool,
}

impl FOVMask {
    // pub fn alloc(width: u32, height: u32) -> Self {
    //     let data = match CACHE.lock().unwrap().pop() {
    //         Some(data) => data,
    //         None => Vec::new(),
    //     };
    //     FOVMask::new(width, height, data)
    // }

    // pub fn free(mut map: FOVMask) {
    //     let mut cache = CACHE.lock().unwrap();
    //     map.active = false;
    //     // println!(" - save: {}x{}", map.width, map.height);
    //     cache.push(map.data.take().unwrap());
    // }

    // fn new(width: u32, height: u32, data: Vec<bool>) -> Self {
    //     let mut c = FOVMask {
    //         width,
    //         height,
    //         data: Some(data),
    //         active: true,
    //     };
    //     c.reset(width, height);
    //     c
    // }

    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let mut data = match CACHE.lock().unwrap().pop() {
            Some(mut data) => {
                log(format!("- reuse FOVMask : {}/{}", data.len(), size));
                data.resize(size, false);
                data
            }
            None => vec![false; size],
        };
        data.fill(false);

        FOVMask {
            width,
            height,
            data: Some(data),
        }
    }

    // fn reset(&mut self, width: u32, height: u32) {
    //     if let Some(data) = self.data.as_mut() {
    //         data.resize((width * height) as usize, false);
    //         data.fill(false);
    //     }
    //     self.width = width;
    //     self.height = height;
    //     self.active = true;
    // }

    fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }
        Some((x + y * self.width as i32) as usize)
    }

    pub fn in_fov(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => return false,
            Some(idx) => idx,
        };
        self.data.as_ref().unwrap()[idx]
    }

    pub fn set_in_fov(&mut self, x: i32, y: i32) {
        match self.to_idx(x, y) {
            None => {}
            Some(idx) => self.data.as_mut().unwrap()[idx] = true,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        FovIter::new(self)
    }
}

impl FovTarget for FOVMask {
    fn reset(&mut self, width: u32, height: u32) {
        let size = (width * height) as usize;
        if let Some(data) = self.data.as_mut() {
            data.resize(size, false);
        }
    }

    fn set_visible(&mut self, x: i32, y: i32, pct: f32) {
        if pct > 0.0 {
            self.set_in_fov(x, y);
        }
    }
}

impl Debug for FOVMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            let mut line = "".to_owned();
            for x in 0..self.width as i32 {
                line += match self.in_fov(x, y) {
                    true => ".",
                    false => " ",
                };
            }
            write!(f, "|{}|\n", line)?
        }
        Ok(())
    }
}

impl Drop for FOVMask {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            CACHE.lock().unwrap().push(data);
        }
    }
}

pub struct FovIter<'a> {
    mask: &'a FOVMask,
    idx: usize,
}

impl<'a> FovIter<'a> {
    pub fn new(mask: &'a FOVMask) -> Self {
        FovIter { mask, idx: 0 }
    }
}

impl<'a> Iterator for FovIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mask = self.mask;
        loop {
            if self.idx >= self.mask.data.as_ref().unwrap().len() {
                return None;
            }
            let idx = self.idx;
            self.idx += 1;

            let val = self.mask.data.as_ref().unwrap()[idx];
            if val {
                let x = idx as i32 % mask.width;
                let y = idx as i32 / mask.width;
                return Some(Point::new(x, y));
            }
        }
    }
}

pub fn get_fov_mask(ecs: &Ecs, entity: Entity, radius: u32) -> FOVMask {
    let map = ecs.resources.get::<Map>().unwrap();
    let size = map.get_size();
    let mut mask = FOVMask::new(size.0 as i32, size.1 as i32);

    let origin: Point = match ecs.world.entry_ref(entity) {
        Err(_) => return FOVMask::new(size.0 as i32, size.1 as i32),
        Ok(obj) => obj.get_component::<Position>().unwrap().point(),
    };
    calculate_fov(&*map, origin, radius, &mut mask);
    mask
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tile::Tiles;
    use gw_app::ecs::Ecs;

    #[test]
    fn simple() {
        let mut ecs = Ecs::new();

        let mut map = Map::new(20, 20);
        let tiles = Tiles::default();
        map.fill(tiles.get("FLOOR").unwrap());

        ecs.resources.insert(map);

        let entity = ecs.world.push((Position::new(10, 10),));

        let mask = get_fov_mask(&ecs, entity, 7);

        println!("{:?}", mask);
        assert_eq!(mask.in_fov(0, 0), false);
        assert_eq!(mask.in_fov(10, 10), true);
    }

    #[test]
    fn corner() {
        let mut ecs = Ecs::new();

        let mut map = Map::new(20, 20);
        let tiles = Tiles::default();
        map.fill(tiles.get("FLOOR").unwrap());

        ecs.resources.insert(map);

        let entity = ecs.world.push((Position::new(3, 3),));

        let mask = get_fov_mask(&ecs, entity, 7);

        println!("{:?}", mask);
        assert_eq!(mask.in_fov(0, 0), true);
        assert_eq!(mask.in_fov(10, 10), false);
    }

    #[test]
    fn with_blockers() {
        let mut ecs = Ecs::new();

        let mut map = Map::new(20, 20);
        let tiles = Tiles::default();
        map.fill(tiles.get("FLOOR").unwrap());
        map.reset_tiles(10, 9, tiles.get("WALL").unwrap());
        map.reset_tiles(7, 10, tiles.get("WALL").unwrap());
        map.reset_tiles(14, 9, tiles.get("WALL").unwrap());
        map.reset_tiles(10, 11, tiles.get("WALL").unwrap());
        ecs.resources.insert(map);

        let entity = ecs.world.push((Position::new(10, 10),));

        let mask = get_fov_mask(&ecs, entity, 9);

        println!("{:?}", mask);
        assert_eq!(mask.in_fov(0, 0), false);
        assert_eq!(mask.in_fov(10, 10), true);
        assert_eq!(mask.in_fov(7, 10), true);
        assert_eq!(mask.in_fov(6, 10), false);
    }

    #[test]
    fn iter() {
        let mut ecs = Ecs::new();

        let mut map = Map::new(20, 20);
        let tiles = Tiles::default();
        map.fill(tiles.get("FLOOR").unwrap());
        map.reset_tiles(10, 9, tiles.get("WALL").unwrap());
        map.reset_tiles(7, 10, tiles.get("WALL").unwrap());
        map.reset_tiles(14, 9, tiles.get("WALL").unwrap());
        map.reset_tiles(10, 11, tiles.get("WALL").unwrap());
        ecs.resources.insert(map);

        let entity = ecs.world.push((Position::new(10, 10),));

        let mask = get_fov_mask(&ecs, entity, 9);

        println!("{:?}", mask);
        let mut count = 0;
        for point in mask.iter() {
            assert!(mask.to_idx(point.x, point.y).is_some());
            assert!(mask.in_fov(point.x, point.y));
            count += 1;
        }
        assert_eq!(count, 127);
    }
}
