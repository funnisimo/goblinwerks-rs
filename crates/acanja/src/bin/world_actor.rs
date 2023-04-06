use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::color::named::WHITE;
use gw_app::ecs::*;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_util::xy::Lock;
use gw_world::action::move_step::MoveStepAction;
use gw_world::ai::Actor;
use gw_world::being::Being;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::hero::Hero;
use gw_world::level::{get_current_level_mut, Level};
use gw_world::map::Map;
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::{do_next_action, DoNextActionResult};
use gw_world::tile::{Tiles, TilesLoader};
use gw_world::widget::Viewport;

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
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();
        map.lock = Lock::XY;

        let mut level = Level::new("WORLD");

        level.resources.insert(map);

        // add position + sprite for actor
        let entity = level.world.push((
            Position::new(80, 50),
            Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()),
            UserControl, // Do we need this?
            Being::new("HERO".to_string()),
            Actor::new("USER_CONTROL".to_string()),
        ));

        let mut camera = Camera::new(80, 50);
        camera.set_follows(entity);
        level.resources.insert(camera);

        level.resources.insert(Hero::new(entity));
        level.reset_tasks();

        ecs.resources.insert(level);
    }

    fn post_action(&mut self, ecs: &mut Ecs) {
        // Post Update
        let mut level = get_current_level_mut(ecs);
        update_camera_follows(&mut *level);
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        resources.get_or_insert_with(|| Tiles::default());
        resources.get_or_insert_with(|| Prefabs::default());

        self.build_new_level(ecs);
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
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let level = ecs.resources.get::<Level>().unwrap();
                    let map_size = level.resources.get::<Map>().unwrap().size();
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
            let res = do_next_action(ecs);
            self.post_action(ecs);
            match res {
                DoNextActionResult::Done => {
                    return ScreenResult::Continue;
                }
                DoNextActionResult::Mob | DoNextActionResult::Other => {
                    continue;
                }
                DoNextActionResult::Hero => {
                    return ScreenResult::Continue;
                }
                DoNextActionResult::PushMode(mode) => return ScreenResult::Push(mode),
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
            Box::new(TilesLoader::new().with_dump()),
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
