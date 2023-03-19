use super::LevelDataLoader;
use gw_app::{
    ecs::{Read, ResourceSet, Write},
    loader::{LoadError, LoadHandler, Loader},
    log, Ecs,
};
use gw_util::{point::Point, value::Value};
use gw_world::{
    level::{Level, Levels},
    map::{Map, PortalInfo, Wrap},
    tile::{Tile, Tiles},
};
use std::{collections::HashMap, sync::Arc};

pub enum MapData {
    Data(Vec<String>),
    FileName(String),
}

#[derive(Debug, Clone)]
pub enum TileType {
    Ref(String),
    Id(String),
    Tile(Arc<Tile>),
}

#[derive(Debug, Clone)]
pub struct Cell {
    tile: Option<TileType>,    // ground tile
    fixture: Option<TileType>, // ground, fixture id
    location: Option<String>,  // ground, fixture, location name
    portal: Option<PortalInfo>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            tile: None,
            fixture: None,
            location: None,
            portal: None,
        }
    }
}

pub struct LevelData {
    pub id: String,
    pub cell_lookup: HashMap<String, Cell>, // char -> id
    pub default_entry: String,              // char
    pub map_data: Option<MapData>,
    pub map_size: (u32, u32),
    pub map_wrap: bool,
}

impl LevelData {
    pub fn new(id: String, cell_lookup: HashMap<String, Cell>, default_tile: String) -> Self {
        LevelData {
            id,
            cell_lookup,
            default_entry: default_tile,
            map_data: None,
            map_size: (0, 0),
            map_wrap: false,
        }
    }
}

#[derive(Default)]
pub struct LevelLoader {}

impl LevelLoader {
    pub fn new() -> Self {
        LevelLoader {}
    }
}

impl LoadHandler for LevelLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        let string = match String::from_utf8(data) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Malformed file data '{}' : {}",
                    path,
                    e.to_string()
                )))
            }
            Ok(v) => v,
        };

        let value = match gw_util::json::parse_string(&string) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Failed to parse '{}' => {}",
                    path, e
                )))
            }
            Ok(v) => v,
        };

        ecs.resources.get_mut_or_insert_with(|| Tiles::default());
        ecs.resources.get_mut_or_insert_with(|| Levels::default());
        let (tiles, mut loader, mut levels) =
            <(Read<Tiles>, Write<Loader>, Write<Levels>)>::fetch_mut(&mut ecs.resources);

        println!("Processing level - {}", path);
        let level_data = load_level_data(&*tiles, value);

        match level_data.map_data {
            None => panic!("No map data in level file."),
            Some(MapData::Data(_)) => {
                let level = make_level(level_data);
                log(format!("Adding Level - {}", level.id));

                levels.insert(level);
            }
            Some(MapData::FileName(ref file)) => {
                // Need to load level file

                let level_data_filename = if file.contains("/") {
                    file.clone()
                } else {
                    let path = match path.rsplit_once("/") {
                        None => "./".to_string(),
                        Some((a, _)) => a.to_string(),
                    };
                    format!("{}/{}", path, file)
                };

                log(format!("Loading level data file - {}", level_data_filename));

                loader
                    .load_file(
                        &level_data_filename,
                        Box::new(LevelDataLoader::new(level_data)),
                    )
                    .expect("Failed to load level data file");
            }
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////
pub fn load_level_data(tiles: &Tiles, json: Value) -> LevelData {
    // let path = "./assets/maps/";
    // let json_file = "sosaria.jsonc";

    let root = json.as_map().unwrap();

    println!("Root keys = {:?}", root.keys());

    // Setup Tiles
    let mut cell_lookup: HashMap<String, Cell> = HashMap::new(); // char -> cell
    let mut default_entry: Option<String> = None;

    let map_id = root.get(&"id".into()).unwrap().to_string().to_uppercase();

    let tile_info = root.get(&"tiles".into()).unwrap().as_map().unwrap();
    for (ch, info) in tile_info.iter() {
        let glyphs = ch.to_string();

        // This is to handle "X" and "ABCDEFGHI...", each char gets an entry in the table
        for glyph in glyphs.chars() {
            let text = format!("{}", glyph); // Convert back to string

            if info.is_string() {
                // "X": "TILE_ID"
                let tile_id = info.to_string().to_uppercase().replace("{}", &text);
                let mut cell = Cell::new();
                match tiles.get(&tile_id) {
                    None => panic!("Entry has unkown tile - {}:{}", glyphs, tile_id),
                    Some(tile) => cell.tile = Some(TileType::Tile(tile)),
                }

                cell_lookup.insert(text.clone(), cell);

                if default_entry.is_none() {
                    default_entry = Some(text.clone());
                }
            } else if info.is_map() {
                // "X": {...}
                let info = info.as_map().unwrap();

                if let Some(def_val) = info.get(&"default".into()) {
                    if def_val.as_bool().unwrap_or(false) {
                        default_entry = Some(text.clone());
                    }
                }

                if default_entry.is_none() {
                    default_entry = Some(text.clone());
                }

                let mut cell = Cell::new();

                // "tile": "X" vs "tile": "TILE_ID"
                if let Some(tile_val) = info.get(&"tile".into()) {
                    let tile = tile_val.to_string();
                    if tile.len() > 1 {
                        // "tile": "TILE_ID"
                        let tile_id = tile.to_uppercase().replace("{}", &text);
                        match tiles.get(&tile_id) {
                            None => panic!("Entry has unkown tile - {}:{}", glyphs, tile_id),
                            Some(tile) => cell.tile = Some(TileType::Tile(tile)),
                        }
                    } else {
                        // "tile": "X" ref prior entry
                        cell.tile = Some(TileType::Ref(tile));
                    };
                }

                if let Some(fixture_value) = info.get(&"fixture".into()) {
                    let fixture = fixture_value.to_string();
                    if fixture.len() > 1 {
                        // "fixture": "TABLE"
                        match tiles.get(&fixture.to_uppercase()) {
                            None => panic!("Entry has unkown fixture - {}:{}", glyphs, fixture),
                            Some(tile) => cell.fixture = Some(TileType::Tile(tile)),
                        }
                    } else {
                        // "fixture": "T"
                        cell.fixture = Some(TileType::Ref(fixture));
                    }
                }

                // actor

                // item

                if let Some(location_value) = info.get(&"location".into()) {
                    let location = location_value.to_string().to_uppercase();
                    cell.location = Some(location);
                }

                if let Some(portal_value) = info.get(&"portal".into()) {
                    if portal_value.is_string() {
                        cell.portal = Some(PortalInfo::new(
                            &portal_value.to_string().to_uppercase(),
                            "START",
                        ));
                    } else if portal_value.is_map() {
                        // need my map id
                        let portal_map = portal_value.as_map().unwrap();

                        let map = match portal_map.get(&"map".into()) {
                            None => map_id.clone(),
                            Some(val) => val.to_string().to_uppercase(),
                        };

                        let location = match portal_map.get(&"location".into()) {
                            None => "START".to_string(),
                            Some(val) => val.to_string().to_uppercase(),
                        };

                        cell.portal = Some(PortalInfo::new(&map, &location));
                    } else {
                        panic!("Invalid portal in entry - {}", text);
                    }
                }

                cell_lookup.insert(text, cell);
            }
        }
    }

    println!("Default entry - {:?}", default_entry.as_ref().unwrap());
    println!("Tile Lookup = {:?}", cell_lookup.keys());

    let entry = cell_lookup.get(default_entry.as_ref().unwrap()).unwrap();
    match entry.tile.as_ref() {
        None => panic!("No default tile set - {}!", default_entry.unwrap()),
        Some(TileType::Ref(_)) => panic!("Default entry cannot contain references!"),
        _ => {}
    }
    match entry.fixture.as_ref() {
        Some(TileType::Ref(_)) => panic!("Default entry cannot contain references!"),
        _ => {}
    }

    cell_lookup = resolve_references(&tiles, &cell_lookup);

    // Map

    println!("Tile Lookup = {:?}", cell_lookup);
    let mut level_data = LevelData::new(map_id, cell_lookup, default_entry.unwrap());

    let map_info = root.get(&"map".into()).unwrap().as_map().unwrap();

    let width: u32 = map_info
        .get(&"width".into())
        .expect("Map width is required")
        .as_int()
        .unwrap() as u32;
    let height: u32 = map_info
        .get(&"height".into())
        .expect("Map height is required")
        .as_int()
        .unwrap() as u32;

    let wrap: bool = map_info
        .get(&"wrap".into())
        .unwrap_or(&false.into())
        .as_bool()
        .unwrap();

    level_data.map_size = (width, height);
    level_data.map_wrap = wrap;

    if let Some(filename) = map_info.get(&"filename".into()) {
        level_data.map_data = Some(MapData::FileName(filename.to_string()));
        // let raw = read_to_string(&format!("{}/{}", path, filename.to_string()))
        //     .expect("Failed to read map data file");

        // let lines: Vec<String> = raw.split('\n').map(|v| v.to_string()).collect();
        // lines
    } else if let Some(data) = map_info.get(&"data".into()) {
        let data: Vec<String> = data
            .as_list()
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect();

        level_data.map_data = Some(MapData::Data(data));
    } else {
        panic!("Map has no data!");
    };

    level_data
}

pub fn make_level(level_data: LevelData) -> Level {
    let cell_lookup = level_data.cell_lookup;
    let (width, height) = level_data.map_size;
    let wrap = level_data.map_wrap;
    let data = match level_data.map_data {
        Some(MapData::Data(data)) => data,
        _ => panic!("Must have map data to make level"),
    };

    let def_entry = cell_lookup
        .get(&level_data.default_entry)
        .expect(&format!(
            "No default tile in tiles! - {}",
            level_data.default_entry
        ))
        .clone();

    let def_tile = match def_entry.tile {
        Some(TileType::Tile(tile)) => tile.clone(),
        _ => unreachable!(),
    };

    let mut map = Map::new(width, height);
    map.fill(def_tile.clone());
    if wrap {
        map.wrap = Wrap::XY;
    }

    for (y, line) in data.iter().enumerate() {
        let y = y as i32;
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            let char = format!("{}", ch);
            match cell_lookup.get(&char) {
                None => panic!("Unknown tile in map data - {}", char),
                Some(place) => {
                    match place.tile {
                        None => map.reset_tiles(x, y, def_tile.clone()),
                        Some(TileType::Tile(ref tile)) => map.reset_tiles(x, y, tile.clone()),
                        _ => {}
                    };

                    if let Some(tile_type) = place.fixture.as_ref() {
                        match tile_type {
                            TileType::Tile(tile) => {
                                map.place_tile(x, y, tile.clone());
                            }
                            _ => {}
                        }
                    }

                    if let Some(ref location) = place.location {
                        map.set_location(location, Point::new(x, y));
                    }

                    if let Some(ref portal) = place.portal {
                        map.set_portal(Point::new(x, y), portal.clone());
                    }
                }
            }
        }
    }

    map.reveal_all();
    map.make_fully_visible();

    let mut level = Level::new(&level_data.id);
    level.resources.insert(map);

    level
}

fn resolve_references(tiles: &Tiles, map: &HashMap<String, Cell>) -> HashMap<String, Cell> {
    let mut result = map.clone();

    let mut work_to_do = true;

    while work_to_do {
        work_to_do = false;
        for key in map.keys() {
            let cell = result.get(key).unwrap();

            let mut needs_work = false;
            match cell.tile {
                Some(TileType::Id(_)) | Some(TileType::Ref(_)) => {
                    needs_work = true;
                }
                _ => {}
            }
            match cell.fixture {
                Some(TileType::Id(_)) | Some(TileType::Ref(_)) => {
                    needs_work = true;
                }
                _ => {}
            }

            if !needs_work {
                continue;
            }

            let mut new_cell = cell.clone();

            match cell.fixture.as_ref() {
                Some(TileType::Id(id)) => {
                    let fixture = match tiles.get(&id) {
                        None => panic!("entry '{}' has invalid fixture = {}", key, id),
                        Some(tile) => tile,
                    };
                    new_cell.fixture = Some(TileType::Tile(fixture));
                }
                Some(TileType::Ref(glyph)) => {
                    drop(cell);
                    let ref_cell = match result.get(glyph) {
                        None => panic!("fixture Ref is missing - {} -> {}", key, glyph),
                        Some(cell) => cell,
                    };
                    match ref_cell.fixture.as_ref() {
                        None => panic!(
                            "Fixture references cell with no fixture - {} -> {}",
                            key, glyph
                        ),
                        Some(TileType::Tile(tile)) => {
                            new_cell.fixture = Some(TileType::Tile(tile.clone()));
                        }
                        _ => {
                            work_to_do = true;
                        }
                    }
                    if new_cell.tile.is_none() {
                        new_cell.tile = ref_cell.tile.clone();
                    }
                }
                _ => {}
            }

            match cell.tile.as_ref() {
                Some(TileType::Id(id)) => {
                    let tile = match tiles.get(&id) {
                        None => panic!("entry '{}' has invalid tile = {}", key, id),
                        Some(tile) => tile,
                    };
                    new_cell.tile = Some(TileType::Tile(tile));
                }
                Some(TileType::Ref(glyph)) => {
                    drop(cell);
                    let ref_cell = match result.get(glyph) {
                        None => panic!("tile Ref is missing - {} -> {}", key, glyph),
                        Some(cell) => cell,
                    };
                    match ref_cell.tile.as_ref() {
                        None => panic!(
                            "Fixture references cell with no tile - {} -> {}",
                            key, glyph
                        ),
                        Some(TileType::Tile(tile)) => {
                            new_cell.tile = Some(TileType::Tile(tile.clone()));
                        }
                        _ => {
                            work_to_do = true;
                        }
                    }
                }
                _ => {}
            }

            result.insert(key.clone(), new_cell);
        }
    }

    result
}
