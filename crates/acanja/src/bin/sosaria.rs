use acanja::effect::{parse_gremlins, parse_mark, parse_moongate_travel, parse_winds};
use acanja::loader::GameConfigLoader;
use gw_app::ecs::IntoQuery;
use gw_app::ecs::{Entity, Read, ResourceSet, Write};
use gw_app::*;
use gw_util::point::Point;
use gw_world::action::idle::IdleAction;
use gw_world::action::move_step::MoveStepAction;
use gw_world::being::{spawn_being, Being, BeingKinds};
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::effect::{register_effect_parser, BoxedEffect};
use gw_world::fov::update_fov;
use gw_world::hero::Hero;
use gw_world::level::{get_current_level_mut, with_current_level, Levels};
use gw_world::map::{Cell, Map};
use gw_world::position::Position;
use gw_world::task::{do_next_task, DoNextTaskResult, Task, UserAction};
use gw_world::task::{get_hero_entity, register_task};
use gw_world::widget::Viewport;

const CAMERA_WIDTH: u32 = 1024 / 32;
const CAMERA_HEIGHT: u32 = 768 / 32;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT")
            // .size(11, 11)
            .font("assets/font_32x58.png")
            // .extents(0.0, 0.0, 0.85, 0.85)
            // .wrap(Wrap::XY)
            .build();

        Box::new(MainScreen { viewport })
    }

    fn post_action(&mut self, ecs: &mut Ecs) {
        // Post Update

        update_camera_follows(&mut *get_current_level_mut(ecs));
        update_fov(ecs);
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        // resources.get_or_insert_with(|| Tiles::default());
        // resources.get_or_insert_with(|| Prefabs::default());

        let (mut levels, actor_kinds) = <(Write<Levels>, Read<BeingKinds>)>::fetch_mut(resources);

        log(format!("START MAP = {}", levels.current_id()));
        levels.setup();
        let level = levels.current_mut();

        let start_pos = {
            let map = level.resources.get::<Map>().unwrap();
            // dump_map(&*map);
            // log(format!("map size = {:?}", map.get_size()));
            map.to_point(map.get_location("START").unwrap())
        };

        let hero_kind = actor_kinds.get("HERO").unwrap();
        log(format!("HERO - {:?}", hero_kind));

        let entity = spawn_being(&hero_kind, level, start_pos);

        ///////////////////////////////////////////

        // SPAWN MOONGATES...

        ///////////////////////////////////////////

        {
            let mut camera = level
                .resources
                .get_mut_or_insert_with(|| Camera::new(CAMERA_WIDTH, CAMERA_HEIGHT));
            camera.set_follows(entity);
        }

        let moongate = level.world.push((Task::new("MOVE_MOONGATE"),));
        level.executor.insert(moongate, 0);

        // level.reset_tasks();

        log(format!("STARTING TASKS = {:?}", level.executor));
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    move_hero(ecs, 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_hero(ecs, -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_hero(ecs, 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_hero(ecs, 1, 0);
                }
                _ => {}
            },
            AppEvent::CharEvent(ch) => match ch {
                '<' => {
                    // Climb
                    log("CLIMB");
                    try_fire_hero_action(ecs, "climb");
                    // let hero_point = get_hero_point(ecs);
                    // try_move_hero_world(ecs, &hero_point, PortalFlags::ON_CLIMB);
                }
                '>' => {
                    // Descend
                    log("DESCEND");
                    try_fire_hero_action(ecs, "descend");
                    // let hero_point = get_hero_point(ecs);
                    // try_move_hero_world(ecs, &hero_point, PortalFlags::ON_DESCEND);
                }
                '.' | ' ' => {
                    hero_idle(ecs);
                }
                't' => with_current_level(ecs, |level| {
                    println!("TASKS = {:?}", level.executor);
                }),
                'a' => with_current_level(ecs, |level| {
                    let mut query = <&Being>::query();

                    // you can then iterate through the components found in the world
                    for actor in query.iter(&level.world) {
                        println!("{:?}", actor);
                    }
                }),
                'k' => {
                    let kinds = ecs.resources.get::<BeingKinds>().unwrap();
                    kinds.dump();
                }
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn message(&mut self, ecs: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id {
            "VIEWPORT_MOVE" => {
                // let pt: Point = value.unwrap().try_into().unwrap();
                // log(format!("Mouse Pos = {}", pt));
            }
            "VIEWPORT_CLICK" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                log(format!("CLICK = {}", pt));

                let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
                let level = levels.current_mut();
                let map = level.resources.get::<Map>().unwrap();

                let idx = map.get_index(pt.x, pt.y).unwrap();
                let cell = map.get_cell(idx).unwrap();

                log(format!("GROUND = {:?}", cell.ground()));
                log(format!("FIXTURE = {:?}", cell.fixture()));

                if let Some(effects) = map.cell_effects.get(&idx) {
                    log(format!("CELL EFFECTS = {:?}", effects));
                }

                for entity in map.iter_beings(idx) {
                    let entry = level.world.entry(entity).unwrap();
                    let being = entry.get_component::<Being>().unwrap();
                    log(format!("BEING({:?}) = {:?}", entity, being));
                    let actor = entry.get_component::<Being>().unwrap();
                    log(format!("ACTOR({:?}) = {:?}", entity, actor));
                }
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

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
            // let res = executor.do_next_action(&mut *level);
            let res = do_next_task(ecs);
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
    }

    fn render(&mut self, app: &mut Ecs) {
        {
            let mut levels = app.resources.get_mut::<Levels>().unwrap();
            let level = levels.current_mut();
            self.viewport.draw_level(&mut *level);
        }
        self.viewport.render(app);
    }
}

fn main() {
    register_effect_parser("winds", parse_winds);
    register_effect_parser("gremlins", parse_gremlins);
    register_effect_parser("mark", parse_mark);
    register_effect_parser("moongate_travel", parse_moongate_travel);
    register_effect_parser("moongate", parse_moongate_travel);

    let app = AppBuilder::new(1024, 768)
        .title("Acanja - World Viewer")
        .font("assets/font_32x58.png")
        .register_components(|registry| {
            registry.register::<gw_world::position::Position>("Position".to_string());
            registry.register::<gw_world::sprite::Sprite>("Sprite".to_string());
            registry.register::<gw_world::being::Being>("Being".to_string());
            registry.register::<gw_world::task::Task>("Task".to_string());
        })
        .startup(Box::new(|_ecs: &mut Ecs| {
            register_task("ANCHORED_WANDER", acanja::ai::anchored_wander);
            register_task("RANDOM_WANDER", acanja::ai::random_wander);
            register_task("SHOPKEEPER", acanja::ai::shopkeeper);
            register_task("MOVE_MOONGATE", acanja::ai::move_moongate);
            log("REGISTERED SOSARIA AI FUNCTIONS");
        }))
        .file("assets/game_config.jsonc", Box::new(GameConfigLoader))
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_hero(ecs: &mut Ecs, dx: i32, dy: i32) {
    let hero_entity = get_hero_entity(ecs);

    let mut level = get_current_level_mut(ecs);
    level
        .resources
        .insert(UserAction::new(Box::new(MoveStepAction::new(
            hero_entity,
            dx,
            dy,
        ))));
}

fn hero_idle(ecs: &mut Ecs) {
    let (hero_entity, act_time) = {
        let mut level = get_current_level_mut(ecs);

        let hero_entity = level.resources.get::<Hero>().unwrap().entity;

        let act_time = {
            let mut entry = level.world.entry(hero_entity).unwrap();
            let actor = entry.get_component_mut::<Being>().unwrap();
            actor.act_time
        };
        (hero_entity, act_time)
    };

    let mut level = get_current_level_mut(ecs);
    level
        .resources
        .insert(UserAction::new(Box::new(IdleAction::new(
            hero_entity,
            act_time,
        ))));
}

fn get_hero_action_effects(
    ecs: &mut Ecs,
    action: &str,
) -> Option<(Entity, Point, Vec<BoxedEffect>)> {
    let action = action.to_uppercase();
    let hero_entity = get_hero_entity(ecs);
    let mut level = get_current_level_mut(ecs);

    let hero_point = level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point();

    let map = level.resources.get::<Map>().unwrap();

    let index = map.get_index(hero_point.x, hero_point.y).unwrap();

    match map.get_cell_effects(index, &action) {
        None => None,
        Some(effects) => Some((hero_entity, hero_point, effects)),
    }
}

fn try_fire_hero_action(ecs: &mut Ecs, action: &str) -> bool {
    match get_hero_action_effects(ecs, action) {
        None => false,
        Some((entity, pos, effects)) => {
            log("FIRE EFFECTS");
            for eff in effects.iter() {
                eff.fire(ecs, pos, Some(entity));
            }
            true
        }
    }
}

/*
fn get_hero_point(ecs: &mut Ecs) -> Point {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();
    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point()
}

fn try_move_hero_world(ecs: &mut Ecs, pt: &Point, flag: PortalFlags) -> bool {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();

    let hero_entity = level.resources.get::<Hero>().unwrap().entity;

    let map = level.resources.get_mut::<Map>().unwrap();
    let index = map.get_wrapped_index(pt.x, pt.y).unwrap();

    log(format!("CLICK = {:?}", pt));

    let (new_map_id, location) = {
        match map.get_portal(index) {
            None => return false,
            Some(info) => {
                if !info.flags().contains(flag) {
                    return false;
                }

                log(format!(
                    "Enter Portal = {} - {}::{}",
                    info.flavor().as_ref().unwrap_or(&"UNKNOWN".to_string()),
                    info.map_id(),
                    info.location()
                ));

                (info.map_id().to_string(), info.location().to_string())
            }
        }
    };

    let current_pt = level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point();

    drop(map);
    drop(level);

    let level = levels.current_mut();
    let mut map = level.resources.get_mut::<Map>().unwrap();
    let index = map.get_wrapped_index(current_pt.x, current_pt.y).unwrap();

    map.remove_actor(index, hero_entity);

    drop(map);
    drop(level);

    log("Moving hero to new world");
    let new_entity = levels.move_current_entity(hero_entity, &new_map_id);
    log("Changing current world");
    if levels.set_current(&new_map_id).is_err() {
        panic!("Failed to change to world - {}", new_map_id);
    }

    let level = levels.current_mut();
    level.resources.insert(Hero::new(hero_entity));

    {
        let mut camera = level
            .resources
            .get_mut_or_insert_with(|| Camera::new(CAMERA_WIDTH, CAMERA_HEIGHT));
        camera.set_follows(new_entity);
    }

    let new_pt = {
        let mut map = level.resources.get_mut::<Map>().unwrap();
        let pt = map.locations.get(&location).unwrap().clone();
        map.add_actor(pt, new_entity, true);
        map.to_point(pt)
    };
    {
        let mut entry = level.world.entry(new_entity).unwrap();
        let pos = entry.get_component_mut::<Position>().unwrap();
        pos.set(new_pt.x, new_pt.y);
    }
    {
        let map = level.resources.get::<Map>().unwrap();
        if let Some(ref welcome) = map.welcome {
            level.logger.log(welcome);
        }
    }
    true
}
*/
