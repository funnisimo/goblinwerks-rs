use gw_app::*;
use gw_ecs::Ecs;
use gw_util::point::Point;
use gw_world::memory::MapMemory;
use gw_world::tile::{Tiles, TilesLoader};
use gw_world::widget::Viewport;
use map::prefab::{PrefabFileLoader, Prefabs};
use map::town::build_town_map;

mod map;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(80, 50).build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_level(&self, ecs: &mut Ecs) {
        let mut map = {
            let tiles = ecs.read_global::<Tiles>();
            let prefabs = ecs.read_global::<Prefabs>();

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_town_map(&tiles, &prefabs, 80, 50, "TOWN")
        };

        map.reveal_all();
        map.make_fully_visible();

        let level = ecs.create_world("WORLD");
        level.insert_resource(map);
        level.insert_resource(MapMemory::new(80, 50));
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
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Acanja")
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
