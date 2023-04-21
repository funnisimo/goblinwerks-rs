use acanja::effect::{parse_gremlins, parse_mark, parse_moongate_travel, parse_winds};
use acanja::loader::GameConfigLoader;
use gw_app::ecs::systems::CommandBuffer;
use gw_app::ecs::{Entity, Read, ResourceSet, Write};
use gw_app::ecs::{IntoQuery, Query};
use gw_app::*;
use gw_util::point::Point;
use gw_world::action::idle::IdleAction;
use gw_world::action::move_step::MoveStepAction;
use gw_world::being::{spawn_being, Being, BeingFlags, BeingKinds, Stats};
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::combat::Melee;
use gw_world::effect::{register_effect_parser, BoxedEffect};
use gw_world::fov::update_fov;
use gw_world::hero::Hero;
use gw_world::horde::{pick_random_horde, HordeSpawn};
use gw_world::level::{get_current_level, get_current_level_mut, with_current_level, Levels};
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

    fn pre_update(&mut self, ecs: &mut Ecs) {
        // spawn stuff?
        spawn_hordes(ecs);
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
                    if let Ok(melee) = entry.get_component::<Melee>() {
                        log(format!("MELEE({:?}) = {:?}", entity, melee));
                    }
                    if let Ok(stats) = entry.get_component::<Stats>() {
                        log(format!("STATS - {:?}", *stats));
                    }
                }
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        // Pre Update

        // spawn things?
        self.pre_update(ecs);

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
            registry.register::<gw_world::being::Stats>("Stats".to_string());
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

fn spawn_hordes(ecs: &mut Ecs) {
    let current_time = { get_current_level(ecs).executor.time() };
    let depth = 1;

    let max_alive = {
        let level = get_current_level_mut(ecs);
        let mut info = match level.resources.get_mut::<HordeSpawn>() {
            None => return,
            Some(info) => info,
        };
        if info.next_time > current_time {
            return;
        }
        info.next_time += info.check_delay;
        info.max_alive
    };

    {
        let level = get_current_level(ecs);

        let mut beings = <&Being>::query();

        let count = beings
            .iter(&level.world)
            .filter(|b| b.has_flag(BeingFlags::SPAWNED))
            .count() as u32;
        if count >= max_alive {
            return;
        }
    }

    // We got to here so we need to spawn a horde...
    if let Some(horde) = pick_random_horde(ecs, depth) {
        log(format!("SPAWN - {:?}", horde));
    }
}
