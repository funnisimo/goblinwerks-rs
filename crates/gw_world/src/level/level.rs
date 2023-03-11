use crate::{actor::Actor, log::Logger, task::Executor};
use gw_app::ecs::query::IntoQuery;
use gw_app::ecs::{self, EntityStore};
use gw_app::ecs::{Entity, Resources, World};
use gw_app::ScreenResult;

pub struct Level {
    pub id: String,
    pub resources: Resources,
    pub world: World,
    pub logger: Logger,
    needs_draw: bool,
    executor: Option<Executor>,
}

impl Level {
    pub fn new(id: &str) -> Self {
        Level {
            id: id.to_string(),
            resources: Resources::default(),
            world: World::default(),
            logger: Logger::new(),
            needs_draw: true,
            executor: Some(Executor::new()),
        }
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

    pub fn execute<F>(&mut self, func: F) -> ScreenResult
    where
        F: FnOnce(&mut Level, &mut Executor) -> ScreenResult,
    {
        if let Some(mut exec) = self.executor.take() {
            let res = (func)(self, &mut exec);
            self.executor = Some(exec);
            res
        } else {
            panic!("No executor!");
        }
    }

    pub fn reset_tasks(&mut self) {
        let mut query = <(Entity, &Actor)>::query();

        let executor = self.executor.as_mut().unwrap();

        // you can then iterate through the components found in the world
        for (entity, actor) in query.iter(&self.world) {
            executor.insert(*entity, actor.act_time);
        }
    }
}

pub fn move_entity(entity: Entity, src: &mut Level, dest: &mut Level) -> Entity {
    let new_entity = ecs::move_entity(entity, &mut src.world, &mut dest.world);

    if let Some(exec) = src.executor.as_mut() {
        exec.remove(entity);
    }

    if let Ok(actor) = dest
        .world
        .entry_ref(new_entity)
        .unwrap()
        .get_component::<Actor>()
    {
        if let Some(exec) = dest.executor.as_mut() {
            exec.insert(new_entity, actor.act_time);
        }
    }

    new_entity
}
