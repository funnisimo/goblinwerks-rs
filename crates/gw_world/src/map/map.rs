use super::{Cell, CellMut, CellRef};
use super::{CellFlags, MapFlags};
use crate::effect::BoxedEffect;
use crate::tile::Tile;
use crate::tile::TileLayer;
use crate::tile::NO_TILE;
use gw_app::ecs::Entity;
use gw_app::log;
use gw_util::point::distance;
use gw_util::point::Point;
use gw_util::rect::Rect;
use gw_util::rng::RandomNumberGenerator;
use gw_util::xy::Lock;
use gw_util::xy::Wrap;
use std::collections::HashMap;
use std::sync::Arc; // For FOV Calc

pub struct Map {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub wrap: Wrap,
    pub lock: Lock,
    flags: MapFlags,
    pub welcome: Option<String>,
    region: Rect,

    any_entity_change: bool, // TODO - MapFlags
    any_tile_change: bool,   // TODO - MapFlags

    pub locations: HashMap<String, usize>,
    pub cell_effects: HashMap<usize, HashMap<String, Vec<BoxedEffect>>>,
    pub flavors: HashMap<usize, String>,

    // per cell information
    pub ground: Vec<Arc<Tile>>,
    pub fixture: Vec<Arc<Tile>>,
    pub blocked: Vec<bool>, // TODO - Move to flag
    pub beings: Vec<Vec<(Entity, bool)>>,
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
            wrap: Wrap::None,
            lock: Lock::None,
            flags: MapFlags::empty(),
            welcome: None,
            region: Rect::with_size(0, 0, width, height),

            any_entity_change: true,
            any_tile_change: true,

            ground: vec![fill_tile.clone(); count],
            fixture: vec![fill_tile.clone(); count],
            cell_effects: HashMap::new(),
            flavors: HashMap::new(),

            blocked: vec![false; count],
            beings: vec![Vec::new(); count],
            items: vec![Vec::new(); count],

            cell_flags: flags,
            // revealed_tiles: vec![false; count],
            // visible_tiles: vec![false; count],
            // changed: vec![true; count],
            // tile_changed: vec![true; count],
            locations: HashMap::new(),
        }
    }

    pub fn region(&self) -> &Rect {
        &self.region
    }

    pub fn select_region(&mut self, left: i32, top: i32, width: u32, height: u32) {
        self.region = Rect::with_size(left, top, width, height);
    }

    pub fn set_region_pos(&mut self, left: i32, top: i32) {
        let cur = &self.region;
        self.region = Rect::with_size(left, top, cur.width(), cur.height());
    }

    pub fn move_region_pos(&mut self, dx: i32, dy: i32) {
        let cur = &self.region;
        self.region = Rect::with_bounds(cur.x1 + dx, cur.y1 + dy, cur.x2 + dx, cur.y2 + dy);
    }

    pub fn clear_region(&mut self) {
        self.region = Rect::with_size(0, 0, self.width, self.height);
    }

    pub fn size(&self) -> (u32, u32) {
        self.region.size()
    }

    pub fn full_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some((y * self.width as i32 + x) as usize)
    }

    pub fn get_wrapped_index(&self, x: i32, y: i32) -> Option<usize> {
        match self.try_wrap_xy(x, y) {
            None => None,
            Some((x, y)) => Some((x + y * self.width as i32) as usize),
        }
    }

    pub fn to_xy(&self, idx: usize) -> (i32, i32) {
        let w = self.width as i32;
        (idx as i32 % w, idx as i32 / w)
    }

    pub fn to_point(&self, idx: usize) -> Point {
        let w = self.width as i32;
        Point::new(idx as i32 % w, idx as i32 / w)
    }

    pub fn try_wrap_xy(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.wrap.try_wrap(x, y, &self.region)
    }

    // pub fn has_xy(&self, x: i32, y: i32) -> bool {
    //     self.get_index(x, y).is_some()
    // }

    fn has_index(&self, idx: usize) -> bool {
        idx < self.ground.len()
    }

    // fn has_point(&self, point: &Point) -> bool {
    //     self.has_xy(point.x, point.y)
    // }

    ///// Everything from here on should use index...

    pub fn get_location(&self, id: &str) -> Option<usize> {
        self.locations.get(id).map(|v| *v)
    }

    pub fn set_location(&mut self, id: &str, index: usize) {
        self.locations.insert(id.to_string(), index);
    }

    pub fn reveal_all(&mut self) {
        self.flags.insert(MapFlags::ALL_REVEALED);
    }

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

    pub fn get_cell(&self, index: usize) -> Option<CellRef> {
        match self.has_index(index) {
            false => {
                log(format!("Cell does not have index = {}", index));
                None
            }
            true => Some(CellRef::new(self, index)),
        }
    }

    pub fn get_cell_mut(&mut self, idx: usize) -> Option<CellMut> {
        match self.has_index(idx) {
            false => None,
            true => Some(CellMut::new(self, idx)),
        }
    }

    pub fn reset_tiles(&mut self, idx: usize, ground: Arc<Tile>) {
        if !self.has_index(idx) {
            return;
        }
        self.ground[idx] = ground;
        self.fixture[idx] = NO_TILE.clone();

        self.cell_flags[idx]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn place_tile(&mut self, index: usize, tile: Arc<Tile>) -> bool {
        match tile.layer {
            TileLayer::GROUND => self.place_ground(index, tile),
            TileLayer::FIXTURE => self.place_fixture(index, tile),
            _ => false,
        }
    }

    pub fn force_tile(&mut self, index: usize, tile: Arc<Tile>) {
        match tile.layer {
            TileLayer::GROUND => self.force_ground(index, tile),
            TileLayer::FIXTURE => self.force_fixture(index, tile),
            _ => {}
        }
    }

    pub fn place_ground(&mut self, index: usize, ground: Arc<Tile>) -> bool {
        // TODO - Check priority vs existing tile (if any)
        // TODO - Check feature required tile (+priority)

        self.force_ground(index, ground);
        true
    }

    pub fn force_ground(&mut self, index: usize, ground: Arc<Tile>) {
        if !self.has_index(index) {
            return;
        }

        self.ground[index] = ground;
        self.cell_flags[index]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn place_fixture(&mut self, index: usize, fixture: Arc<Tile>) -> bool {
        // TODO - Check priority vs existing tile (if any)
        // TODO - Check fixture required tile (+priority)

        self.force_fixture(index, fixture);
        true
    }

    pub fn force_fixture(&mut self, index: usize, fixture: Arc<Tile>) {
        if !self.has_index(index) {
            return;
        }

        self.fixture[index] = fixture;
        self.cell_flags[index]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn clear_fixture(&mut self, index: usize) {
        if !self.has_index(index) {
            return;
        }

        self.fixture[index] = NO_TILE.clone();
        self.cell_flags[index]
            .insert(CellFlags::NEEDS_DRAW | CellFlags::TILE_CHANGED | CellFlags::NEEDS_SNAPSHOT);
        self.any_tile_change = true;
    }

    pub fn add_effect(&mut self, index: usize, action: &str, effect: BoxedEffect) {
        match self.cell_effects.get_mut(&index) {
            None => {
                let mut map = HashMap::new();
                map.insert(action.to_string(), vec![effect]);
                self.cell_effects.insert(index, map);
            }
            Some(eff) => match eff.get_mut(action) {
                None => {
                    eff.insert(action.to_string(), vec![effect]);
                }
                Some(data) => {
                    data.push(effect);
                }
            },
        }
    }

    pub fn set_effects(&mut self, index: usize, action: &str, effects: Vec<BoxedEffect>) {
        match self.cell_effects.get_mut(&index) {
            None => {
                let mut map = HashMap::new();
                map.insert(action.to_string(), effects);
                self.cell_effects.insert(index, map);
            }
            Some(eff) => {
                eff.insert(action.to_string(), effects);
            }
        }
    }

    pub fn get_cell_effects(&self, index: usize, action: &str) -> Option<Vec<BoxedEffect>> {
        match self.cell_effects.get(&index) {
            None => None,
            Some(all) => match all.get(action) {
                None => None,
                Some(data) => Some(data.clone()),
            },
        }
    }

    pub fn set_flavor(&mut self, index: usize, text: String) {
        self.flavors.insert(index, text);
    }

    pub fn has_blocker(&self, index: usize) -> bool {
        match self.blocked.get(index) {
            None => true,
            Some(val) => *val,
        }
    }

    pub fn is_blocked(&self, idx: usize) -> bool {
        if !self.has_index(idx) {
            return true;
        }
        if self.blocked[idx] {
            return true;
        }
        self.ground[idx].blocks()
    }

    pub fn iter_beings(&self, idx: usize) -> impl Iterator<Item = Entity> + '_ {
        if !self.has_index(idx) {
            panic!("asked for actors at invalid index: {}", idx);
        }
        self.beings[idx].iter().map(|(a, _)| *a)
    }

    pub fn remove_being(&mut self, idx: usize, entity: Entity) {
        if !self.has_index(idx) {
            return;
        }

        self.mark_entity_changed(idx);
        match self.beings[idx].iter().position(|e| e.0 == entity) {
            None => {}
            Some(found_idx) => {
                self.beings[idx].remove(found_idx);
                self.blocked[idx] = self.beings[idx]
                    .iter()
                    .fold(false, |out: bool, ent| out || ent.1)
                    || self.items[idx].iter().fold(false, |out, ent| out || ent.1);
            }
        }
    }

    pub fn add_being(&mut self, idx: usize, entity: Entity, blocks: bool) {
        if !self.has_index(idx) {
            return;
        }

        match self.beings[idx].iter().position(|e| e.0 == entity) {
            None => {
                self.beings[idx].push((entity, blocks));
                self.mark_entity_changed(idx);
                if blocks {
                    self.blocked[idx] = true;
                }
            }
            Some(_) => {}
        }
    }

    pub fn iter_items(&self, idx: usize) -> impl Iterator<Item = Entity> + '_ {
        if !self.has_index(idx) {
            panic!("asked for items at invalid index: {}", idx);
        }
        self.items[idx].iter().map(|(a, _)| *a)
    }

    pub fn remove_item(&mut self, idx: usize, entity: Entity) {
        if !self.has_index(idx) {
            return;
        }

        match self.items[idx].iter().position(|e| e.0 == entity) {
            None => {}
            Some(found_idx) => {
                self.mark_entity_changed(idx);
                self.items[idx].remove(found_idx);
                self.blocked[idx] = self.beings[idx]
                    .iter()
                    .fold(false, |out: bool, ent| out || ent.1)
                    || self.items[idx].iter().fold(false, |out, ent| out || ent.1);
            }
        }
    }

    pub fn add_item(&mut self, idx: usize, entity: Entity, blocks: bool) {
        if !self.has_index(idx) {
            panic!("Invalid map index for add_item: {}", idx);
        }

        match self.items[idx].iter().position(|e| e.0 == entity) {
            None => {
                self.items[idx].push((entity, blocks));
                self.mark_entity_changed(idx);
                if blocks {
                    self.blocked[idx] = true;
                }
            }
            Some(_) => {}
        }
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        if !self.has_index(idx) {
            return false;
        }
        let cell = self.get_cell(idx).unwrap();
        cell.is_opaque()
    }

    pub fn needs_draw(&self, idx: usize) -> bool {
        if !self.has_index(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::NEEDS_DRAW)
    }

    pub fn clear_needs_draw(&mut self, idx: usize) {
        if !self.has_index(idx) {
            return;
        }
        self.cell_flags[idx].remove(CellFlags::NEEDS_DRAW);
    }

    pub fn set_needs_draw(&mut self, idx: usize) {
        if self.has_index(idx) {
            self.cell_flags[idx].insert(CellFlags::NEEDS_DRAW);
        }
    }

    pub fn needs_snapshot(&self, idx: usize) -> bool {
        if !self.has_index(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::NEEDS_SNAPSHOT)
    }

    pub fn set_needs_snapshot(&mut self, idx: usize) {
        if self.has_index(idx) {
            self.cell_flags[idx].insert(CellFlags::NEEDS_SNAPSHOT);
        }
    }

    pub fn clear_needs_snapshot(&mut self, idx: usize) {
        if !self.has_index(idx) {
            return;
        }
        self.cell_flags[idx].remove(CellFlags::NEEDS_SNAPSHOT);
    }

    pub fn set_entity_changed(&mut self, idx: usize) {
        if !self.has_index(idx) {
            return;
        }
        self.mark_entity_changed(idx);
        // println!(" - changed: {}", idx);
    }

    pub fn mark_entity_changed(&mut self, idx: usize) {
        if !self.has_index(idx) {
            return;
        }
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

    pub fn has_tile_changed(&self, idx: usize) -> bool {
        if !self.has_index(idx) {
            return true;
        }
        self.cell_flags[idx].contains(CellFlags::TILE_CHANGED)
    }

    pub fn clear_tile_changed(&mut self, idx: usize) {
        if !self.has_index(idx) {
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

        for content in self.beings.iter_mut() {
            content.clear();
        }
    }

    pub fn set_flag(&mut self, index: usize, flag: CellFlags) {
        if !self.has_index(index) {
            return;
        }
        self.cell_flags[index].insert(flag);
    }

    pub fn has_flag(&self, idx: usize, flag: CellFlags) -> bool {
        if !self.has_index(idx) {
            return false;
        }
        self.cell_flags[idx].contains(flag)
    }

    pub fn has_any_flag_idx(&self, idx: usize, flag: CellFlags) -> bool {
        if !self.has_index(idx) {
            return false;
        }
        self.cell_flags[idx].intersects(flag)
    }

    pub fn set_flag_everywhere(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            cell.insert(flag);
        }
    }

    pub fn clear_flag_everywhere(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            cell.remove(flag);
        }
    }

    pub fn clear_flag(&mut self, index: usize, flag: CellFlags) {
        if !self.has_index(index) {
            return;
        }
        let flags = &mut self.cell_flags[index];
        if flags.intersects(flag) {
            flags.remove(flag);
        }
    }

    pub fn clear_flag_everywhere_with_redraw(&mut self, flag: CellFlags) {
        for cell in self.cell_flags.iter_mut() {
            if cell.intersects(flag) {
                cell.remove(flag);
                cell.insert(CellFlags::NEEDS_DRAW);
            }
        }
    }

    pub fn clear_flag_with_redraw(&mut self, index: usize, flag: CellFlags) {
        if !self.has_index(index) {
            return;
        }
        let flags = &mut self.cell_flags[index];
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

pub fn find_random_point<F>(map: &Map, rng: &mut RandomNumberGenerator, func: F) -> Option<Point>
where
    F: Fn(i32, i32, CellRef) -> bool,
{
    let region = map.region();

    for _ in 0..200 {
        let x = rng.range(region.left(), region.right() + 1);
        let y = rng.range(region.top(), region.bottom() + 1);

        if let Some(idx) = map.get_wrapped_index(x, y) {
            if let Some(cell) = map.get_cell(idx) {
                if func(x, y, cell) {
                    return Some(Point::new(x, y));
                }
            }
        }
    }
    None
}

pub fn closest_points_matching<F>(map: &Map, x: i32, y: i32, func: F) -> Vec<Point>
where
    F: Fn(i32, i32, CellRef) -> bool,
{
    let mut points: Vec<Point> = Vec::new();

    let start = Point::new(x, y);
    let mut current = Point::new(x, y);

    for dist in 0..200 {
        for x1 in (x - dist)..(x + dist + 1) {
            for y1 in (y - dist)..(y + dist + 1) {
                let (x2, y2) = match map.try_wrap_xy(x1, y1) {
                    None => continue,
                    Some((x, y)) => (x, y),
                };
                current.x = x2;
                current.y = y2;
                if distance::manhattan(&start, &current).round() as i32 != dist {
                    continue;
                }
                if let Some(index) = map.get_wrapped_index(x2, y2) {
                    if let Some(cell) = map.get_cell(index) {
                        if func(x2, y2, cell) {
                            points.push(Point::new(x2, y2));
                        }
                    }
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
            let index = map.get_wrapped_index(x, y).unwrap();
            let cell = map.get_cell(index).unwrap();
            let sprite = cell.sprite();
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn default_no_region() {
        let map = Map::new(100, 100);
        assert_eq!(map.size(), (100, 100));

        assert_eq!(map.get_wrapped_index(5, 5).unwrap(), 505);
        assert_eq!(map.try_wrap_xy(5, 5).unwrap(), (5, 5));
        assert_eq!(map.get_wrapped_index(15, 15).unwrap(), 1515);
        assert_eq!(map.try_wrap_xy(15, 15).unwrap(), (15, 15));

        let region = map.region();
        assert_eq!(region.left(), 0);
        assert_eq!(region.top(), 0);
        assert_eq!(region.right(), 99);
        assert_eq!(region.bottom(), 99);
        assert_eq!(region.width(), 100);
        assert_eq!(region.height(), 100);

        assert_eq!(map.size(), (100, 100));
    }

    #[test]
    fn region() {
        let mut map = Map::new(100, 100);
        map.select_region(0, 0, 10, 10);
        {
            let region = map.region();
            assert_eq!(region.left(), 0);
            assert_eq!(region.top(), 0);
            assert_eq!(region.right(), 9);
            assert_eq!(region.bottom(), 9);
            assert_eq!(region.width(), 10);
            assert_eq!(region.height(), 10);
        }
        assert_eq!(map.size(), (10, 10));

        assert_eq!(map.get_wrapped_index(5, 5).unwrap(), 505);
        assert_eq!(map.try_wrap_xy(5, 5).unwrap(), (5, 5));
        assert_eq!(map.get_wrapped_index(15, 15), None);
        assert_eq!(map.try_wrap_xy(15, 15), None);

        map.set_region_pos(10, 10);
        {
            let region = map.region();
            assert_eq!(region.left(), 10);
            assert_eq!(region.top(), 10);
            assert_eq!(region.right(), 19);
            assert_eq!(region.bottom(), 19);
            assert_eq!(region.width(), 10);
            assert_eq!(region.height(), 10);
        }
        assert_eq!(map.try_wrap_xy(5, 5), None);
        assert_eq!(map.try_wrap_xy(15, 15).unwrap(), (15, 15));

        map.move_region_pos(0, 10);
        {
            let region = map.region();
            assert_eq!(region.left(), 10);
            assert_eq!(region.top(), 20);
            assert_eq!(region.right(), 19);
            assert_eq!(region.bottom(), 29);
            assert_eq!(region.width(), 10);
            assert_eq!(region.height(), 10);
        }
        assert_eq!(map.try_wrap_xy(15, 15), None);
        assert_eq!(map.try_wrap_xy(15, 25).unwrap(), (15, 25));
    }

    #[test]
    fn region_wrap() {
        let mut map = Map::new(100, 100);
        map.select_region(0, 0, 10, 10);
        map.wrap = Wrap::XY;

        assert_eq!(map.size(), (10, 10));

        assert_eq!(map.get_wrapped_index(5, 5).unwrap(), 505);
        assert_eq!(map.try_wrap_xy(5, 5).unwrap(), (5, 5));
        assert_eq!(map.get_wrapped_index(15, 15).unwrap(), 1515);
        assert_eq!(map.try_wrap_xy(15, 15).unwrap(), (5, 5));
    }
}
