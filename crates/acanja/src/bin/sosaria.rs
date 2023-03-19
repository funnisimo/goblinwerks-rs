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

const CAMERA_WIDTH: u32 = 1024 / 32;
const CAMERA_HEIGHT: u32 = 768 / 32;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT")
            .size(11, 11)
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

        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    move_hero(&mut *level, 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_hero(&mut *level, -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_hero(&mut *level, 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_hero(&mut *level, 1, 0);
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

fn move_hero(level: &mut Level, dx: i32, dy: i32) {
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let mut entry = level.world.entry(hero_entity).unwrap();
    let actor = entry.get_component_mut::<Actor>().unwrap();
    actor.next_action = Some(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}
