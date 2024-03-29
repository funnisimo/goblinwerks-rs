use super::{load_level_data_file, LevelDataLoader};
use gw_app::{
    loader::{LoadError, LoadHandler, Loader},
    log,
};
use gw_ecs::{Ecs, ReadGlobal, SystemData, World, WriteGlobal};
use gw_util::{
    json::parse_string,
    point::Point,
    rect::Rect,
    value::Value,
    xy::{Lock, Wrap},
};
use gw_world::{
    being::{set_field, spawn_being, BeingKind, BeingKinds},
    camera::Camera,
    effect::{parse_effects, BoxedEffect, Message, Portal},
    fov::FOV,
    horde::{parse_spawn, HordeSpawner},
    level::NeedsDraw,
    log::Logger,
    map::Map,
    task::UserAction,
    tile::{Tile, Tiles},
};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    sync::Arc,
};

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

#[derive(Debug, Clone, Default)]
pub struct Cell {
    tile: Option<TileType>,                     // ground tile
    fixture: Option<TileType>,                  // ground, fixture id
    location: Option<String>,                   // ground, fixture, location name
    effects: HashMap<String, Vec<BoxedEffect>>, // action effects
    flavor: Option<String>,
    being: Option<Arc<BeingKind>>, // actor kind id
}

impl Cell {
    pub fn new() -> Self {
        Cell::default()
    }
}

pub struct LevelData {
    pub id: String,
    pub cell_lookup: HashMap<String, Cell>, // char -> id
    pub default_entry: String,              // char
    pub map_data: Option<MapData>,
    pub map_size: (u32, u32),
    pub map_wrap: bool,
    pub welcome: Option<String>,
    pub camera_size: (u32, u32),
    pub region: Option<Rect>,
    pub fov: Option<u32>,
    pub groups: HashMap<String, HashMap<String, Value>>,
    pub spawn: Option<HashSet<HordeSpawner>>,
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
            welcome: None,
            camera_size: (11, 11),
            region: None,
            fov: None,
            groups: HashMap::new(),
            spawn: None,
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

        println!("Processing level - {}", path);
        let value = match gw_util::json::parse_string(&string) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Failed to parse '{}' => {}",
                    path, e
                )))
            }
            Ok(v) => v,
        };

        ecs.ensure_global::<Tiles>();
        ecs.ensure_global::<BeingKinds>();

        let level_data = load_level_data(ecs, value);

        match level_data.map_data {
            None => panic!("No map data in level file."),
            Some(MapData::Data(_)) => {
                let level = make_level(ecs, level_data);
                log(format!("Adding Level - {}", level.id()));
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

                ecs.write_global::<Loader>()
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
pub fn load_level_data(ecs: &Ecs, json: Value) -> LevelData {
    // let path = "./assets/maps/";
    // let json_file = "sosaria.jsonc";

    let tiles = ecs.read_global::<Tiles>();
    let being_kinds = ecs.read_global::<BeingKinds>();

    let root = json.as_map().unwrap();

    // println!("Root keys = {:?}", root.keys());

    // Setup Tiles
    let mut cell_lookup: HashMap<String, Cell> = HashMap::new(); // char -> cell
    let mut default_entry: Option<String> = None;

    let map_id = match root.get(&"id".into()) {
        None => panic!("Map file does not have id field."),
        Some(val) => val.to_string().to_uppercase(),
    };

    let group_info_map: HashMap<String, HashMap<String, Value>> = match root.get(&"groups".into()) {
        None => HashMap::new(),
        Some(info) => {
            if !info.is_map() {
                panic!("groups section must be object type");
            }
            let info_map = info.as_map().unwrap();
            let mut result = HashMap::new();

            for (section, fields) in info_map.iter() {
                if !fields.is_map() {
                    panic!("each entry in groups section must be object.");
                }

                let fields_map = HashMap::<String, Value>::from_iter(
                    fields
                        .clone()
                        .to_map()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.clone())),
                );
                result.insert(section.to_string(), fields_map);
            }

            result
        }
    };

    println!("GROUPS = {:?}", group_info_map);

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
                if let Some(being_value) = info.get(&"being".into()) {
                    log(format!("Being - {:?}", being_value));
                    if being_value.is_string() {
                        cell.being = match being_kinds.get(&being_value.to_string().to_uppercase())
                        {
                            None => panic!("Being kind is unknown = {}", being_value.to_string()),
                            Some(k) => Some(k),
                        };
                    } else if being_value.is_map() {
                        let map = being_value.as_map().unwrap();

                        if let Some(kind_value) = map.get(&"kind".into()) {
                            let id = format!(
                                "{}@{}-{}",
                                kind_value.to_string().to_uppercase(),
                                map_id,
                                ch,
                            );
                            let mut builder = BeingKind::builder(&id);

                            match being_kinds.get(&kind_value.to_string().to_uppercase()) {
                                None => panic!(
                                    "Being kind extends missing being - {}",
                                    kind_value.to_string()
                                ),
                                Some(base) => {
                                    builder.extend(&base);
                                }
                            }

                            // ALL BEINGS HAVE THESE VALUES
                            if let Some(group_values) = group_info_map.get("BEING") {
                                for (k, v) in group_values.iter() {
                                    set_field(&mut builder, k, v).unwrap();
                                }
                            }

                            // ANY CUSTOM GROUP VALUES
                            if let Some(groups) = map.get(&"groups".into()) {
                                for group in
                                    groups.to_string().split(&['|', ',', ':']).map(|v| v.trim())
                                {
                                    if let Some(group_values) = group_info_map.get(group) {
                                        for (k, v) in group_values.iter() {
                                            set_field(&mut builder, k, v).unwrap();
                                        }
                                    }
                                }
                            } else if let Some(group) = map.get(&"group".into()) {
                                let group = group.to_string();
                                if let Some(group_values) = group_info_map.get(group.trim()) {
                                    for (k, v) in group_values.iter() {
                                        set_field(&mut builder, k, v).unwrap();
                                    }
                                }
                            }

                            if let Some(talk) = map.get(&"talk".into()) {
                                builder.talk(&talk.to_string());
                            }

                            if let Some(name) = map.get(&"name".into()) {
                                builder.name(&name.to_string());
                            }

                            if let Some(ai) = map.get(&"ai".into()) {
                                if ai.is_string() {
                                    builder.ai(&ai.to_string());
                                } else if ai.is_bool() {
                                    if !ai.as_bool().unwrap() {
                                        builder.ai("IDLE");
                                    }
                                }
                            }

                            let new_being = builder.build();
                            println!("CUSTOM BEING - {:?}", new_being);
                            cell.being = Some(new_being);
                        } else {
                            panic!("Being with no kind information - {:?}", being_value);
                        }
                    } else {
                        panic!("Invalid being data type = {:?}", being_value);
                    };
                }

                // item

                // location
                if let Some(location_value) = info.get(&"location".into()) {
                    let location = location_value.to_string().to_uppercase();
                    cell.location = Some(location);
                }

                // flavor
                if let Some(flavor_value) = info.get(&"flavor".into()) {
                    let flavor = flavor_value.to_string();
                    cell.flavor = Some(flavor);
                }

                // use
                if let Some(use_value) = info.get(&"use".into()) {
                    match parse_effects(use_value) {
                        Err(e) => panic!("{}", e),
                        Ok(val) => {
                            cell.effects.insert("USE".to_string(), val);
                        }
                    }
                }

                // climb
                if let Some(climb_value) = info.get(&"climb".into()) {
                    if climb_value.is_string() {
                        cell.effects.insert(
                            "CLIMB".to_string(),
                            vec![Box::new(Portal::new(
                                climb_value.to_string(),
                                "START".to_string(),
                            ))],
                        );
                    } else {
                        match parse_effects(climb_value) {
                            Err(e) => panic!("{}", e),
                            Ok(val) => {
                                cell.effects.insert("CLIMB".to_string(), val);
                            }
                        }
                    }
                }

                // descend
                if let Some(descend_value) = info.get(&"descend".into()) {
                    if descend_value.is_string() {
                        cell.effects.insert(
                            "DESCEND".to_string(),
                            vec![Box::new(Portal::new(
                                descend_value.to_string(),
                                "START".to_string(),
                            ))],
                        );
                    } else {
                        match parse_effects(descend_value) {
                            Err(e) => panic!("{}", e),
                            Ok(val) => {
                                cell.effects.insert("DESCEND".to_string(), val);
                            }
                        }
                    }
                }
                // enter
                if let Some(enter_value) = info.get(&"enter".into()) {
                    match parse_effects(enter_value) {
                        Err(e) => panic!("{}", e),
                        Ok(val) => {
                            cell.effects.insert("ENTER".to_string(), val);
                        }
                    }
                }

                // message - shorthand for "enter": { "message": <TEXT> }
                if let Some(message_value) = info.get(&"message".into()) {
                    if message_value.is_string() {
                        let text = message_value.to_string();
                        match cell.effects.get_mut("ENTER") {
                            None => {
                                cell.effects.insert(
                                    "ENTER".to_string(),
                                    vec![Box::new(Message::new(&text))],
                                );
                            }
                            Some(current) => {
                                current.push(Box::new(Message::new(&text)));
                            }
                        }
                    } else {
                        panic!("Message must be a string!  found: {:?}", message_value);
                    }
                }

                // exit
                if let Some(exit_value) = info.get(&"exit".into()) {
                    match parse_effects(exit_value) {
                        Err(e) => panic!("{}", e),
                        Ok(val) => {
                            cell.effects.insert("EXIT".to_string(), val);
                        }
                    }
                }

                cell_lookup.insert(text, cell);
            }
        }
    }

    // println!("Default entry - {:?}", default_entry.as_ref().unwrap());
    // println!("Tile Lookup = {:?}", cell_lookup.keys());

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

    // println!("Tile Lookup = {:?}", cell_lookup);
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
    level_data.welcome = {
        match root.get(&"welcome".into()) {
            None => None,
            Some(val) => match val.is_string() {
                false => None,
                true => Some(val.to_string()),
            },
        }
    };

    level_data.fov = {
        match root.get(&"fov".into()) {
            None => None,
            Some(fov_data) => match fov_data {
                Value::Boolean(b) => match b {
                    true => Some(11),
                    false => None,
                },
                Value::Integer(v) => Some(*v as u32),
                _ => panic!(
                    "Received invalid fov value = {:?}, expected # or false.",
                    fov_data
                ),
            },
        }
    };

    // camera size
    if let Some(camera_value) = root.get(&"camera".into()) {
        if camera_value.is_map() {
            let camera_map = camera_value.as_map().unwrap();
            if let Some(size_val) = camera_map.get(&"size".into()) {
                let size: Vec<i64> = size_val
                    .as_list()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.as_int().unwrap())
                    .collect();

                level_data.camera_size = (size[0] as u32, size[1] as u32);
            }
        } else if camera_value.is_list() {
            let size: Vec<i64> = camera_value
                .as_list()
                .unwrap()
                .into_iter()
                .map(|v| v.as_int().unwrap())
                .collect();

            level_data.camera_size = (size[0] as u32, size[1] as u32);
        }
    }

    // display region
    if let Some(region_val) = root.get(&"region".into()) {
        if region_val.is_list() {
            let region = region_val.as_list().unwrap();
            if region.len() != 4 || region.iter().any(|v| !v.is_int()) {
                panic!(
                    "map region must be array of [x,y,w,h].  Found: {:?}",
                    region
                );
            }

            let vals: Vec<i64> = region.iter().map(|v| v.as_int().unwrap()).collect();

            level_data.region = Some(Rect::with_size(
                vals[0] as i32,
                vals[1] as i32,
                vals[2] as u32,
                vals[3] as u32,
            ));
        } else {
            panic!(
                "map region must be array of [x,y,w,h].  Found: {:?}",
                region_val
            );
        }
    }

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

    // spawn
    if let Some(spawn_value) = root.get(&"spawn".into()) {
        match parse_spawn(spawn_value) {
            Ok(info) => {
                log(format!("Parsed spawn info - {:?}", info));
                level_data.spawn = Some(info);
            }
            Err(e) => panic!("spawn has invalid value - {:?}", e),
        }
    }

    level_data
}

pub fn make_level<'a>(ecs: &'a mut Ecs, mut level_data: LevelData) -> &'a World {
    let cell_lookup = level_data.cell_lookup;
    let (width, height) = level_data.map_size;
    let wrap = level_data.map_wrap;
    let data = match level_data.map_data {
        Some(MapData::Data(data)) => data,
        _ => panic!("Must have map data to make level"),
    };

    let mut world = ecs.create_world(level_data.id.as_str());
    gw_world::setup_world(world);

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
    map.welcome = level_data.welcome.take();
    map.fill(def_tile.clone());
    if wrap {
        map.wrap = Wrap::XY;
    } else {
        map.lock = Lock::XY;
    }

    for (y, line) in data.iter().enumerate() {
        let y = y as i32;
        if y >= height as i32 {
            break;
        }
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            if x >= width as i32 {
                break;
            }
            let index = map.get_index(x, y).unwrap();
            let char = format!("{}", ch);
            match cell_lookup.get(&char) {
                None => panic!("Unknown tile in map data - {}", char),
                Some(place) => {
                    match place.tile {
                        None => {
                            map.reset_tiles(index, def_tile.clone());
                        }
                        Some(TileType::Tile(ref tile)) => map.reset_tiles(index, tile.clone()),
                        _ => {
                            panic!("Invalid tile entry!");
                        }
                    };

                    if let Some(tile_type) = place.fixture.as_ref() {
                        match tile_type {
                            TileType::Tile(tile) => {
                                map.place_tile(index, tile.clone());
                            }
                            _ => {}
                        }
                    }

                    if let Some(ref location) = place.location {
                        map.set_location(location, index);
                    }

                    for (action, effects) in place.effects.iter() {
                        map.set_effects(index, action, effects.clone());
                    }
                }
            }
        }
    }

    map.reveal_all();
    map.make_fully_visible();

    if let Some(ref region) = level_data.region {
        log("SETTING REGION");
        map.select_region(region.left(), region.top(), region.width(), region.height());
    }

    world.insert_resource(map);

    for (y, line) in data.iter().enumerate() {
        let y = y as i32;
        if y >= height as i32 {
            break;
        }
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            if x >= width as i32 {
                break;
            }
            let char = format!("{}", ch);
            match cell_lookup.get(&char) {
                None => panic!("Unknown tile in map data - {}", char),
                Some(place) => {
                    if let Some(ref kind) = place.being {
                        // log(format!(
                        //     "Spawn Actor - {} @ {},{} - being: {:?}",
                        //     kind.id, x, y, kind.being
                        // ));
                        spawn_being(kind, &mut world, Point::new(x, y));
                    }
                }
            }
        }
    }

    if level_data.camera_size.0 > 0 {
        let mut camera = world.write_resource::<Camera>();
        log(format!("MAP CAMERA SIZE = {:?}", level_data.camera_size));
        camera.resize(level_data.camera_size.0, level_data.camera_size.1);
    }

    if let Some(range) = level_data.fov {
        world.insert_resource(FOV::new(range));
        log(format!("[[[[ FOV ]]]] = {}", range));
    }

    if let Some(spawn) = level_data.spawn {
        // TODO - Need to support multiple live spawners (e.g. Land and Sea)
        if let Some(first) = spawn.into_iter().next() {
            log(format!("CONFIGURED SPAWN - {:?}", first));
            world.insert_resource(first);
        }
    }

    world.ensure_resource::<NeedsDraw>();
    world.ensure_resource::<UserAction>();
    world.ensure_global::<Logger>();

    world
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

pub fn load_level_file<'a>(
    ecs: &'a mut Ecs,
    filename: &str,
    // tiles: &Tiles,
    // actor_kinds: &BeingKinds,
) -> &'a World {
    let file_text = read_to_string(filename).expect(&format!("Failed to open {filename}"));

    let json = parse_string(&file_text).expect(&format!("Failed to parse level file - {filename}"));

    let mut level_data = load_level_data(ecs, json);

    match level_data.map_data {
        None => panic!("No map data in level file."),
        Some(MapData::Data(_)) => make_level(ecs, level_data),
        Some(MapData::FileName(ref file)) => {
            // Need to load level file

            let level_data_filename = if file.contains("/") {
                file.clone()
            } else {
                let path = match filename.rsplit_once("/") {
                    None => "./".to_string(),
                    Some((a, _)) => a.to_string(),
                };
                format!("{}/{}", path, file)
            };

            log(format!("Loading level data file - {}", level_data_filename));

            load_level_data_file(&level_data_filename, &mut level_data);

            make_level(ecs, level_data)
        }
    }
}
