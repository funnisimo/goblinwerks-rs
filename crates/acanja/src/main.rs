use gw_app::*;
use gw_util::point::Point;
use gw_world::map::{dig_room_level, dump_map};
use gw_world::memory::MapMemory;
use gw_world::tile::{TileFileLoader, Tiles};
use gw_world::widget::Viewport;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(80, 50).build();

        Box::new(MainScreen { viewport })
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, app: &mut Ecs) {
        let resources = &mut app.resources;

        let tiles = Tiles::default();
        let mut map = dig_room_level(&tiles, 80, 50);
        map.reveal_all();
        map.make_fully_visible();

        dump_map(&map);

        resources.insert(tiles);
        resources.insert(map);
        resources.insert(MapMemory::new(80, 50));
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(app, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    let new_map = {
                        let tiles = app.resources.get::<Tiles>().unwrap();
                        dig_room_level(&tiles, 80, 50)
                    };
                    app.resources.insert(new_map);
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

    fn message(&mut self, _app: &mut Ecs, id: String, value: Option<Value>) -> ScreenResult {
        match id.as_str() {
            "VIEWPORT_MOVE" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                log(format!("Mouse Pos = {}", pt));
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
        .file("assets/tiles.toml", Box::new(TileFileLoader::new()))
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
