use super::{CellFlags, MapFlags};
use super::{CellMut, CellRef};
// use crate::fov::FovSource;
use crate::tile::Tile;
use crate::tile::TileLayer;
use crate::tile::TileSet;
use crate::tile::NO_TILE;
use gw_app::ecs::Entity;
use gw_util::point::distance;
use gw_util::point::Point;
use gw_util::rng::RandomNumberGenerator;
use std::collections::HashMap;
use std::sync::Arc; // For FOV Calc

// #[derive(PartialEq, Copy, Clone)]
// pub enum TileId {
//     None,
//     Wall,
//     Floor,
//     Hall,
// }

// impl TileId {
//     pub fn blocks(&self) -> bool {
//         *self == TileId::Wall
//     }

//     pub fn blocks_vision(&self) -> bool {
//         *self == TileId::Wall
//     }
// }

pub struct Map {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    flags: MapFlags,

    any_entity_change: bool, // TODO - MapFlags
    any_tile_change: bool,   // TODO - MapFlags

    pub locations: HashMap<String, Point>,
    pub portals: HashMap<Point, (u32, String)>,

    // per cell information
    pub ground: Vec<Arc<Tile>>,
    pub feature: Vec<Arc<Tile>>,
    pub blocked: Vec<bool>, // TODO - Move to flag
    pub actors: Vec<Vec<(Entity, bool)>>,
    pub items: Vec<Vec<(Entity, bool)>>,
    cell_flags: Vec<CellFlags>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let count = (width * height) as usize;
        let fill_tile = NO_TILE.clone();

        let mut flags = vec![CellFlags::empty(); count];
        for flag in flags.iter_mut() {
            flag.insert(CellFlags::NEEDS_DRAW | CellFlags::NEEDS_SNAPSHOT);
        }

        Map {
            id: 0,
            width: width,
            height: height,
            flags: MapFlags::empty(),

            any_entity_change: true,
            any_tile_change: true,

            ground: vec![fill_tile.clone(); count],
            feature: vec![fill_tile.clone(); count],

            blocked: vec![false; count],
            actors: vec![Vec::new(); count],
            items: vec![Vec::new(); count],

            cell_flags: flags,
            // revealed_tiles: vec![false; count],
            // visible_tiles: vec![false; count],
            // changed: vec![true; count],
            // tile_changed: vec![true; count],
            locations: HashMap::new(),
            portals: HashMap::new(),
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some((y as usize * self.width as usize) + x as usize)
    }

    pub fn to_point(&self, idx: usize) -> Point {
        let w = self.width as i32;
        Point::new(idx as i32 % w, idx as i32 / w)
    }

    pub fn has_xy(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn has_idx(&self, idx: usize) -> bool {
        idx < self.ground.len()
    }

    pub fn has_point(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    pub fn get_portal(&self, x: i32, y: i32) -> Option<(u32, String)> {
        match self.portals.get(&Point::new(x, y)) {
            None => None,
            Some((id, location)) => Some((*id, location.clone())),
        }
    }

    pub fn get_location(&self, id: &str) -> Option<Point> {
        match self.locations.get(id) {
            None => None,
            Some(pt) => Some(pt.clone()),
        }
    }

    // pub fn revealed_xy(&self, x: i32, y: i32) -> bool {
    //     let idx = self.to_idx(x, y);
    //     self.revealed_idx(idx)
    // }

    // pub fn revealed_idx(&self, idx: usize) -> bool {
    //     if !self.has_idx(idx) {
    //         return false;
    //     }
    //     self.flags[idx].contains(CellFlags::REVEALED)
    // }

    // pub fn set_revealed_xy(&mut self, x: i32, y: i32) {
    //     let idx = self.to_idx(x, y);
    //     self.set_revealed_idx(idx)
    // }

    // pub fn set_revealed_idx(&mut self, idx: usize) {
    //     if !self.has_idx(idx) {
    //         return;
    //     }
    //     self.flags[idx].insert(CellFlags::REVEALED)
    // }

    pub fn reveal_all(&mut self) {
        self.flags.insert(MapFlags::ALL_REVEALED);
    }

    // pub fn visible_xy(&self, x: i32, y: i32) -> bool {
    //     let idx = self.to_idx(x, y);
    //     self.visible_idx(idx)
    // }

    // pub fn visible_idx(&self, idx: usize) -> bool {
    //     if !self.has_idx(idx) {
    //         return false;
    //     }
    //     self.flags[idx].contains(CellFlags::VISIBLE)
    // }

    // pub fn set_visible_xy(&mut self, x: i32, y: i32) {
    //     let idx = self.to_idx(x, y);
    //     self.set_visible_idx(idx)
    // }

    // pub fn set_visible_idx(&mut self, idx: usize) {
    //     if !self.has_idx(idx) {
    //         return;
    //     }
    //     self.flags[idx].insert(CellFlags::VISIBLE)
    // }

    // pub fn clear_visible_xy(&mut self, x: i32, y: i32) {
    //     let idx = self.to_idx(x, y);
    //     self.clear_visible_idx(idx)
    // }

    // pub fn clear_visible_idx(&mut self, idx: usize) {
    //     if !self.has_idx(idx) {
    //         return;
    //     }
    //     self.flags[idx].remove(CellFlags::VISIBLE)
    // }

    pub fn make_fully_visible(&mut self) {
        self.flags.insert(MapFlags::ALL_VISIBLE);
    }

    pub fn fill(&mut self, tile: Arc<Tile>) {
        self.ground.fill_with(|| tile.clone());
        for idx in 0..self.cell_flags.len() {
            self.cell_flags[idx].insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED);
        }
        self.any_tile_change = true;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<CellRef> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => Some(CellRef::new(self, idx)),
        }
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<CellMut> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => Some(CellMut::new(self, idx)),
        }
    }

    pub fn reset_tiles(&mut self, x: i32, y: i32, ground: Arc<Tile>) {
        let idx = match self.to_idx(x, y) {
            None => return,
            Some(idx) => idx,
        };
        self.ground[idx] = ground;
        self.feature[idx] = NO_TILE.clone();
        self.cell_flags[idx]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn get_tiles(&self, x: i32, y: i32) -> TileSet {
        match self.to_idx(x, y) {
            None => TileSet::new(NO_TILE.clone(), NO_TILE.clone()),
            Some(idx) => TileSet::new(self.ground[idx].clone(), self.feature[idx].clone()),
        }
    }

    pub(crate) fn get_tiles_at_idx(&self, idx: usize) -> TileSet {
        match self.ground.get(idx) {
            Some(tile) => TileSet::new(tile.clone(), self.feature[idx].clone()),
            None => TileSet::new(NO_TILE.clone(), NO_TILE.clone()),
        }
    }

    pub fn place_tile(&mut self, x: i32, y: i32, tile: Arc<Tile>) -> bool {
        match tile.layer {
            TileLayer::GROUND => self.place_ground(x, y, tile),
            TileLayer::FEATURE => self.place_feature(x, y, tile),
            _ => false,
        }
    }

    pub fn force_tile(&mut self, x: i32, y: i32, tile: Arc<Tile>) {
        match tile.layer {
            TileLayer::GROUND => self.force_ground(x, y, tile),
            TileLayer::FEATURE => self.force_feature(x, y, tile),
            _ => {}
        }
    }

    pub fn place_ground(&mut self, x: i32, y: i32, ground: Arc<Tile>) -> bool {
        // let idx = match self.to_idx(x, y) {
        //     None => return false,
        //     Some(idx) => idx,
        // };

        // TODO - Check priority vs existing tile (if any)
        // TODO - Check feature required tile (+priority)

        self.force_ground(x, y, ground);
        true
    }

    pub fn force_ground(&mut self, x: i32, y: i32, ground: Arc<Tile>) {
        let idx = match self.to_idx(x, y) {
            None => return,
            Some(idx) => idx,
        };

        self.ground[idx] = ground;
        self.cell_flags[idx]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn place_feature(&mut self, x: i32, y: i32, feature: Arc<Tile>) -> bool {
        // let idx = match self.to_idx(x, y) {
        //     None => return false,
        //     Some(idx) => idx,
        // };

        // TODO - Check priority vs existing tile (if any)
        // TODO - Check feature required tile (+priority)

        self.force_feature(x, y, feature);
        true
    }

    pub fn force_feature(&mut self, x: i32, y: i32, feature: Arc<Tile>) {
        let idx = match self.to_idx(x, y) {
            None => return,
            Some(idx) => idx,
        };

        self.feature[idx] = feature;
        self.cell_flags[idx]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn has_blocker_xy(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => return true,
            Some(idx) => idx,
        };
        self.blocked[idx]
    }

    pub fn blocked_xy(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => return true,
            Some(idx) => idx,
        };
        self.blocked_idx(idx)
    }

    fn blocked_idx(&self, idx: usize) -> bool {
        if !self.has_idx(idx) {
            return true;
        }
        if self.blocked[idx] {
            return true;
        }
        self.ground[idx].blocks()
    }

    pub fn actors_at_xy(&self, x: i32, y: i32) -> impl Iterator<Item = Entity> + '_ {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for actors at invalid xy: {},{}", x, y),
            Some(idx) => idx,
        };
        self.actors[idx].iter().map(|(a, _)| *a)
    }

    #[allow(dead_code)]
    fn actors_at_idx(&self, idx: usize) -> impl Iterator<Item = Entity> + '_ {
        if !self.has_idx(idx) {
            panic!("asked for actors at invalid index: {}", idx);
        }
        self.actors[idx].iter().map(|(a, _)| *a)
    }

    pub fn remove_actor(&mut self, idx: usize, entity: Entity) {
        if !self.has_idx(idx) {
            return;
        }

        match self.actors[idx].iter().position(|e| e.0 == entity) {
            None => {}
            Some(found_idx) => {
                self.mark_entity_changed_idx(idx);
                self.actors[idx].remove(found_idx);
                self.blocked[idx] = self.actors[idx]
                    .iter()
                    .fold(false, |out: bool, ent| out || ent.1)
                    || self.items[idx].iter().fold(false, |out, ent| out || ent.1);
            }
        }
    }

    pub fn add_actor(&mut self, idx: usize, entity: Entity, blocks: bool) {
        if !self.has_idx(idx) {
            return;
        }

        match self.actors[idx].iter().position(|e| e.0 == entity) {
            None => {
                self.actors[idx].push((entity, blocks));
                self.mark_entity_changed_idx(idx);
                if blocks {
                    self.blocked[idx] = true;
                }
            }
            Some(_) => {}
        }
    }

    pub fn items_at_xy(&self, x: i32, y: i32) -> impl Iterator<Item = Entity> + '_ {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for actors at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        self.items[idx].iter().map(|(a, _)| *a)
    }

    pub fn items_at_idx(&self, idx: usize) -> impl Iterator<Item = Entity> + '_ {
        if !self.has_idx(idx) {
            panic!("asked for items at invalid index: {}", idx);
        }
        self.items[idx].iter().map(|(a, _)| *a)
    }

    pub fn remove_item_at_xy(&mut self, x: i32, y: i32, entity: Entity) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for item at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        self.remove_item(idx, entity)
    }

    pub fn remove_item(&mut self, idx: usize, entity: Entity) {
        if !self.has_idx(idx) {
            return;
        }

        match self.items[idx].iter().position(|e| e.0 == entity) {
            None => {}
            Some(found_idx) => {
                self.mark_entity_changed_idx(idx);
                self.items[idx].remove(found_idx);
                self.blocked[idx] = self.actors[idx]
                    .iter()
                    .fold(false, |out: bool, ent| out || ent.1)
                    || self.items[idx].iter().fold(false, |out, ent| out || ent.1);
            }
        }
    }

    pub fn add_item_at_xy(&mut self, x: i32, y: i32, entity: Entity, blocks: bool) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for item at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.add_item(idx, entity, blocks)
    }

    pub fn add_item(&mut self, idx: usize, entity: Entity, blocks: bool) {
        if !self.has_idx(idx) {
            panic!("Invalid map index for add_item: {}", idx);
        }

        match self.items[idx].iter().position(|e| e.0 == entity) {
            None => {
                self.items[idx].push((entity, blocks));
                self.mark_entity_changed_idx(idx);
                if blocks {
                    self.blocked[idx] = true;
                }
            }
            Some(_) => {}
        }
    }

    pub fn opaque_xy(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.opaque_idx(idx)
    }

    fn opaque_idx(&self, idx: usize) -> bool {
        if !self.has_idx(idx) {
            return false;
        }
        let tile = &self.ground[idx];
        tile.blocks_vision()
    }

    pub fn needs_draw_xy(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.needs_draw_idx(idx)
    }

    pub fn needs_draw_idx(&self, idx: usize) -> bool {
        if !self.has_idx(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::NEEDS_DRAW)
    }

    pub fn clear_needs_draw_idx(&mut self, idx: usize) {
        if !self.has_idx(idx) {
            return;
        }
        self.cell_flags[idx].remove(CellFlags::NEEDS_DRAW);
    }

    pub fn set_needs_draw_xy(&mut self, x: i32, y: i32) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.set_needs_draw_idx(idx)
    }

    pub fn set_needs_draw_idx(&mut self, idx: usize) {
        if self.has_idx(idx) {
            self.cell_flags[idx].insert(CellFlags::NEEDS_DRAW);
        }
    }

    pub fn needs_snapshot_xy(&self, x: i32, y: i32) -> bool {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.needs_snapshot_idx(idx)
    }

    pub fn needs_snapshot_idx(&self, idx: usize) -> bool {
        if !self.has_idx(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::NEEDS_SNAPSHOT)
    }

    pub fn set_needs_snapshot_xy(&mut self, x: i32, y: i32) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for actors at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        self.set_needs_snapshot_idx(idx);
    }

    pub fn set_needs_snapshot_idx(&mut self, idx: usize) {
        if self.has_idx(idx) {
            self.cell_flags[idx].insert(CellFlags::NEEDS_SNAPSHOT);
        }
    }

    pub fn clear_needs_snapshot_idx(&mut self, idx: usize) {
        if !self.has_idx(idx) {
            return;
        }
        self.cell_flags[idx].remove(CellFlags::NEEDS_SNAPSHOT);
    }

    pub fn set_entity_changed_xy(&mut self, x: i32, y: i32) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };

        self.set_entity_changed_idx(idx);
    }

    pub fn set_entity_changed_idx(&mut self, idx: usize) {
        if !self.has_idx(idx) {
            return;
        }
        self.mark_entity_changed_idx(idx);
        // println!(" - changed: {}", idx);
    }

    fn mark_entity_changed_idx(&mut self, idx: usize) {
        self.cell_flags[idx].insert(CellFlags::ENTITY_CHANGED | CellFlags::NEEDS_DRAW);
        self.any_entity_change = true;
    }

    pub fn any_changes(&self) -> bool {
        self.any_entity_change || self.any_tile_change
    }

    pub fn any_tile_changed(&self) -> bool {
        self.any_tile_change
    }

    pub fn any_entity_changed(&self) -> bool {
        self.any_entity_change
    }

    pub fn tile_changed_idx(&self, idx: usize) -> bool {
        if !self.has_idx(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::TILE_CHANGED)
    }

    pub fn clear_tile_changed_idx(&mut self, idx: usize) {
        if !self.has_idx(idx) {
            return;
        }
        self.cell_flags[idx].remove(CellFlags::TILE_CHANGED)
    }

    pub fn clear_change_flags(&mut self) {
        self.any_tile_change = false;
        self.any_entity_change = false;
    }

    pub fn clear_all_changed_flags(&mut self) {
        for idx in 0..self.cell_flags.len() {
            self.cell_flags[idx].remove(CellFlags::ENTITY_CHANGED | CellFlags::TILE_CHANGED);
        }
        self.clear_change_flags();
    }

    pub fn clear_tile_content(&mut self) {
        for content in self.items.iter_mut() {
            content.clear();
        }

        for content in self.actors.iter_mut() {
            content.clear();
        }
    }

    pub fn set_flag_xy(&mut self, x: i32, y: i32, flag: CellFlags) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        self.cell_flags[idx].insert(flag);
    }

    pub fn has_flag_xy(&self, x: i32, y: i32, flag: CellFlags) -> bool {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        self.cell_flags[idx].intersects(flag)
    }

    pub fn set_flag(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            cell.insert(flag);
        }
    }

    pub fn clear_flag(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            cell.remove(flag);
        }
    }

    pub fn clear_flag_xy(&mut self, x: i32, y: i32, flag: CellFlags) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        let flags = &mut self.cell_flags[idx];
        if flags.intersects(flag) {
            flags.remove(flag);
        }
    }

    pub fn clear_flag_with_redraw(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            if cell.intersects(flag) {
                cell.remove(flag);
                cell.insert(CellFlags::NEEDS_DRAW);
            }
        }
    }

    pub fn clear_flag_with_redraw_xy(&mut self, x: i32, y: i32, flag: CellFlags) {
        let idx = match self.to_idx(x, y) {
            None => panic!("asked for tile at invalid x,y {},{}", x, y),
            Some(idx) => idx,
        };
        let flags = &mut self.cell_flags[idx];
        if flags.intersects(flag) {
            flags.remove(flag);
            flags.insert(CellFlags::NEEDS_DRAW);
        }
    }
}

// impl BaseMap for Map {
//     fn is_opaque(&self, idx: usize) -> bool {
//         self.opaque_idx(idx)
//     }
// }

// impl PathfindingSource for Map {
//     // Handled in default for Trait
//     // fn estimate_pathing_distance(&self, a: Point, b: Point) -> f32 {
//     //     DistanceAlg::Pythagoras.distance2d(a, b)
//     // }

//     fn move_cost(&self, x: i32, y: i32) -> Option<f32> {
//         if !self.has_xy(x, y) {
//             return None;
//         }
//         let idx = self.to_idx(x, y);
//         let tile_idx = self.tiles[idx];

//         match get_tile(tile_idx) {
//             Some(tile) => {
//                 if tile.blocks() {
//                     return None;
//                 }
//             }
//             None => return None,
//         }

//         if self.blocked[idx] {
//             // TODO - Allies?  Enemies?  Fixture?
//             return Some(5.0);
//         }

//         Some(1.0)
//     }

//     fn get_size(&self) -> (usize, usize) {
//         (self.width, self.height)
//     }
// }

// impl Algorithm2D for Map {
//     fn dimensions(&self) -> Point {
//         Point::new(self.width, self.height)
//     }
// }

pub fn find_random_point<F>(map: &Map, rng: &mut RandomNumberGenerator, func: F) -> Option<Point>
where
    F: Fn(i32, i32, TileSet) -> bool,
{
    for _ in 0..200 {
        let x = rng.range(0i32, map.width as i32 - 1);
        let y = rng.range(0i32, map.height as i32 - 1);
        let tiles = map.get_tiles(x, y);
        if func(x, y, tiles) {
            return Some(Point::new(x, y));
        }
    }
    None
}

pub fn closest_points_matching<F>(map: &Map, x: i32, y: i32, func: F) -> Vec<Point>
where
    F: Fn(i32, i32, TileSet) -> bool,
{
    let mut points: Vec<Point> = Vec::new();

    let start = Point::new(x, y);
    let mut current = Point::new(x, y);

    for dist in 0..200 {
        for x1 in (x - dist)..(x + dist + 1) {
            for y1 in (y - dist)..(y + dist + 1) {
                if !map.has_xy(x1, y1) {
                    continue;
                }
                current.x = x1;
                current.y = y1;
                if distance::manhattan(&start, &current).round() as i32 != dist {
                    continue;
                }

                let tiles = map.get_tiles(x1, y1);
                if func(x1, y1, tiles) {
                    points.push(Point::new(x1, y1));
                }
            }
        }
        if points.len() > 0 {
            return points;
        }
    }
    points
}

pub fn dump_map(map: &Map) {
    let mut header = "   |".to_string();
    for x in 0..map.width {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);

    for y in 0..map.height as i32 {
        let mut line = format!("{:2} |", y);
        for x in 0..map.width as i32 {
            let tiles = map.get_tiles(x, y);
            let sprite = tiles.sprite();
            let ch = match sprite.glyph {
                0 => ' ',
                x => char::from_u32(x).unwrap(),
            };
            line.push(ch);
        }
        line.push_str(&format!("| {:2}", y));
        println!("{}", line);
    }

    let mut header = "   |".to_string();
    for x in 0..map.width {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);
}

pub fn dump_map_with<F>(map: &Map, func: F)
where
    F: Fn(&Map, i32, i32) -> char,
{
    let mut header = "   |".to_string();
    for x in 0..map.width {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);

    for y in 0..map.height as i32 {
        let mut line = format!("{:2} |", y);
        for x in 0..map.width as i32 {
            let tile = func(map, x, y);
            line.push(tile);
        }
        line.push_str(&format!("| {:2}", y));
        println!("{}", line);
    }

    let mut header = "   |".to_string();
    for x in 0..map.width {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);
}
