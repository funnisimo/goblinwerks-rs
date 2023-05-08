use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::*;
use gw_util::point::Point;
use gw_world::being::BeingKinds;
use gw_world::camera::Camera;
use gw_world::level::NeedsDraw;
use gw_world::map::Map;
use gw_world::memory::MapMemory;
use gw_world::tile::{Tiles, TilesLoader};
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
            let tiles = ecs.read_global::<Tiles>();
            let prefabs = ecs.read_global::<Prefabs>();

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, MAP_WIDTH, MAP_HEIGHT)
        };

        map.reveal_all();
        map.make_fully_visible();

        // TODO - ecs.delete_world("WORLD");
        let level = ecs.create_world("WORLD");
        level.insert_resource(map);
        level.insert_resource(MapMemory::new(160, 100));
        level.insert_resource(NeedsDraw::default());
        ecs.set_current_world("WORLD").unwrap();
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
                    let mut camera = ecs.write_resource::<Camera>();
                    log("Camera down");
                    camera.move_center(0, 1);
                }
                VirtualKeyCode::Left => {
                    let mut camera = ecs.write_resource::<Camera>();
                    camera.move_center(-1, 0);
                }
                VirtualKeyCode::Up => {
                    let mut camera = ecs.write_resource::<Camera>();
                    camera.move_center(0, -1);
                }
                VirtualKeyCode::Right => {
                    let mut camera = ecs.write_resource::<Camera>();
                    camera.move_center(1, 0);
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
            ecs.ensure_global::<Tiles>();
            ecs.ensure_global::<Prefabs>();
            ecs.ensure_global::<BeingKinds>();
        })
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
