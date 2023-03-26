use acanja::map::prefab::{PrefabFileLoader, Prefabs};
use acanja::map::town::build_town_map;
use acanja::map::world::build_world_map;
use gw_app::ecs::*;
use gw_app::ecs::{systems::ResourceSet, Deserialize, Read, Serialize};
use gw_app::*;
use gw_util::point::Point;
use gw_world::action::move_step::MoveStepAction;
use gw_world::actor::Actor;
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::effect::BoxedEffect;
use gw_world::hero::Hero;
use gw_world::level::{Level, Levels};
use gw_world::map::{Cell, Map};
use gw_world::position::Position;
use gw_world::sprite::Sprite;
use gw_world::task::{do_next_action, DoNextActionResult};
use gw_world::tile::{Tiles, TilesLoader};
use gw_world::widget::Viewport;

#[derive(Serialize, Deserialize)]
struct UserControl;

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
            build_world_map(&tiles, &prefabs, 160, 100)
        };

        map.reveal_all();
        map.make_fully_visible();
        let start_index = map.locations.get("START").unwrap().clone();
        let start_loc = map.to_point(start_index);

        log(format!("locations = {:?}", &map.locations));

        let mut level = Level::new("WORLD");

        level.resources.insert(map);

        // add position + sprite for actor
        let entity = level.world.push((
            Position::new(start_loc.x, start_loc.y),
            Sprite::new('@' as Glyph, WHITE.into(), RGBA::new()),
            UserControl, // Do we need this?
            Actor::new().with_ai("USER_CONTROL"),
        ));

        let mut camera = Camera::new(80, 50);
        camera.set_follows(entity);
        level.resources.insert(camera);

        level.resources.insert(Hero::new(entity));
        level.reset_tasks();

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
    fn post_action(&mut self, ecs: &mut Ecs) {
        // Post Update
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();
        update_camera_follows(level);
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
                    let level = levels.current_mut();
                    move_hero(level, 0, 1);
                }
                VirtualKeyCode::Left => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let level = levels.current_mut();
                    move_hero(level, -1, 0);
                }
                VirtualKeyCode::Up => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let level = levels.current_mut();
                    move_hero(level, 0, -1);
                }
                VirtualKeyCode::Right => {
                    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                    let level = levels.current_mut();
                    move_hero(level, 1, 0);
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
            AppEvent::CharEvent(ch) => match ch {
                '<' => {
                    // Climb
                    try_fire_hero_action(ecs, "climb");
                    // let hero_point = get_hero_point(ecs);
                    // try_move_hero_world(ecs, &hero_point, PortalFlags::ON_CLIMB);
                }
                '>' => {
                    // Descend
                    try_fire_hero_action(ecs, "descend");
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
            let res = do_next_action(ecs);
            self.post_action(ecs);
            match res {
                DoNextActionResult::Done => {
                    return ScreenResult::Continue;
                }
                DoNextActionResult::Mob => {
                    continue;
                }
                DoNextActionResult::Hero => {
                    return ScreenResult::Continue;
                }
                DoNextActionResult::PushMode(mode) => return ScreenResult::Push(mode),
            }
        }
        // })
        // post update
    }

    fn message(&mut self, ecs: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "VIEWPORT_MOVE" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                let levels = ecs.resources.get::<Levels>().unwrap();
                let level = levels.current();
                let map = level.resources.get::<Map>().unwrap();
                let index = map.get_wrapped_index(pt.x, pt.y).unwrap();
                let cell = map.get_cell(index).unwrap();
                log(format!("Mouse Pos = {} - {}", pt, cell.flavor()));
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
        .register_components(|registry| {
            registry.register::<gw_world::position::Position>("Position".to_string());
            registry.register::<gw_world::sprite::Sprite>("Sprite".to_string());
            registry.register::<gw_world::actor::Actor>("Actor".to_string());
            registry.register::<UserControl>("UserControl".to_string());
        })
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

#[allow(dead_code)]
fn move_camera(levels: &mut Levels, dx: i32, dy: i32) {
    let level = levels.current_mut();
    let mut camera = level.resources.get_mut::<Camera>().unwrap();
    camera.move_center(dx, dy);
}

fn move_hero(level: &mut Level, dx: i32, dy: i32) {
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let mut entry = level.world.entry(hero_entity).unwrap();
    let actor = entry.get_component_mut::<Actor>().unwrap();
    actor.next_action = Some(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}

fn get_hero_action_effects(
    ecs: &mut Ecs,
    action: &str,
) -> Option<(Entity, Point, Vec<BoxedEffect>)> {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let hero_point = level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point();

    let map = level.resources.get::<Map>().unwrap();

    let index = map.get_index(hero_point.x, hero_point.y).unwrap();

    match map.cell_effects.get(&index) {
        None => None,
        Some(effect_map) => match effect_map.get(action) {
            None => None,
            Some(effects) => Some((hero_entity, hero_point, effects.clone())),
        },
    }
}

fn try_fire_hero_action(ecs: &mut Ecs, action: &str) -> bool {
    match get_hero_action_effects(ecs, action) {
        None => false,
        Some((entity, pos, effects)) => {
            for eff in effects.iter() {
                eff.fire(ecs, pos, Some(entity));
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
