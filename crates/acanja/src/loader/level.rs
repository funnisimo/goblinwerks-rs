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
    tile::Tiles,
};
use std::collections::HashMap;

pub enum MapData {
    Data(Vec<String>),
    FileName(String),
}

pub struct LevelData {
    pub id: String,
    pub tile_lookup: HashMap<String, Place>, // char -> id
    pub default_tile: String,                // id
    pub map_data: Option<MapData>,
    pub map_size: (u32, u32),
    pub map_wrap: bool,
}

impl LevelData {
    pub fn new(id: String, tile_lookup: HashMap<String, Place>, default_tile: String) -> Self {
        LevelData {
            id,
            tile_lookup,
            default_tile,
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

        let level_data = load_level_data(value);

        match level_data.map_data {
            None => panic!("No map data in level file."),
            Some(MapData::Data(_)) => {
                let level = make_level(level_data, &tiles);
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

#[derive(Debug, Clone)]
pub enum Place {
    Tile(String),                             // ground tile
    Fixture(String, String),                  // ground, fixture id
    Actor(String, String),                    // ground, actor kind id
    Location(String, Option<String>, String), // ground, fixture, location name
    Portal(String, Option<String>, PortalInfo),
}

pub fn load_level_data(json: Value) -> LevelData {
    // let path = "./assets/maps/";
    // let json_file = "sosaria.jsonc";

    let root = json.as_map().unwrap();

    println!("Root keys = {:?}", root.keys());

    // Setup Tiles
    let mut tile_lookup: HashMap<String, Place> = HashMap::new(); // char -> id
    let mut default_tile = "NONE".to_string();

    let id = root.get(&"id".into()).unwrap().to_string().to_uppercase();

    let tile_info = root.get(&"tiles".into()).unwrap().as_map().unwrap();
    for (ch, info) in tile_info.iter() {
        let glyphs = ch.to_string();

        for glyph in glyphs.chars() {
            let text = format!("{}", glyph);

            if info.is_string() {
                let tile_id = info.to_string().to_uppercase().replace("{}", &text);

                if default_tile == "NONE" {
                    default_tile = tile_id.clone();
                }
                tile_lookup.insert(text, Place::Tile(tile_id));
            } else if info.is_map() {
                let info = info.as_map().unwrap();

                let tile_id = info
                    .get(&"id".into())
                    .expect(&format!(
                        "Tile entry is missing 'id' field - {}",
                        ch.to_string()
                    ))
                    .to_string()
                    .to_uppercase()
                    .replace("{}", &text);

                tile_lookup.insert(text, Place::Tile(tile_id.clone()));

                if default_tile == "NONE" || info.contains_key(&"default".into()) {
                    default_tile = tile_id;
                }
            } else {
                panic!(
                    "Found unexpected tiles entry - {}: {:?}",
                    ch.to_string(),
                    info
                );
            }
        }
    }

    println!("DEFAULT TILE = {}", default_tile);

    tile_lookup.insert("DEFAULT".to_string(), Place::Tile(default_tile.clone()));

    println!("Tile Lookup = {:?}", tile_lookup);

    // Setup Fixtures

    if let Some(fixture_value) = root.get(&"fixtures".into()) {
        let fixture_info = fixture_value.as_map().unwrap();

        for (id, info) in fixture_info.iter() {
            if info.is_string() {
                tile_lookup.insert(
                    id.to_string(),
                    Place::Fixture(default_tile.clone(), info.to_string().to_uppercase()),
                );
            } else if info.is_map() {
                let info = info.as_map().unwrap();

                let fix_id = info.get(&"id".into()).expect(&format!(
                    "Fixture entry is missing 'id' field - {}",
                    id.to_string()
                ));
                let tile_id = match info.get(&"tile".into()) {
                    None => default_tile.clone(),
                    Some(field) => match tile_lookup.get(&field.to_string()) {
                        None => panic!(
                            "Fixture entry references unknown tile - {}",
                            field.to_string()
                        ),
                        Some(t) => match t {
                            Place::Tile(v) => v.clone(),
                            x => panic!("Fixture entry references wrong type = {:?}", x),
                        },
                    },
                };
                tile_lookup.insert(
                    id.to_string(),
                    Place::Fixture(tile_id, fix_id.to_string().to_uppercase()),
                );
            } else {
                panic!(
                    "Found unexpected fixtures entry - {}: {:?}",
                    id.to_string(),
                    info
                );
            }
        }
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Setup Items

    // Setup Actors

    if let Some(actor_value) = root.get(&"actors".into()) {
        let actor_info = actor_value.as_map().unwrap();

        for (id, info) in actor_info.iter() {
            let glyph = id.to_string();
            // println!("TILE {} = {:?}", glyph, info);

            let info = info.as_map().unwrap();

            let id = info.get(&"id".into()).unwrap().to_string().to_uppercase();
            match info.get(&"tile".into()) {
                None => {
                    tile_lookup.insert(
                        glyph.clone(),
                        Place::Actor(default_tile.clone(), id.clone()),
                    );
                }
                Some(t) => match tile_lookup.get(&t.to_string()) {
                    None => panic!("Actor has unknown tile! - {}", t.to_string()),
                    Some(place) => match place {
                        Place::Tile(ground) => {
                            tile_lookup
                                .insert(glyph.clone(), Place::Actor(ground.clone(), id.clone()));
                        }
                        _ => panic!("Actor tile field did not reference a tile"),
                    },
                },
            };
        }
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Locations

    if let Some(location_value) = root.get(&"locations".into()) {
        let location_info = location_value.as_map().unwrap();
        for (id, info) in location_info.iter() {
            let (ground, fixture, location) = if info.is_string() {
                (default_tile.clone(), None, info.to_string())
            } else if info.is_map() {
                let map = info.as_map().unwrap();

                let name = map
                    .get(&"name".into())
                    .expect("Location is missing name field.")
                    .to_string();

                let (mut tile, fixture) = match map.get(&"fixture".into()) {
                    None => (default_tile.clone(), None),
                    Some(fix) => match tile_lookup.get(&fix.to_string()) {
                        None => panic!("Tile location has unknown fixture - {}", fix.to_string()),
                        Some(p) => match p {
                            Place::Fixture(ground, fixture) => {
                                (ground.clone(), Some(fixture.clone()))
                            }
                            x => panic!("Tile location has fixture with wrong type - {:?}", x),
                        },
                    },
                };

                tile = match map.get(&"tile".into()) {
                    None => tile,
                    Some(t) => match tile_lookup.get(&t.to_string()) {
                        None => panic!("Tile location has unknown tile - {}", t.to_string()),
                        Some(p) => match p {
                            Place::Tile(t) => t.clone(),
                            x => panic!("Tile location has tile with wrong type - {:?}", x),
                        },
                    },
                };

                (tile, fixture, name)
            } else {
                panic!(
                    "Found unexpected locations entry - {}: {:?}",
                    id.to_string(),
                    info
                );
            };

            println!("LOCATION - {}", location);
            tile_lookup.insert(id.to_string(), Place::Location(ground, fixture, location));
        }
    }

    // setup portals

    if let Some(portal_value) = root.get(&"portals".into()) {
        let portal_info = portal_value.as_map().unwrap();
        for (id, info) in portal_info.iter() {
            let (ground, fixture, portal) = if info.is_string() {
                (
                    default_tile.clone(),
                    None,
                    PortalInfo::new(&info.to_string(), "START"),
                )
            } else if info.is_map() {
                let map = info.as_map().unwrap();

                let map_id = map
                    .get(&"map".into())
                    .expect("Location is missing map field.")
                    .to_string()
                    .to_uppercase();

                let location = map
                    .get(&"location".into())
                    .unwrap_or(&"START".into())
                    .to_string()
                    .to_uppercase();

                let (tile, fixture) = match map.get(&"fixture".into()) {
                    None => (default_tile.clone(), None),
                    Some(fix) => match tile_lookup.get(&fix.to_string()) {
                        None => panic!("Tile location has unknown fixture - {}", fix.to_string()),
                        Some(p) => match p {
                            Place::Fixture(ground, fixture) => {
                                (ground.clone(), Some(fixture.clone()))
                            }
                            x => panic!("Tile location has fixture with wrong type - {:?}", x),
                        },
                    },
                };

                let (tile, fixture) = match map.get(&"tile".into()) {
                    None => (tile, fixture),
                    Some(t) => match tile_lookup.get(&t.to_string()) {
                        None => panic!("Tile location has unknown tile - {}", t.to_string()),
                        Some(p) => match p {
                            Place::Tile(t) => (t.clone(), fixture),
                            Place::Fixture(t, f) => (t.clone(), Some(f.clone())),
                            x => panic!("Tile location has tile with wrong type - {:?}", x),
                        },
                    },
                };

                (tile, fixture, PortalInfo::new(&map_id, &location))
            } else {
                panic!(
                    "Found unexpected locations entry - {}: {:?}",
                    id.to_string(),
                    info
                );
            };

            println!("PORTAL - {:?}", portal);
            tile_lookup.insert(id.to_string(), Place::Portal(ground, fixture, portal));
        }
    }

    // Map

    println!("Tile Lookup = {:?}", tile_lookup);
    let mut level_data = LevelData::new(id, tile_lookup, default_tile);

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

pub fn make_level(level_data: LevelData, tiles: &Tiles) -> Level {
    let default_tile = level_data.default_tile;
    let tile_lookup = level_data.tile_lookup;
    let (width, height) = level_data.map_size;
    let wrap = level_data.map_wrap;
    let data = match level_data.map_data {
        Some(MapData::Data(data)) => data,
        _ => panic!("Must have map data to make level"),
    };

    let def_tile = tiles
        .get(&default_tile)
        .expect(&format!("No default tile in tiles! - {}", default_tile));

    let mut map = Map::new(width, height);
    map.fill(def_tile);
    if wrap {
        map.wrap = Wrap::XY;
    }

    for (y, line) in data.iter().enumerate() {
        let y = y as i32;
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            let char = format!("{}", ch);
            match tile_lookup.get(&char) {
                None => panic!("Unknown tile in map data - {}", char),
                Some(place) => match place {
                    Place::Tile(tile) => {
                        let t = tiles
                            .get(tile)
                            .expect(&format!("Failed to find tile in tiles - {}", tile));
                        map.reset_tiles(x, y, t);
                    }
                    Place::Fixture(tile, fix) => {
                        let t = tiles.get(tile).expect("Failed to find tile in tiles");
                        map.reset_tiles(x, y, t);
                        let f = tiles.get(fix).expect("Failed to find fixture in tiles.");
                        map.place_feature(x, y, f);
                    }
                    Place::Actor(tile, _) => {
                        let t = tiles.get(tile).expect("Failed to find tile in tiles");
                        map.reset_tiles(x, y, t);
                    }
                    Place::Location(tile, fix, name) => {
                        let t = tiles
                            .get(tile)
                            .expect(&format!("Failed to find tile in tiles - {}", tile));
                        map.reset_tiles(x, y, t);
                        if let Some(fix) = fix {
                            let f = tiles.get(fix).expect("Failed to find fixture in tiles.");
                            map.place_feature(x, y, f);
                        }
                        map.set_location(name, Point::new(x, y));
                    }
                    Place::Portal(tile, fix, portal) => {
                        let t = tiles
                            .get(tile)
                            .expect(&format!("Failed to find tile in tiles - {}", tile));
                        map.reset_tiles(x, y, t);
                        if let Some(fix) = fix {
                            let f = tiles.get(fix).expect("Failed to find fixture in tiles.");
                            map.place_feature(x, y, f);
                        }
                        map.set_portal(Point::new(x, y), portal.clone());
                    }
                },
            }
        }
    }

    map.reveal_all();
    map.make_fully_visible();

    let mut level = Level::new(&level_data.id);
    level.resources.insert(map);

    level
}
