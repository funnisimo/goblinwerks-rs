use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::town::build_town_map;
use acanja::map::world::build_world_map;
use gw_app::ecs::*;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_world::camera::Camera;
use gw_world::effect::BoxedEffect;
use gw_world::level::{Level, Levels};
use gw_world::map::{cell_flavor, Cell, Map};
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

    fn build_new_world(&mut self, ecs: &mut Ecs) -> Level {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();

        // Need to add towns

        let mut level = Level::new("WORLD");

        level.resources.insert(map);
        level
    }

    fn build_new_town(&mut self, ecs: &mut Ecs, idx: u8) -> Level {
        let town_name = format!("TOWN{}", idx);

        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_town_map(&tiles, &prefabs, 80, 50, &town_name)
        };

        map.reveal_all();
        map.make_fully_visible();

        let mut level = Level::new(&town_name);

        level.resources.insert(map);
        level.resources.insert(Camera::new(80, 50));
        level
    }

    fn build_new_levels(&mut self, ecs: &mut Ecs) {
        let mut levels = Levels::new();

        levels.insert(self.build_new_world(ecs));

        levels.insert(self.build_new_town(ecs, 1));
        levels.insert(self.build_new_town(ecs, 2));
        levels.insert(self.build_new_town(ecs, 3));
        levels.insert(self.build_new_town(ecs, 4));

        log(format!(
            "Built 4 town maps - total levels = {}",
            levels.len()
        ));

        ecs.resources.insert(levels);
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
        resources.insert(Levels::new());

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
                    // zoom in
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let level = levels.current_mut();
                    let mut camera = level.resources.get_mut::<Camera>().unwrap();
                    let size = *camera.size();
                    camera.resize((size.0 - 8).max(16), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    // zoom out
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let level = levels.current_mut();
                    let (map, mut camera) =
                        <(Read<Map>, Write<Camera>)>::fetch_mut(&mut level.resources);
                    let map_size = map.size();
                    let size = *camera.size();
                    camera.resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Return => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let idx = levels.current_id();
                    let current_idx = levels.iter().position(|level| level.id == idx).unwrap();

                    let next_id = match levels.iter().skip(current_idx + 1).next() {
                        None => levels.iter().next().unwrap().id.clone(),
                        Some(level) => level.id.clone(),
                    };
                    levels.set_current(&next_id).unwrap();
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
                let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                let level = levels.current_mut();
                let map = level.resources.get::<Map>().unwrap();
                let index = map.get_wrapped_index(pt.x, pt.y).unwrap();
                log(format!(
                    "Mouse Pos = {} - {}",
                    pt,
                    cell_flavor(&*map, &mut level.world, index)
                ));
            }
            "VIEWPORT_CLICK" => {
                let pos: Point = value.unwrap().try_into().unwrap();
                match try_pos_action(ecs, pos, "descend") {
                    false => {
                        try_pos_action(ecs, pos, "climb");
                    }
                    true => {}
                }
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        {
            let mut levels = app.resources.get_mut::<Levels>().unwrap();
            let level = levels.current_mut();
            self.viewport.draw_level(level);
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

fn move_camera(levels: &mut Levels, dx: i32, dy: i32) {
    let level = levels.current_mut();
    let mut camera = level.resources.get_mut::<Camera>().unwrap();
    camera.move_center(dx, dy);
}

fn get_pos_action_effects(ecs: &mut Ecs, pos: &Point, action: &str) -> Option<Vec<BoxedEffect>> {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();

    let map = level.resources.get::<Map>().unwrap();

    let index = map.get_index(pos.x, pos.y).unwrap();

    match map.cell_effects.get(&index) {
        None => None,
        Some(effect_map) => match effect_map.get(action) {
            None => None,
            Some(effects) => Some(effects.clone()),
        },
    }
}

fn try_pos_action(ecs: &mut Ecs, pos: Point, action: &str) -> bool {
    match get_pos_action_effects(ecs, &pos, action) {
        None => false,
        Some(effects) => {
            for eff in effects.iter() {
                eff.fire(ecs, pos, None);
            }
            true
        }
    }
}
