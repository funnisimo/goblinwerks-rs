use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::town::build_town_map;
use acanja::map::world::build_world_map;
use gw_app::ecs::*;
use gw_app::*;
use gw_util::point::Point;
use gw_world::action::move_step::MoveStepAction;
use gw_world::being::Being;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::effect::BoxedEffect;
use gw_world::hero::Hero;
use gw_world::level::NeedsDraw;
use gw_world::log::Logger;
use gw_world::map::{cell_flavor, Map};
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::{do_next_task, DoNextTaskResult, Executor, Task, UserAction};
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
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();
        let start_index = map.locations.get("START").unwrap().clone();
        let start_loc = map.to_point(start_index);

        log(format!("locations = {:?}", &map.locations));

        let level = ecs.create_world("WORLD");

        level.insert_resource(map);

        // add position + sprite for actor
        let entity = level
            .create_entity()
            .with(Position::new(start_loc.x, start_loc.y))
            .with(Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()))
            .with(Being::new("HERO".to_string()))
            .with(Task::new("USER_CONTROL"))
            .build();

        level.ensure_resource::<Executor>();
        level.write_resource::<Executor>().insert(entity, 0);

        let mut camera = Camera::new(80, 50);
        camera.set_follows(entity);
        level.insert_resource(camera);

        level.insert_resource(Hero::new(entity));
        level.insert_resource(NeedsDraw::default());
        level.insert_resource(UserAction::default());
        level.insert_resource(Logger::default());

        // log(format!("CAMERA = {:?}", *level.read_resource::<Camera>()));

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

        log(format!("Built 4 town maps - total levels = {}", ecs.len()));

        ecs.set_current_world("WORLD").unwrap();
    }

    #[allow(dead_code)]
    fn post_action(&mut self, ecs: &mut Ecs) {
        // Post Update
        update_camera_follows(ecs.current_world_mut());
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
                    let level = ecs.current_world_mut();
                    move_hero(level, 0, 1);
                }
                VirtualKeyCode::Left => {
                    let level = ecs.current_world_mut();
                    move_hero(level, -1, 0);
                }
                VirtualKeyCode::Up => {
                    let level = ecs.current_world_mut();
                    move_hero(level, 0, -1);
                }
                VirtualKeyCode::Right => {
                    let level = ecs.current_world_mut();
                    move_hero(level, 1, 0);
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let level = ecs.current_world();
                    let map_size = level.read_resource::<Map>().size();
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                    drop(level);
                }
                _ => {}
            },
            AppEvent::CharEvent(ch) => match ch {
                '<' => {
                    // Climb
                    try_fire_hero_action(ecs.current_world_mut(), "climb");
                    // let hero_point = get_hero_point(ecs);
                    // try_move_hero_world(ecs, &hero_point, PortalFlags::ON_CLIMB);
                }
                '>' => {
                    // Descend
                    try_fire_hero_action(ecs.current_world_mut(), "descend");
                    // let hero_point = get_hero_point(ecs);
                    // try_move_hero_world(ecs, &hero_point, PortalFlags::ON_DESCEND);
                }
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

        // Update
        // let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        // let level = levels.current_mut();

        // level.execute(|level, executor| {
        // Update
        loop {
            // if world.is_game_over() {
            //     return (self.game_over)(world, ctx);
            // } else if !world.animations().is_empty() {
            //     return ScreenResult::Continue;
            // }
            let res = do_next_task(ecs.current_world_mut());
            self.post_action(ecs);
            match res {
                DoNextTaskResult::Done => {
                    return ScreenResult::Continue;
                }
                DoNextTaskResult::Other => {
                    continue;
                }
                DoNextTaskResult::Hero => {
                    return ScreenResult::Continue;
                }
                DoNextTaskResult::PushMode(mode) => return ScreenResult::Push(mode),
            }
        }
        // })
        // post update
    }

    fn message(&mut self, ecs: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "VIEWPORT_MOVE" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                let level = ecs.current_world();
                let map = level.read_resource::<Map>();
                let index = map.get_wrapped_index(pt.x, pt.y).unwrap();
                log(format!(
                    "Mouse Pos = {} - {}",
                    pt,
                    cell_flavor(&*map, ecs.current_world(), index)
                ));
            }
            "VIEWPORT_CLICK" => {
                // let pos: Point = value.unwrap().try_into().unwrap();
                // match try_pos_action(ecs, pos, "descend") {
                //     false => {
                //         try_pos_action(ecs, pos, "climb");
                //     }
                //     true => {}
                // }
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.viewport.draw_level(app.current_world_mut());
        self.viewport.render(app);

        // TODO - This should be in APP
        app.maintain();
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

#[allow(dead_code)]
fn move_camera(level: &mut World, dx: i32, dy: i32) {
    let mut camera = level.write_resource::<Camera>();
    camera.move_center(dx, dy);
}

fn move_hero(level: &mut World, dx: i32, dy: i32) {
    let hero_entity = level.read_resource::<Hero>().entity;

    let mut user_action = level.write_resource::<UserAction>();
    user_action.set(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}

fn get_hero_action_effects(
    level: &mut World,
    action: &str,
) -> Option<(Entity, Point, Vec<BoxedEffect>)> {
    let hero_entity = level.read_resource::<Hero>().entity;

    let hero_point = level
        .read_component::<Position>()
        .get(hero_entity)
        .unwrap()
        .point();

    let map = level.read_resource::<Map>();

    let index = map.get_index(hero_point.x, hero_point.y).unwrap();

    match map.cell_effects.get(&index) {
        None => None,
        Some(effect_map) => match effect_map.get(action) {
            None => None,
            Some(effects) => Some((hero_entity, hero_point, effects.clone())),
        },
    }
}

fn try_fire_hero_action(world: &mut World, action: &str) -> bool {
    match get_hero_action_effects(world, action) {
        None => false,
        Some((entity, pos, effects)) => {
            log(format!("Firing hero actions - {:?}", effects));
            for eff in effects.iter() {
                eff.fire(world, pos, Some(entity));
            }
            true
        }
    }
}

// fn get_hero_point(ecs: &mut Ecs) -> Point {
//     let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
//     let level = levels.current_mut();
//     let hero_entity = level.resources.get::<Hero>().unwrap().entity;

//     level
//         .world
//         .entry(hero_entity)
//         .unwrap()
//         .get_component::<Position>()
//         .unwrap()
//         .point()
// }

// fn try_move_hero_world(ecs: &mut Ecs, pt: &Point, flag: PortalFlags) -> bool {
//     let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
//     let level = levels.current_mut();

//     let hero_entity = level.resources.get::<Hero>().unwrap().entity;

//     let map = level.resources.get_mut::<Map>().unwrap();
//     let index = map.get_wrapped_index(pt.x, pt.y).unwrap();

//     log(format!("CLICK = {:?}", pt));

//     let (new_map_id, location) = {
//         match map.get_portal(index) {
//             None => return false,
//             Some(info) => {
//                 if !info.flags().contains(flag) {
//                     return false;
//                 }

//                 log(format!(
//                     "Enter Portal = {} - {}::{}",
//                     info.flavor().as_ref().unwrap(),
//                     info.map_id(),
//                     info.location()
//                 ));

//                 (info.map_id().to_string(), info.location().to_string())
//             }
//         }
//     };

//     let current_pt = level
//         .world
//         .entry(hero_entity)
//         .unwrap()
//         .get_component::<Position>()
//         .unwrap()
//         .point();

//     drop(map);
//     drop(level);

//     let level = levels.current_mut();
//     let mut map = level.resources.get_mut::<Map>().unwrap();
//     let current_idx = map.get_wrapped_index(current_pt.x, current_pt.y).unwrap();

//     map.remove_actor(current_idx, hero_entity);

//     drop(map);
//     drop(level);

//     log("Moving hero to new world");
//     let new_entity = levels.move_current_entity(hero_entity, &new_map_id);
//     log("Changing current world");
//     if levels.set_current(&new_map_id).is_err() {
//         panic!("Failed to change world - {}", new_map_id);
//     }

//     let level = levels.current_mut();
//     level.resources.insert(Hero::new(hero_entity));
//     let new_pt = {
//         let mut map = level.resources.get_mut::<Map>().unwrap();
//         let pt = map.locations.get(&location).unwrap().clone();
//         map.add_actor(pt, new_entity, true);
//         map.to_point(pt)
//     };
//     {
//         let mut entry = level.world.entry(new_entity).unwrap();
//         let pos = entry.get_component_mut::<Position>().unwrap();
//         pos.set(new_pt.x, new_pt.y);
//     }
//     true
// }
