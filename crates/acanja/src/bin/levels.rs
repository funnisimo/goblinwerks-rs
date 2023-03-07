use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::ecs::*;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_world::level::{Level, Levels};
use gw_world::log::Logger;
use gw_world::map::Map;
use gw_world::memory::MapMemory;
use gw_world::tile::{TileFileLoader, Tiles};
use gw_world::widget::{Camera, Viewport};

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(160, 100).build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_world(&mut self, ecs: &mut Ecs) -> Level {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();

        let mut level = Level::new("WORLD");

        level.resources.insert(map);
        level.resources.insert(MapMemory::new(160, 100));
        level.resources.insert(Logger::new());

        level
    }

    fn build_new_levels(&mut self, ecs: &mut Ecs) {
        let mut levels = Levels::new();

        let index = levels.push(self.build_new_world(ecs));
        levels.set_current_index(index);
    }

    #[allow(dead_code)]
    fn post_action(&mut self, _level: &mut Level) {
        // Post Update
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        resources.get_or_insert_with(|| Tiles::default());
        resources.get_or_insert_with(|| Prefabs::default());
        resources.get_or_insert_with(|| Logger::new());

        self.build_new_levels(ecs);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    self.build_new_levels(ecs);
                }
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    move_camera(&mut *levels, 0, 1);
                }
                VirtualKeyCode::Left => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    move_camera(&mut *levels, -1, 0);
                }
                VirtualKeyCode::Up => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    move_camera(&mut *levels, 0, -1);
                }
                VirtualKeyCode::Right => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    move_camera(&mut *levels, 1, 0);
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let levels = ecs.resources.get::<Levels>().unwrap();
                    let level = levels.current();
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

    fn update(&mut self, _ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

        // Update

        // post update

        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: String, value: Option<Value>) -> ScreenResult {
        match id.as_str() {
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
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_camera(levels: &mut Levels, dx: i32, dy: i32) {
    let level = levels.current_mut();
    let mut camera = level.resources.get_mut::<Camera>().unwrap();
    camera.center.x = camera.center.x + dx;
    camera.center.y = camera.center.y + dy;
}
