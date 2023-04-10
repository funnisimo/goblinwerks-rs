use crate::being::Being;
use crate::{log::Logger, task::Executor};
use gw_app::ecs::query::IntoQuery;
use gw_app::ecs::{self, EntityStore};
use gw_app::ecs::{Entity, Resources, World};
use gw_util::rng::RandomNumberGenerator;
// use gw_app::ScreenResult;

pub struct Level {
    pub id: String,
    pub resources: Resources,
    pub world: World,
    pub logger: Logger,
    needs_draw: bool,
    pub executor: Executor,
    pub rng: RandomNumberGenerator,
}

impl Level {
    pub fn new(id: &str) -> Self {
        Level {
            id: id.to_string(),
            resources: Resources::default(),
            world: World::default(),
            logger: Logger::new(),
            needs_draw: true,
            executor: Executor::new(),
            rng: RandomNumberGenerator::new(),
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = RandomNumberGenerator::seeded(seed);
    }

    pub fn needs_draw(&self) -> bool {
        self.needs_draw
    }

    pub fn clear_needs_draw(&mut self) {
        self.needs_draw = false;
    }

    pub fn set_needs_draw(&mut self) {
        self.needs_draw = true;
    }

    // pub fn execute<F>(&mut self, func: F) -> ScreenResult
    // where
    //     F: FnOnce(&mut Level, &mut Executor) -> ScreenResult,
    // {
    //     if let Some(mut exec) = self.executor.take() {
    //         let res = (func)(self, &mut exec);
    //         self.executor = Some(exec);
    //         res
    //     } else {
    //         panic!("No executor!");
    //     }
    // }

    fn reset_tasks(&mut self) {
        let mut query = <(Entity, &Being)>::query();

        let Level {
            executor, world, ..
        } = self;

        executor.clear();

        // you can then iterate through the components found in the world
        for (entity, being) in query.iter(world) {
            executor.insert(*entity, being.act_time as u64);
        }
    }
}

pub fn move_entity(entity: Entity, src: &mut Level, dest: &mut Level) -> Entity {
    let new_entity = ecs::move_entity(entity, &mut src.world, &mut dest.world);

    // src.executor.remove(entity);

    if let Ok(being) = dest
        .world
        .entry_ref(new_entity)
        .unwrap()
        .get_component::<Being>()
    {
        dest.executor.insert(new_entity, being.act_time as u64);
    }

    new_entity
}
