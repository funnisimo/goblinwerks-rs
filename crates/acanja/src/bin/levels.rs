use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::town::build_town_map;
use acanja::map::world::build_world_map;
use gw_app::ecs::*;
use gw_app::*;
use gw_util::point::Point;
use gw_world::camera::Camera;
use gw_world::effect::BoxedEffect;
use gw_world::level::NeedsDraw;
use gw_world::log::Logger;
use gw_world::map::{cell_flavor, Map};
use gw_world::task::{Executor, UserAction};
use gw_world::tile::{Tiles, TilesLoader};
use gw_world::widget::Viewport;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(160, 100).build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_world<'a>(&mut self, ecs: &'a mut Ecs) -> &'a mut World {
        let mut map = {
            let tiles = ecs.read_global::<Tiles>();
            let prefabs = ecs.read_global::<Prefabs>();

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();

        // Need to add towns

        let level = ecs.create_world("WORLD");
        level.insert_resource(map);
        level.insert_resource(Camera::new(80, 50));
        level.insert_resource(Executor::new());
        level.insert_resource(NeedsDraw::default());
        level.insert_resource(UserAction::default());
        level.insert_resource(Logger::default());

        level
    }

    fn build_new_town<'a>(&mut self, ecs: &'a mut Ecs, idx: u8) -> &'a mut World {
        let town_name = format!("TOWN{}", idx);

        let mut map = {
            let tiles = ecs.read_global::<Tiles>();
            let prefabs = ecs.read_global::<Prefabs>();

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_town_map(&tiles, &prefabs, 80, 50, &town_name)
        };

        map.reveal_all();
        map.make_fully_visible();

        let level = ecs.create_world(town_name.as_str());

        level.insert_resource(map);
        level.insert_resource(Camera::new(80, 50));
        level.insert_resource(Executor::new());
        level.insert_resource(NeedsDraw::default());
        level.insert_resource(UserAction::default());
        level.insert_resource(Logger::default());
        level
    }

    fn build_new_levels(&mut self, ecs: &mut Ecs) {
        self.build_new_world(ecs);

        self.build_new_town(ecs, 1);
        self.build_new_town(ecs, 2);
        self.build_new_town(ecs, 3);
        self.build_new_town(ecs, 4);

        ecs.set_current_world("WORLD").unwrap();

        log(format!("Built 4 town maps - total levels = {}", ecs.len()));
    }

    #[allow(dead_code)]
    fn post_action(&mut self, _level: &mut World) {
        // Post Update
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        ecs.ensure_global::<Tiles>();
        ecs.ensure_global::<Prefabs>();

        self.build_new_levels(ecs);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs.current_world_mut(), ev) {
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
                    move_camera(ecs.current_world_mut(), 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_camera(ecs.current_world_mut(), -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_camera(ecs.current_world_mut(), 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_camera(ecs.current_world_mut(), 1, 0);
                }
                VirtualKeyCode::Equals => {
                    // zoom in
                    let level = ecs.current_world();
                    let mut camera = level.write_resource::<Camera>();
                    let size = *camera.size();
                    camera.resize((size.0 - 8).max(16), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    // zoom out
                    let level = ecs.current_world_mut();
                    let (map, mut camera) = <(ReadRes<Map>, WriteRes<Camera>)>::fetch(level);
                    let map_size = map.size();
                    let size = *camera.size();
                    camera.resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Return => {
                    let idx = ecs.current_world().id();
                    let current_idx = ecs
                        .iter_worlds()
                        .position(|level| level.id() == idx)
                        .unwrap();

                    let next_id = match ecs.iter_worlds().skip(current_idx + 1).next() {
                        None => ecs.iter_worlds().next().unwrap().id().clone(),
                        Some(level) => level.id().clone(),
                    };
                    ecs.set_current_world(next_id).unwrap();
                    ecs.write_resource::<NeedsDraw>().set();
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

    fn message(&mut self, ecs: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "VIEWPORT_MOVE" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                let map = ecs.read_resource::<Map>();
                let index = map.get_wrapped_index(pt.x, pt.y).unwrap();
                log(format!(
                    "Mouse Pos = {} - {}",
                    pt,
                    cell_flavor(&*map, ecs.current_world(), index)
                ));
            }
            "VIEWPORT_CLICK" => {
                let pos: Point = value.unwrap().try_into().unwrap();
                match try_pos_action(ecs.current_world_mut(), pos, "descend") {
                    false => {
                        try_pos_action(ecs.current_world_mut(), pos, "climb");
                    }
                    true => {}
                }
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
        })
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_camera(level: &mut World, dx: i32, dy: i32) {
    let mut camera = level.write_resource::<Camera>();
    camera.move_center(dx, dy);
}

fn get_pos_action_effects(
    level: &mut World,
    pos: &Point,
    action: &str,
) -> Option<Vec<BoxedEffect>> {
    let map = level.read_resource::<Map>();

    let index = map.get_index(pos.x, pos.y).unwrap();

    match map.cell_effects.get(&index) {
        None => None,
        Some(effect_map) => match effect_map.get(action) {
            None => None,
            Some(effects) => Some(effects.clone()),
        },
    }
}

fn try_pos_action(world: &mut World, pos: Point, action: &str) -> bool {
    match get_pos_action_effects(world, &pos, action) {
        None => false,
        Some(effects) => {
            for eff in effects.iter() {
                eff.fire(world, pos, None);
            }
            true
        }
    }
}
