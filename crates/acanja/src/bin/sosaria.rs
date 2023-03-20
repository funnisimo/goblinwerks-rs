use acanja::loader::GameConfigLoader;
use acanja::map::prefab::{PrefabFileLoader, Prefabs};
// use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Write};
use gw_app::loader::{LoadError, LoadHandler, Loader};
use gw_app::*;
use gw_util::json::parse_file;
use gw_util::point::Point;
use gw_world::action::move_step::MoveStepAction;
use gw_world::actor::Actor;
use gw_world::hero::Hero;
use gw_world::level::{Level, Levels};
use gw_world::map::{Map, PortalFlags};
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::DoNextActionResult;
// use gw_world::memory::MapMemory;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::map::Wrap;
use gw_world::tile::TileJsonFileLoader;
use gw_world::tile::Tiles;
use gw_world::widget::Viewport;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserControl;

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
            // .wrap(Wrap::XY)
            .build();

        Box::new(MainScreen { viewport })
    }

    fn post_action(&mut self, level: &mut Level) {
        // Post Update
        update_camera_follows(level);
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        // resources.get_or_insert_with(|| Tiles::default());
        // resources.get_or_insert_with(|| Prefabs::default());

        let mut levels = resources.get_mut::<Levels>().unwrap();
        log(format!("START MAP = {}", levels.current_id()));
        levels.setup();
        let level = levels.current_mut();

        let start_pos = {
            let map = level.resources.get::<Map>().unwrap();
            map.get_location("START").unwrap()
        };
        let entity = level.world.push((
            Position::new(start_pos.x, start_pos.y),
            Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()),
            UserControl, // Do we need this?
            Actor::new("USER_CONTROL"),
        ));

        {
            let map = level.resources.get::<Map>().unwrap();
            log(format!("MAP LOCATIONS - {:?}", &map.locations));
            log(format!("MAP PORTALS - {:?}", &map.portals));
        }

        let mut camera = Camera::new(CAMERA_WIDTH, CAMERA_HEIGHT);
        camera.set_follows(entity);
        level.resources.insert(camera);

        level.resources.insert(Hero::new(entity));
        level.reset_tasks();
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    move_hero(ecs, 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_hero(ecs, -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_hero(ecs, 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_hero(ecs, 1, 0);
                }
                VirtualKeyCode::Period if key_down.shift => {
                    // Down
                    let hero_point = get_hero_point(ecs);
                    try_move_hero_world(ecs, &hero_point, PortalFlags::ON_DESCEND);
                }
                VirtualKeyCode::Comma if key_down.shift => {
                    // Up
                    let hero_point = get_hero_point(ecs);
                    try_move_hero_world(ecs, &hero_point, PortalFlags::ON_CLIMB);
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

        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

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
            let mut levels = app.resources.get_mut::<Levels>().unwrap();
            let level = levels.current_mut();
            self.viewport.draw_level(&mut *level);
        }
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Acanja - World Viewer")
        .font("assets/font_32x58.png")
        .register_components(|registry| {
            registry.register::<gw_world::position::Position>("Position".to_string());
            registry.register::<gw_world::sprite::Sprite>("Sprite".to_string());
            registry.register::<gw_world::actor::Actor>("Actor".to_string());
            registry.register::<UserControl>("UserControl".to_string());
        })
        .file(
            "assets/maps/tiles.jsonc",
            Box::new(TileJsonFileLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .file("assets/game_config.jsonc", Box::new(GameConfigLoader))
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_hero(ecs: &mut Ecs, dx: i32, dy: i32) {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();

    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let mut entry = level.world.entry(hero_entity).unwrap();
    let actor = entry.get_component_mut::<Actor>().unwrap();
    actor.next_action = Some(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}

fn get_hero_point(ecs: &mut Ecs) -> Point {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point()
}

fn try_move_hero_world(ecs: &mut Ecs, pt: &Point, flag: PortalFlags) -> bool {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();

    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let map = level.resources.get_mut::<Map>().unwrap();

    log(format!("CLICK = {:?}", pt));

    let (new_map_id, location) = {
        match map.get_portal(&pt) {
            None => return false,
            Some(info) => {
                if !info.flags().contains(flag) {
                    return false;
                }

                log(format!(
                    "Enter Portal = {} - {}::{}",
                    info.flavor().as_ref().unwrap_or(&"UNKNOWN".to_string()),
                    info.map_id(),
                    info.location()
                ));

                (info.map_id().to_string(), info.location().to_string())
            }
        }
    };

    let current_pt = level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point();

    drop(map);
    drop(level);

    let level = levels.current_mut();
    let mut map = level.resources.get_mut::<Map>().unwrap();

    map.remove_actor_at_xy(current_pt.x, current_pt.y, hero_entity);

    drop(map);
    drop(level);

    log("Moving hero to new world");
    let new_entity = levels.move_current_entity(hero_entity, &new_map_id);
    log("Changing current world");
    levels.set_current(&new_map_id);

    let level = levels.current_mut();
    level.resources.insert(Hero::new(hero_entity));

    let mut camera = Camera::new(CAMERA_WIDTH, CAMERA_HEIGHT);
    camera.set_follows(new_entity);
    level.resources.insert(camera);

    let new_pt = {
        let mut map = level.resources.get_mut::<Map>().unwrap();
        let pt = map.locations.get(&location).unwrap().clone();
        map.add_actor_at_xy(pt.x, pt.y, new_entity, true);
        pt
    };
    {
        let mut entry = level.world.entry(new_entity).unwrap();
        let pos = entry.get_component_mut::<Position>().unwrap();
        pos.set(new_pt.x, new_pt.y);
    }
    {
        let map = level.resources.get::<Map>().unwrap();
        if let Some(ref welcome) = map.welcome {
            level.logger.log(welcome);
        }
    }
    true
}
