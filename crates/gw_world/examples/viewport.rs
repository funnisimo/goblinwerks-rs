use gw_app::ecs::{Read, ResourceSet, Write};
use gw_app::*;
use gw_util::point::Point;
use gw_world::map::{dig_room_level, dump_map, Map};
use gw_world::memory::MapMemory;
use gw_world::tile::Tiles;
use gw_world::widget::{Camera, Viewport};

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
        resources.insert(Camera::new(80, 50));
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    let new_map = {
                        let tiles = ecs.resources.get::<Tiles>().unwrap();
                        dig_room_level(&tiles, 80, 50)
                    };
                    ecs.resources.insert(new_map);
                }
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        log("Camera down");
                        camera.move_center(0, 1);
                    }
                }
                VirtualKeyCode::Left => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.move_center(-1, 0);
                    }
                }
                VirtualKeyCode::Up => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.move_center(0, -1);
                    }
                }
                VirtualKeyCode::Right => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.move_center(1, 0);
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

    fn message(&mut self, _app: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
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
        {
            let (mut map, camera) = <(Write<Map>, Read<Camera>)>::fetch_mut(&mut app.resources);
            let offset = camera.offset_for(self.viewport.size());
            self.viewport.draw_map(&mut map, None, offset, false);
        }
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Map Viewport Example")
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
