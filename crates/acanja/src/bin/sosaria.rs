use acanja::effect::{parse_gremlins, parse_mark, parse_moongate_travel, parse_winds};
use acanja::loader::{GameConfigLoader, StartMap};
use acanja::map::prefab::Prefabs;
use acanja::tasks::AnchorPos;
use gw_app::*;
use gw_ecs::{Builder, Entity, Join, World};
use gw_util::grid::{dump_grid, random_point_with};
use gw_util::point::Point;
use gw_util::rng::RandomNumberGenerator;
use gw_world::action::idle::IdleAction;
use gw_world::action::move_step::MoveStepAction;
use gw_world::being::{spawn_being, Being, BeingFlags, BeingKinds, Stats};
use gw_world::camera::{update_camera_follows, Camera};
use gw_world::combat::Melee;
use gw_world::effect::{register_effect_parser, BoxedEffect};
use gw_world::fov::update_fov;
use gw_world::hero::Hero;
use gw_world::horde::{pick_random_horde, spawn_horde, HordeSpawn};
use gw_world::map::{ensure_area_grid, AreaGrid, Cell, Map};
use gw_world::position::Position;
use gw_world::task::{do_next_task, DoNextTaskResult, Executor, Task, UserAction};
use gw_world::task::{get_hero_entity, register_task};
use gw_world::tile::Tiles;
use gw_world::widget::Viewport;
use std::ops::DerefMut;

// const CAMERA_WIDTH: u32 = 1024 / 32;
// const CAMERA_HEIGHT: u32 = 768 / 32;

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

        update_camera_follows(ecs.current_world_mut());
        update_fov(ecs.current_world_mut());
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        ecs.ensure_global::<Tiles>();
        ecs.ensure_global::<Prefabs>();
        ecs.ensure_global::<BeingKinds>();

        set_start_map(ecs);

        {
            let level = ecs.current_world_mut();
            log(format!("START MAP = {}", level.id()));

            let start_pos = {
                let map = level.read_resource::<Map>();
                // dump_map(&*map);
                // log(format!("map size = {:?}", map.get_size()));
                map.to_point(map.get_location("START").unwrap())
            };

            let entity = {
                let actor_kinds = level.read_global::<BeingKinds>();
                let hero_kind = actor_kinds.get("HERO").unwrap();
                log(format!("HERO - {:?}", hero_kind));
                spawn_being(&hero_kind, level, start_pos)
            };

            ///////////////////////////////////////////

            // SPAWN MOONGATES...

            ///////////////////////////////////////////

            {
                let mut camera = level.write_resource::<Camera>();
                camera.set_follows(entity);
            }

            let moongate = level
                .create_entity()
                .with(Task::new("MOVE_MOONGATE"))
                .build();

            {
                let mut executor = level.write_resource::<Executor>();
                executor.insert(moongate, 0);
                log(format!("STARTING TASKS = {:?}", *executor));
            }

            // level.ensure_resource::<NeedsDraw>();
            // level.ensure_resource::<UserAction>();
            // level.ensure_resource::<Logger>();
        }

        ecs.maintain();
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs.current_world_mut(), ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    move_hero(ecs.current_world_mut(), 0, 1);
                }
                VirtualKeyCode::Left => {
                    move_hero(ecs.current_world_mut(), -1, 0);
                }
                VirtualKeyCode::Up => {
                    move_hero(ecs.current_world_mut(), 0, -1);
                }
                VirtualKeyCode::Right => {
                    move_hero(ecs.current_world_mut(), 1, 0);
                }
                _ => {}
            },
            AppEvent::CharEvent(ch) => match ch {
                '<' => {
                    // Climb
                    log("CLIMB");
                    try_fire_hero_action(ecs.current_world_mut(), "climb");
                    // let hero_point = get_hero_point(ecs.current_world_mut());
                    // try_move_hero_world(ecs.current_world_mut(), &hero_point, PortalFlags::ON_CLIMB);
                }
                '>' => {
                    // Descend
                    log("DESCEND");
                    try_fire_hero_action(ecs.current_world_mut(), "descend");
                    // let hero_point = get_hero_point(ecs.current_world_mut());
                    // try_move_hero_world(ecs.current_world_mut(), &hero_point, PortalFlags::ON_DESCEND);
                }
                '.' | ' ' => {
                    hero_idle(ecs.current_world_mut());
                }
                't' => {
                    let executor = ecs.read_resource::<Executor>();
                    println!("TASKS = {:?}", *executor);
                }
                'a' => {
                    let beings = ecs.read_component::<Being>();
                    // you can then iterate through the components found in the world
                    for actor in beings.join() {
                        println!("{:?}", actor);
                    }
                }
                'k' => {
                    let kinds = ecs.read_global::<BeingKinds>();
                    kinds.dump();
                }
                'm' => {
                    for world in ecs.iter_worlds() {
                        println!("world = {:?}", world.id());
                    }
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

                let level = ecs.current_world();
                let map = level.read_resource::<Map>();

                let idx = map.get_index(pt.x, pt.y).unwrap();
                let cell = map.get_cell(idx).unwrap();

                log(format!("GROUND = {:?}", cell.ground()));
                log(format!("FIXTURE = {:?}", cell.fixture()));

                if let Some(effects) = map.cell_effects.get(&idx) {
                    log(format!("CELL EFFECTS = {:?}", effects));
                }

                for entity in map.iter_beings(idx) {
                    let beings = level.read_component::<Being>();
                    let being = beings.get(entity).unwrap();
                    log(format!("BEING({:?}) = {:?}", entity, being));
                    let melees = level.read_component::<Melee>();
                    if let Some(melee) = melees.get(entity) {
                        log(format!("MELEE({:?}) = {:?}", entity, melee));
                    }
                    let stats = level.read_component::<Stats>();
                    if let Some(stats) = stats.get(entity) {
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
    }

    fn render(&mut self, app: &mut Ecs) {
        self.viewport.draw_level(app.current_world_mut());
        self.viewport.render(app);

        // TODO - This should be in APP
        app.maintain();
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
        .register_components(|ecs| {
            gw_world::register_components(ecs);
            ecs.register::<AnchorPos>();
        })
        .startup(Box::new(|_ecs: &mut Ecs| {
            register_task("ANCHORED_WANDER", acanja::tasks::anchored_wander);
            register_task("RANDOM_MOVE", acanja::tasks::random_move);
            register_task("SHOPKEEPER", acanja::tasks::shopkeeper);
            register_task("MOVE_MOONGATE", acanja::tasks::move_moongate);
            log("REGISTERED SOSARIA AI FUNCTIONS");
        }))
        .file("assets/game_config.jsonc", Box::new(GameConfigLoader))
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn move_hero(level: &mut World, dx: i32, dy: i32) {
    let hero_entity = get_hero_entity(level);
    let mut user_action = level.write_resource::<UserAction>();
    user_action.set(Box::new(MoveStepAction::new(hero_entity, dx, dy)));
}

fn hero_idle(level: &mut World) {
    let (hero_entity, act_time) = {
        let hero_entity = level.read_resource::<Hero>().entity;

        let act_time = {
            let beings = level.read_component::<Being>();
            let actor = beings.get(hero_entity).unwrap();
            actor.act_time
        };
        (hero_entity, act_time)
    };

    let mut user_action = level.write_resource::<UserAction>();
    user_action.set(Box::new(IdleAction::new(hero_entity, act_time)));
}

fn get_hero_action_effects(
    world: &mut World,
    action: &str,
) -> Option<(Entity, Point, Vec<BoxedEffect>)> {
    let action = action.to_uppercase();
    let hero_entity = get_hero_entity(world);

    let hero_point = world
        .read_component::<Position>()
        .get(hero_entity)
        .unwrap()
        .point();

    let map = world.read_resource::<Map>();

    let index = map.get_index(hero_point.x, hero_point.y).unwrap();

    match map.get_cell_effects(index, &action) {
        None => None,
        Some(effects) => Some((hero_entity, hero_point, effects)),
    }
}

fn try_fire_hero_action(world: &mut World, action: &str) -> bool {
    match get_hero_action_effects(world, action) {
        None => false,
        Some((entity, pos, effects)) => {
            log("FIRE EFFECTS");
            for eff in effects.iter() {
                eff.fire(world, pos, Some(entity));
            }
            true
        }
    }
}

fn spawn_hordes(ecs: &mut Ecs) {
    let current_time = ecs.read_resource::<Executor>().time();
    let depth = 1;

    let max_alive = {
        let level = ecs.current_world_mut();
        let mut info = match level.try_write_resource::<HordeSpawn>() {
            None => {
                return;
            }
            Some(info) => info,
        };
        if info.next_time > current_time {
            return;
        }
        info.next_time += info.check_delay;
        info.max_alive
    };

    {
        let beings = ecs.read_component::<Being>();

        let count = beings
            .join()
            .filter(|b| b.has_flag(BeingFlags::SPAWNED))
            .count() as u32;
        if count >= max_alive {
            log(format!(
                "spawn_hordes - too many alive => count:{} > max_alive:{}",
                count, max_alive
            ));
            return;
        }
    }

    // We got to here so we need to spawn a horde...
    if let Some(horde) = pick_random_horde(ecs.current_world_mut(), depth) {
        log(format!("SPAWN - {:?}", horde));

        // need spawn point...

        ensure_area_grid(ecs.current_world_mut());
        let mut level = ecs.current_world_mut();
        let spawn_point = {
            let area_grid = level.read_resource::<AreaGrid>();
            let mut rng = level.write_resource::<RandomNumberGenerator>();

            let spawn_point = match random_point_with(&area_grid.grid(), |v, _, _| *v > 0, &mut rng)
            {
                None => {
                    log(format!("Failed to find point to spawn horde"));
                    dump_grid(area_grid.grid());
                    return;
                }
                Some(point) => point,
            };

            let map = level.read_resource::<Map>();
            let map_idx = map.get_index(spawn_point.x, spawn_point.y).unwrap();
            if map.is_blocked(map_idx) {
                log(format!(
                    "Not spawning - location blocked - {:?}",
                    spawn_point
                ));
                return;
            }
            spawn_point
        };

        let entity = spawn_horde(&horde, level.deref_mut(), spawn_point);
        log(format!("Spawned entity {:?} @ {:?}", entity, spawn_point));
    }
}

fn set_start_map(ecs: &mut Ecs) {
    let map_id = match ecs.try_read_global::<StartMap>() {
        None => return,
        Some(start_map) => start_map.map_id,
    };

    ecs.set_current_world(map_id).unwrap();
}
