use crate::{
    action::{ActionResult, BoxedAction},
    actor::Actor,
    hero::Hero,
    level::Level,
    log::Logger,
};
use gw_app::{ecs::Entity, screen::BoxedScreen};

#[derive(Copy, Clone, Debug)]
struct TaskEntry {
    pub entity: Entity,
    pub time: u64,
}

impl TaskEntry {
    pub fn new(entity: Entity, time: u64) -> TaskEntry {
        TaskEntry { entity, time }
    }
}

pub struct TaskList {
    tasks: Vec<TaskEntry>,
    pub time: u64,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            time: 0,
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn next_time(&self) -> u64 {
        match self.tasks.get(0) {
            Some(a) => a.time,
            None => self.time,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.len() == 0
    }

    pub fn insert(&mut self, entity: Entity, in_time: u32) -> () {
        let task = TaskEntry::new(entity, in_time as u64 + self.time);
        match self.tasks.iter().position(|t| t.time > task.time) {
            Some(idx) => {
                self.tasks.insert(idx, task);
            }
            None => {
                self.tasks.push(task);
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        self.tasks = self
            .tasks
            .iter()
            .filter(|&task| task.entity != entity)
            .cloned()
            .collect();
    }

    pub fn pop(&mut self) -> Option<Entity> {
        if self.tasks.len() < 1 {
            return None;
        }

        let res = self.tasks.remove(0);
        self.time = res.time;
        Some(res.entity)
    }

    pub fn unpop(&mut self, entity: Entity) -> () {
        self.tasks.insert(0, TaskEntry::new(entity, self.time));
    }
}

pub enum DoNextActionResult {
    Hero,
    Mob,
    Done,
    PushMode(BoxedScreen),
}

pub struct Executor {
    tasks: TaskList,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: TaskList::new(),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn insert(&mut self, entity: Entity, in_time: u32) {
        self.tasks.insert(entity, in_time)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.tasks.remove(entity);
    }

    pub fn get_next_action(&self, entity: Entity, ecs: &mut Level) -> Option<BoxedAction> {
        let mut entry = match ecs.world.entry(entity) {
            None => return None,
            Some(entry) => entry,
        };

        let ai_fn = {
            let actor = entry.get_component_mut::<Actor>().unwrap();
            match actor.next_action.take() {
                Some(action) => return Some(action),
                None => actor.ai,
            }
        };

        ai_fn(ecs, entity)
    }

    #[must_use]
    pub fn do_next_action(&mut self, ecs: &mut Level) -> DoNextActionResult {
        let hero_entity = ecs.resources.get::<Hero>().unwrap().entity;

        loop {
            match self.tasks.pop() {
                None => return DoNextActionResult::Done,
                Some(entity) => {
                    let is_player = entity == hero_entity;

                    match self.get_next_action(entity, ecs) {
                        None => continue,
                        Some(mut action) => {
                            'inner: loop {
                                match action.execute(ecs) {
                                    ActionResult::Dead(_) => {
                                        // no rescedule - entity dead
                                        let mut logger = ecs.resources.get_mut::<Logger>().unwrap();
                                        logger.debug(format!("{:?} - Dead result", entity));
                                        break 'inner;
                                    }
                                    ActionResult::Done(time) => {
                                        // do_debug!("{} - Done result : {}", entity, time);
                                        self.tasks.insert(entity, time);

                                        break 'inner;
                                    }
                                    ActionResult::Fail(msg) => {
                                        let mut logger = ecs.resources.get_mut::<Logger>().unwrap();
                                        logger.debug(format!("#[violetred]{}", msg));
                                        self.tasks.unpop(entity); // reschedule in future?
                                        break 'inner;
                                    }
                                    ActionResult::Replace(new_action) => {
                                        // do_debug!("{} - Replace result - {:?}", entity, new_action);
                                        action = new_action;
                                    }
                                    ActionResult::WaitForInput => {
                                        // debug_msg(format!("{} - Wait for input", entity));
                                        self.tasks.unpop(entity); // try again next cycle
                                        return DoNextActionResult::Done;
                                    }
                                    ActionResult::Retry => {
                                        self.tasks.unpop(entity);
                                        break 'inner;
                                    }
                                    ActionResult::PushMode(mode) => {
                                        self.tasks.unpop(entity); // try again next cycle
                                        return DoNextActionResult::PushMode(mode);
                                    }
                                }
                            }
                        }
                    }
                    return match is_player {
                        true => DoNextActionResult::Hero,
                        false => DoNextActionResult::Mob,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use gw_app::ecs::World;

    use crate::position::Position;

    use super::*;
    // use crate::prelude::*;

    #[test]
    fn one_task() {
        let mut scheduler = TaskList::new();
        assert_eq!(scheduler.is_empty(), true);

        let mut world: World = World::default();
        let entity = world.push((Position::new(0, 0),));

        scheduler.insert(entity, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(entity) => {
                assert_eq!(entity, entity);
                assert_eq!(scheduler.time, 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(scheduler.is_empty());
    }

    #[test]
    fn two_actions() {
        let mut scheduler = TaskList::new();

        let mut world: World = World::default();
        let entity_a = world.push((Position::new(0, 0),));
        let entity_b = world.push((Position::new(1, 1),));

        scheduler.insert(entity_a, 10);
        scheduler.insert(entity_b, 20);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(entity) => {
                assert_eq!(scheduler.time, 10);
                assert_eq!(entity, entity_a);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time, 10);
    }

    #[test]
    fn two_actions_last_first() {
        let mut scheduler = TaskList::new();

        let mut world: World = World::default();
        let entity_a = world.push((Position::new(0, 0),));
        let entity_b = world.push((Position::new(1, 1),));

        scheduler.insert(entity_b, 20);
        scheduler.insert(entity_a, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(entity) => {
                assert_eq!(scheduler.time, 10);
                assert_eq!(entity, entity_a);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time, 10);
    }

    #[test]
    fn two_actions_second() {
        let mut scheduler = TaskList::new();

        let mut world: World = World::default();
        let entity_a = world.push((Position::new(0, 0),));
        let entity_b = world.push((Position::new(1, 1),));

        scheduler.insert(entity_a, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(entity) => {
                assert_eq!(scheduler.time, 10);
                assert_eq!(entity, entity_a);
            }
            None => {
                panic!("Should be there.");
            }
        }

        scheduler.insert(entity_b, 20);

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time, 10);

        match scheduler.pop() {
            Some(entity) => {
                assert_eq!(scheduler.time, 30);
                assert_eq!(entity, entity_b);
            }
            None => {
                panic!("Should be there.");
            }
        }
        assert_eq!(scheduler.time, 30);
    }
}
