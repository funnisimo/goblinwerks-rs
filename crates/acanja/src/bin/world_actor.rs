use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::color::named::WHITE;
use gw_app::*;
use gw_ecs::*;
use gw_util::point::Point;
use gw_util::xy::Lock;
use gw_world::action::move_step::MoveStepAction;
use gw_world::being::Being;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::hero::Hero;
use gw_world::level::NeedsDraw;
use gw_world::log::Logger;
use gw_world::map::Map;
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::{do_next_task, DoNextTaskResult, Executor, Task, UserAction};
use gw_world::tile::{Tiles, TilesLoader};
use gw_world::widget::Viewport;

#[derive(Component)]
struct UserControl;

struct MainScreen {
    viewport: Viewport,
    // executor: Executor,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(80, 50).build();

        Box::new(MainScreen {
            viewport,
            // executor: Executor::new(),
        })
    }

    fn build_new_level(&mut self, ecs: &mut Ecs) {
        let mut map = {
            let tiles = ecs.read_global::<Tiles>();
            let prefabs = ecs.read_global::<Prefabs>();

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();
        map.lock = Lock::XY;

        let level = ecs.create_world("WORLD");

        level.insert_resource(map);

        // add position + sprite for actor
        let entity = level
            .create_entity()
            .with(Position::new(80, 50))
            .with(Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()))
            .with(UserControl)
            .with(
                // Do we need this?
                Being::new("HERO".to_string()),
            )
            .with(Task::new("USER_CONTROL".to_string()))
            .build();

        let mut camera = Camera::new(80, 50);
        camera.set_follows(entity);
        level.insert_resource(camera);

        level.insert_resource(Hero::new(entity));
        level.insert_resource(Executor::new());
        level.insert_resource(NeedsDraw::default());
        level.insert_resource(UserAction::default());
        level.insert_resource(Logger::default());
        level.write_resource::<Executor>().insert(entity, 0);
    }

    fn post_action(&mut self, ecs: &mut Ecs) {
        // Post Update
        let level = ecs.current_world_mut();
        update_camera_follows(level);
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        ecs.ensure_global::<Tiles>();
        ecs.ensure_global::<Prefabs>();

        self.build_new_level(ecs);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs.current_world_mut(), ev) {
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
                    move_hero(ecs.current_world_mut(), 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_hero(ecs.current_world_mut(), -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_hero(ecs.current_world_mut(), 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_hero(ecs.current_world_mut(), 1, 0);
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let map_size = ecs.read_resource::<Map>().size();
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

        // let mut level = ecs.resources.get_mut::<Level>().unwrap();

        // level.execute(|level, executor| {
        // Update
        loop {
            // if world.is_game_over() {
            //     return (self.game_over)(world, ctx);
            // } else if !world.animations().is_empty() {
            //     return ScreenResult::Continue;
            // }
            // let res = executor.do_next_action(&mut *level);
            let res = do_next_task(ecs.current_world_mut());
            self.post_action(ecs);
            match res {
                DoNextTaskResult::Done => {
                    return ScreenResult::Continue;
                }
                DoNextTaskResult::Other => {
                    continue;
                }
                DoNextTaskResult::Hero => {
                    return ScreenResult::Continue;
                }
                DoNextTaskResult::PushMode(mode) => return ScreenResult::Push(mode),
            }
        }
        // })
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
        self.viewport.draw_level(app.current_world_mut());
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Acanja - World Viewer")
        .file(
            "assets/tiles.toml",
            Box::new(TilesLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .register_components(|ecs| {
            gw_world::register_components(ecs);
            ecs.register::<UserControl>();
        })
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_hero(world: &mut World, dx: i32, dy: i32) {
    let hero_entity = world.read_resource::<Hero>().entity;

    let mut user_action = world.write_resource::<UserAction>();
    user_action.set(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}
