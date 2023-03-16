use acanja::map::prefab::{PrefabFileLoader, Prefabs};
// use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Write};
use gw_app::*;
use gw_util::json::parse_file;
use gw_util::point::Point;
use gw_world::action::move_step::MoveStepAction;
use gw_world::actor::Actor;
use gw_world::hero::Hero;
use gw_world::level::Level;
use gw_world::map::Map;
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::DoNextActionResult;
// use gw_world::memory::MapMemory;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::map::Wrap;
use gw_world::tile::TileJsonFileLoader;
use gw_world::tile::Tiles;
use gw_world::widget::Viewport;
use std::{collections::HashMap, fs::read_to_string};

struct UserControl;

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
    for (ch, info) in tile_info.iter() {
        if info.is_string() {
            if default_tile == "NONE" {
                default_tile = info.to_string().to_uppercase();
            }
            tile_lookup.insert(ch.to_string(), Place::Tile(info.to_string().to_uppercase()));
        } else if info.is_map() {
            let info = info.as_map().unwrap();

            let tile_id = info.get(&"id".into()).expect(&format!(
                "Tile entry is missing 'id' field - {}",
                ch.to_string()
            ));
            tile_lookup.insert(
                ch.to_string(),
                Place::Tile(tile_id.to_string().to_uppercase()),
            );

            if default_tile == "NONE" || info.contains_key(&"default".into()) {
                default_tile = tile_id.to_string().to_uppercase();
            }
        } else {
            panic!(
                "Found unexpected tiles entry - {}: {:?}",
                ch.to_string(),
                info
            );
        }
    }

    println!("DEFAULT TILE = {}", default_tile);

    tile_lookup.insert("DEFAULT".to_string(), Place::Tile(default_tile.clone()));

    println!("Tile Lookup = {:?}", tile_lookup);

    // Setup Fixtures

    let fixture_info = root.get(&"fixtures".into()).unwrap().as_map().unwrap();
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
                        Place::Fixture(ground, fixture) => (ground.clone(), Some(fixture.clone())),
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

    let def_tile = tiles
        .get(&default_tile)
        .expect(&format!("No default tile in tiles! - {}", default_tile));

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
                        let t = tiles
                            .get(tile)
                            .expect(&format!("Failed to find tile in map - {}", tile));
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

const CAMERA_WIDTH: u32 = 1024 / 32;
const CAMERA_HEIGHT: u32 = 768 / 32;

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

        // add position + sprite for actor
        let entity = level.world.push((
            Position::new(start_pos.x, start_pos.y),
            Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()),
            UserControl, // Do we need this?
            Actor::new("USER_CONTROL"),
        ));

        let mut camera = Camera::new(CAMERA_WIDTH, CAMERA_HEIGHT);
        camera.set_follows(entity);
        level.resources.insert(camera);

        level.resources.insert(Hero::new(entity));
        level.reset_tasks();

        ecs.resources.insert(level);

        size
    }

    fn post_action(&mut self, level: &mut Level) {
        // Post Update
        update_camera_follows(level);
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
                    let mut level = ecs.resources.get_mut::<Level>().unwrap();
                    move_hero(&mut *level, 0, 1);
                    drop(level);
                }
                VirtualKeyCode::Left => {
                    let mut level = ecs.resources.get_mut::<Level>().unwrap();
                    move_hero(&mut *level, -1, 0);
                    drop(level);
                }
                VirtualKeyCode::Up => {
                    let mut level = ecs.resources.get_mut::<Level>().unwrap();
                    move_hero(&mut *level, 0, -1);
                    drop(level);
                }
                VirtualKeyCode::Right => {
                    let mut level = ecs.resources.get_mut::<Level>().unwrap();
                    move_hero(&mut *level, 1, 0);
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

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

        let mut level = ecs.resources.get_mut::<Level>().unwrap();

        level.execute(|level, executor| {
            // Update
            loop {
                // if world.is_game_over() {
                //     return (self.game_over)(world, ctx);
                // } else if !world.animations().is_empty() {
                //     return ScreenResult::Continue;
                // }
                let res = executor.do_next_action(&mut *level);
                self.post_action(&mut *level);
                match res {
                    DoNextActionResult::Done => {
                        return ScreenResult::Continue;
                    }
                    DoNextActionResult::Mob => {
                        continue;
                    }
                    DoNextActionResult::Hero => {
                        return ScreenResult::Continue;
                    }
                    DoNextActionResult::PushMode(mode) => return ScreenResult::Push(mode),
                }
            }
        })
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
        .font("assets/font_32x58.png")
        .file(
            "assets/maps/tiles.jsonc",
            Box::new(TileJsonFileLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_hero(level: &mut Level, dx: i32, dy: i32) {
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let mut entry = level.world.entry(hero_entity).unwrap();
    let actor = entry.get_component_mut::<Actor>().unwrap();
    actor.next_action = Some(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}
