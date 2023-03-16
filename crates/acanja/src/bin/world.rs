use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_world::camera::Camera;
use gw_world::level::Level;
use gw_world::map::Map;
use gw_world::memory::MapMemory;
use gw_world::tile::{TileTomlFileLoader, Tiles};
use gw_world::widget::Viewport;

const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 50;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT")
            .size(MAP_WIDTH, MAP_HEIGHT)
            .build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_level(&self, ecs: &mut Ecs) {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, MAP_WIDTH, MAP_HEIGHT)
        };

        map.reveal_all();
        map.make_fully_visible();

        let mut level = Level::new("WORLD");

        level.resources.insert(map);
        level.resources.insert(MapMemory::new(160, 100));

        ecs.resources.insert(level);
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
            Box::new(TileTomlFileLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
