use acanja::map::prefab::{PrefabFileLoader, Prefabs};
// use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Write};
use gw_app::*;
use gw_util::json::parse_file;
use gw_util::point::Point;
use gw_world::level::Level;
use gw_world::map::Map;
// use gw_world::memory::MapMemory;
use gw_world::tile::TileBuilder;
use gw_world::tile::{TileFileLoader, Tiles};
use gw_world::widget::{Camera, Viewport, Wrap};
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
enum Place {
    Tile(String),                             // ground tile
    Fixture(String, String),                  // ground, fixture id
    Actor(String, String),                    // ground, actor kind id
    Location(String, Option<String>, String), // ground, fixture, location name
}

fn load_map(path: &str, json_file: &str, tiles: &mut Tiles) -> Map {
    // let path = "./assets/maps/";
    // let json_file = "sosaria.jsonc";

    let json = parse_file(&format!("{}/{}", path, json_file)).unwrap();

    let root = json.as_map().unwrap();

    println!("Root keys = {:?}", root.keys());

    // Setup Tiles
    let mut tile_lookup: HashMap<String, Place> = HashMap::new(); // char -> id
    let mut default_tile = "NONE".to_string();

    let tile_info = root.get(&"tiles".into()).unwrap().as_map().unwrap();
    for (id, info) in tile_info.iter() {
        let glyph = id.to_string();
        // println!("TILE {} = {:?}", glyph, info);

        let info = info.as_map().unwrap();

        let id = info.get(&"id".into()).unwrap().to_string().to_uppercase();
        let mut builder = TileBuilder::new(&id);
        tile_lookup.insert(glyph.clone(), Place::Tile(id.clone()));

        builder.set("glyph", &glyph).expect("Failed to set glyph");

        for (field, val) in info.iter() {
            let mut field = field.to_string().to_lowercase();
            let mut value = val.to_string();

            if field == "id" {
                continue;
            } else if field == "blocks" {
                value = match value.as_str() {
                    "move" => "BLOCKS_MOVE".to_string(),
                    "sight" => "BLOCKS_VISION".to_string(),
                    "vision" => "BLOCKS_VISION".to_string(),
                    "true" => "BLOCKS_MOVE | BLOCKS_VISION".to_string(),
                    _ => panic!("Unexpected blocks value - {}", value),
                };
                field = "move".to_string();
            } else if field == "ch" {
                field = "glyph".to_string();
            } else if field == "glyph" {
                if val.is_int() {
                    let int: u32 = val.as_int().unwrap() as u32;
                    value = format!("0x{:x}", int);
                }
            }

            if field == "default" {
                tile_lookup.insert("DEFAULT".to_string(), Place::Tile(id.clone()));
                default_tile = id.clone();
            } else {
                builder
                    .set(field.as_str(), value.as_str())
                    .expect("Unknown field in tile");
            }
        }

        let tile = builder.build();
        println!("TILE = {:?}", &tile);
        tiles.insert(tile);

        if !tile_lookup.contains_key("DEFAULT") {
            tile_lookup.insert("DEFAULT".to_string(), Place::Tile(id.clone()));
            default_tile = id.clone();
        }
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Setup Fixtures

    let fixture_info = root.get(&"fixtures".into()).unwrap().as_map().unwrap();
    for (id, info) in fixture_info.iter() {
        let glyph = id.to_string();
        // println!("TILE {} = {:?}", glyph, info);

        let info = info.as_map().unwrap();

        let id = info.get(&"id".into()).unwrap().to_string().to_uppercase();
        let mut builder = TileBuilder::new(&id);

        builder.set("glyph", &glyph).expect("Failed to set glyph");
        builder
            .set("layer", "FIXTURE")
            .expect("Failed to set layer");

        for (field, val) in info.iter() {
            let mut field = field.to_string().to_lowercase();
            let mut value = val.to_string();

            if field == "id" {
                continue;
            } else if field == "blocks" {
                value = match value.as_str() {
                    "move" => "BLOCKS_MOVE".to_string(),
                    "sight" => "BLOCKS_VISION".to_string(),
                    "vision" => "BLOCKS_VISION".to_string(),
                    "true" => "BLOCKS_MOVE | BLOCKS_VISION".to_string(),
                    _ => panic!("Unexpected blocks value - {}", value),
                };
                field = "move".to_string();
            } else if field == "ch" {
                field = "glyph".to_string();
            } else if field == "glyph" {
                if val.is_int() {
                    let int: u32 = val.as_int().unwrap() as u32;
                    value = format!("0x{:x}", int);
                }
            }

            if field == "tile" {
                let tile_id = tile_lookup.get(&value).unwrap();
                match tile_id {
                    Place::Tile(ground) => {
                        tile_lookup
                            .insert(glyph.clone(), Place::Fixture(ground.clone(), id.clone()));
                    }
                    _ => panic!("Invalid ground tile in feature"),
                }
            } else {
                builder
                    .set(field.as_str(), value.as_str())
                    .expect("Unknown field in tile");
            }
        }

        let tile = builder.build();
        println!("FIXTURE = {:?}", &tile);
        tiles.insert(tile);

        if !tile_lookup.contains_key(&glyph) {
            let default = tile_lookup.get("DEFAULT").unwrap();
            match default {
                Place::Tile(ground) => {
                    tile_lookup.insert(glyph, Place::Fixture(ground.clone(), id.clone()));
                }
                _ => panic!("Invalid default ground tile!"),
            }
        }
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Setup Items

    // Setup Actors

    let actor_info = root.get(&"actors".into()).unwrap().as_map().unwrap();
    for (id, info) in actor_info.iter() {
        let glyph = id.to_string();
        // println!("TILE {} = {:?}", glyph, info);

        let info = info.as_map().unwrap();

        let id = info.get(&"id".into()).unwrap().to_string().to_uppercase();
        let tile = info.get(&"tile".into()).unwrap().to_string();

        match tile_lookup.get(&tile) {
            None => panic!("Actor has unknown tile!"),
            Some(place) => match place {
                Place::Tile(ground) => {
                    tile_lookup.insert(glyph.clone(), Place::Actor(ground.clone(), id.clone()));
                }
                _ => panic!("Actor tile field did not reference a tile"),
            },
        }
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Locations

    let location_info = root.get(&"locations".into()).unwrap().as_map().unwrap();
    for (id, info) in location_info.iter() {
        let glyph = id.to_string();
        // println!("TILE {} = {:?}", glyph, info);

        let info = info.as_map().unwrap();

        let name = info.get(&"name".into()).unwrap().to_string().to_uppercase();

        let (mut tile, fixture) = match info.get(&"fixture".into()) {
            None => (default_tile.clone(), None),
            Some(ch) => match tile_lookup.get(&ch.to_string()) {
                None => panic!("Location has unknown fixture!"),
                Some(place) => match place {
                    Place::Fixture(ground, fixture) => (ground.clone(), Some(fixture.clone())),
                    x => panic!("Location fixture is not a fixture - {:?}", x),
                },
            },
        };

        match info.get(&"tile".into()) {
            None => {}
            Some(ch) => match tile_lookup.get(&ch.to_string()) {
                None => panic!("Location has unknown tile!"),
                Some(place) => match place {
                    Place::Tile(ground) => {
                        tile = ground.clone();
                    }
                    _ => panic!("Actor tile field did not reference a tile"),
                },
            },
        };

        println!("LOCATION - {}", name);
        tile_lookup.insert(glyph, Place::Location(tile, fixture, name));
    }

    println!("Tile Lookup = {:?}", tile_lookup);

    // Map

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

    let _wrap: bool = map_info
        .get(&"wrap".into())
        .unwrap_or(&false.into())
        .as_bool()
        .unwrap();

    let data = if let Some(filename) = map_info.get(&"filename".into()) {
        let raw = read_to_string(&format!("{}/{}", path, filename.to_string()))
            .expect("Failed to read map data file");

        let lines: Vec<String> = raw.split('\n').map(|v| v.to_string()).collect();
        lines
    } else if let Some(data) = map_info.get(&"data".into()) {
        data.as_list()
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect()
    } else {
        panic!("Map has no data!");
    };

    let def_tile = tiles.get(&default_tile).expect("No default tile!");
    let mut map = Map::new(width, height);
    map.fill(def_tile);

    for (y, line) in data.iter().enumerate() {
        let y = y as i32;
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            let char = format!("{}", ch);
            match tile_lookup.get(&char) {
                None => panic!("Unknown tile in map data - {}", char),
                Some(place) => match place {
                    Place::Tile(tile) => {
                        let t = tiles.get(tile).expect("Failed to find tile in map");
                        map.reset_tiles(x, y, t);
                    }
                    Place::Fixture(tile, fix) => {
                        let t = tiles.get(tile).expect("Failed to find tile in map");
                        map.reset_tiles(x, y, t);
                        let f = tiles.get(fix).expect("Failed to find fixture.");
                        map.place_feature(x, y, f);
                    }
                    Place::Actor(tile, _) => {
                        let t = tiles.get(tile).expect("Failed to find tile in map");
                        map.reset_tiles(x, y, t);
                    }
                    Place::Location(tile, fix, name) => {
                        let t = tiles.get(tile).expect("Failed to find tile in map");
                        map.reset_tiles(x, y, t);
                        if let Some(fix) = fix {
                            let f = tiles.get(fix).expect("Failed to find fixture.");
                            map.place_feature(x, y, f);
                        }
                        map.set_location(name, Point::new(x, y));
                    }
                },
            }
        }
    }

    map
}

const MAP_WIDTH: u32 = 1024 / 32;
const MAP_HEIGHT: u32 = 768 / 32;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT")
            // .size(11, 11)
            .font("assets/font_32x58.png")
            // .extents(0.0, 0.0, 0.85, 0.85)
            .wrap(Wrap::XY)
            .build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_level(&self, ecs: &mut Ecs) -> (u32, u32) {
        let mut map = {
            let (mut tiles,) = <(Write<Tiles>,)>::fetch_mut(&mut ecs.resources);

            // log(format!("- prefabs: {}", prefabs.len()));
            // // let mut map = dig_room_level(&tiles, 80, 50);
            // build_world_map(&tiles, &prefabs, MAP_WIDTH, MAP_HEIGHT)

            let path = "./assets/maps/";
            let json_file = "sosaria.jsonc";

            load_map(path, json_file, &mut tiles)
        };

        map.reveal_all();
        map.make_fully_visible();

        let start_pos = map.get_location("START").unwrap();

        let size = map.get_size();

        let mut level = Level::new("WORLD");

        level.resources.insert(map);
        // level.resources.insert(MapMemory::new(160, 100));
        level
            .resources
            .insert(Camera::new(MAP_WIDTH, MAP_HEIGHT).with_center(start_pos.x, start_pos.y));

        ecs.resources.insert(level);

        size
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        resources.get_or_insert_with(|| Tiles::default());
        resources.get_or_insert_with(|| Prefabs::default());

        let _size = self.build_new_level(ecs);

        self.viewport.resize(11, 11);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    self.build_new_level(ecs);
                }
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    if let Some(mut camera) = level.resources.get_mut::<Camera>() {
                        log("Camera down");
                        camera.move_center(0, 1);
                    }
                    drop(level);
                }
                VirtualKeyCode::Left => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    if let Some(mut camera) = level.resources.get_mut::<Camera>() {
                        camera.move_center(-1, 0);
                    }
                    drop(level);
                }
                VirtualKeyCode::Up => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    if let Some(mut camera) = level.resources.get_mut::<Camera>() {
                        camera.move_center(0, -1);
                    }
                    drop(level);
                }
                VirtualKeyCode::Right => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    if let Some(mut camera) = level.resources.get_mut::<Camera>() {
                        camera.move_center(1, 0);
                    }
                    drop(level);
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    let map_size = level.resources.get::<Map>().unwrap().get_size();
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                    drop(level);
                }
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "VIEWPORT_MOVE" => {
                // let pt: Point = value.unwrap().try_into().unwrap();
                // log(format!("Mouse Pos = {}", pt));
            }
            "VIEWPORT_CLICK" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                log(format!("CLICK = {}", pt));
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        {
            let mut level = app.resources.get_mut::<Level>().unwrap();
            self.viewport.draw_level(&mut *level);
        }
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Acanja - World Viewer")
        .file(
            "assets/tiles.toml",
            Box::new(TileFileLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .font("assets/font_32x58.png")
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
