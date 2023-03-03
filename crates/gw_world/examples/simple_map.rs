use gw_app::*;
// use gw_ui::css::{load_stylesheet_data, STYLES};
use gw_ui::ui::*;
use gw_world::map::{dig_room_level, dump_map, dump_map_with};
use gw_world::memory::MapMemory;
use gw_world::tile::Tiles;
use gw_world::ui::ViewPort;

struct MainScreen {
    ui: UI,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let ui = page((80, 50), "DEFAULT", |body| {
            body.bind_key(VirtualKeyCode::Escape, UiAction::close_app());
            body.bind_key(VirtualKeyCode::Space, UiAction::message("NEW_MAP", None));

            ViewPort::new(body, |vp| {
                vp.size(80, 50);
            });
        });

        ui.dump();

        Box::new(MainScreen { ui })
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
        dump_map_with(&map, |map, x, y| {
            if map.needs_snapshot_xy(x, y) {
                '#'
            } else {
                ' '
            }
        });

        resources.insert(tiles);
        resources.insert(map);
        resources.insert(MapMemory::new(80, 50));
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
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
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: String, _value: Option<MsgData>) -> ScreenResult {
        log(format!("message - {}", id));
        match id.as_str() {
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.ui.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Map Viewport Example")
        // .file(
        //     "resources/styles.css",
        //     Box::new(|path: &str, data: Vec<u8>, app: &mut Ecs| {
        //         let r = load_stylesheet_data(path, data, app);
        //         if r.is_ok() {
        //             STYLES.lock().unwrap().dump();
        //         }
        //         r
        //     }),
        // )
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
