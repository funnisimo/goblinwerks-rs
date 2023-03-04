use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_world::map::dump_map;
use gw_world::map::Map;
use gw_world::memory::MapMemory;
use gw_world::tile::{TileFileLoader, Tiles};
use gw_world::widget::{Camera, Viewport};

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(80, 50).build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_map(&self, ecs: &mut Ecs) {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();

        ecs.resources.insert(map);
        ecs.resources.insert(MapMemory::new(160, 100));
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        resources.get_or_insert_with(|| Tiles::default());
        resources.get_or_insert_with(|| Prefabs::default());

        self.build_new_map(ecs);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    self.build_new_map(ecs);
                }
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        log("Camera down");
                        camera.pos.y = camera.pos.y + 1;
                    }
                }
                VirtualKeyCode::Left => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.x = camera.pos.x - 1;
                    }
                }
                VirtualKeyCode::Up => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.y = camera.pos.y - 1;
                    }
                }
                VirtualKeyCode::Right => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.x = camera.pos.x + 1;
                    }
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let map_size = ecs.resources.get::<Map>().unwrap().get_size();
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