use super::{basic_monster_ai, idle_ai, mirror_entity_ai, move_randomly_ai, user_control_ai};
use crate::{hero::Hero, position::Position};
use gw_app::log;
use gw_app::screen::BoxedScreen;
use gw_ecs::{Component, DenseVecStorage, Entity, World};
use gw_util::point::Point;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Mutex};

pub enum DoNextTaskResult {
    Hero,                  // Hero did something
    Other,                 // Any other entity did something
    Done,                  // Nothing done
    PushMode(BoxedScreen), // Need to push a screen
}

pub enum TaskResult {
    Retry, // Try again (unpop)
    // WaitForInput,    // IS THIS NECESSARY (IS IT DIFFERENT FROM RETRY?)
    Success(u64), // Time in future to run again
    Finished,     // This task is done - Remove the component?
    PushMode(BoxedScreen),
}

pub type TaskFn = fn(&mut World, Entity) -> TaskResult;

lazy_static! {
    static ref TASK_FNS: Mutex<HashMap<String, TaskFn>> = {
        let mut map: HashMap<String, TaskFn> = HashMap::new();
        map.insert("USER_CONTROL".to_string(), user_control_ai);
        map.insert("IDLE".to_string(), idle_ai);
        map.insert("MIRROR_ENTITY".to_string(), mirror_entity_ai);
        map.insert("BASIC_MONSTER".to_string(), basic_monster_ai);
        map.insert("MOVE_RANDOMLY".to_string(), move_randomly_ai);
        Mutex::new(map)
    };
}

pub fn register_task<S: ToString>(name: S, func: TaskFn) {
    TASK_FNS
        .lock()
        .unwrap()
        .insert(name.to_string().to_uppercase(), func);
}

pub fn get_task(name: &str) -> Option<TaskFn> {
    TASK_FNS.lock().unwrap().get(name).map(|v| *v)
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Task {
    name: String,
}

impl Task {
    pub fn new<S: ToString>(name: S) -> Self {
        Task {
            name: name.to_string().to_uppercase(),
        }
    }
}

#[derive(Debug, Clone)]
struct TaskEntry {
    // pub entity: Entity,
    pub entity: Entity,
    pub time: u64,
}

impl TaskEntry {
    // pub fn new(entity: Entity, time: u64) -> TaskEntry {
    //     TaskEntry { entity, time }
    // }
    pub fn new(entity: Entity, time: u64) -> TaskEntry {
        TaskEntry { entity, time }
    }
}

#[derive(Default)]
struct TaskList {
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
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn insert(&mut self, task: TaskEntry) -> () {
        let mut task = task;
        task.time += self.time;
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

    pub fn pop(&mut self) -> Option<TaskEntry> {
        if self.tasks.is_empty() {
            return None;
        }

        let res = self.tasks.remove(0);
        self.time = res.time;
        Some(res)
    }

    pub fn unpop(&mut self, task: TaskEntry) -> () {
        let mut task = task;
        task.time = self.time;
        self.tasks.insert(0, task);
    }
}

impl Debug for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("TaskList");
        s.field("tasks", &self.tasks.len());
        s.field("time", &self.time);
        s.finish()
    }
}

#[derive(Debug, Default)]
pub struct Executor {
    tasks: TaskList,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: TaskList::new(),
        }
    }

    pub fn time(&self) -> u64 {
        self.tasks.time
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    // pub fn insert_actor(&mut self, entity: Entity, in_time: u32) {
    //     let task = Box::new(move |ecs: &mut World| do_entity_action(entity, ecs));
    //     self.tasks.insert(task, in_time)
    // }

    pub fn insert(&mut self, entity: Entity, in_time: u64) {
        let entry = TaskEntry::new(entity, in_time);
        self.tasks.insert(entry)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.tasks.remove(entity);
    }

    fn pop(&mut self) -> Option<TaskEntry> {
        self.tasks.pop()
    }

    fn unpop(&mut self, task: TaskEntry) {
        self.tasks.unpop(task);
    }

    // pub fn get_next_action(&self, entity: Entity, ecs: &mut World) -> Option<BoxedAction> {
    //     let ai = {
    //         let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    //         let level = levels.current_mut();
    //         let mut entry = match level.world.entry(entity) {
    //             None => return None,
    //             Some(entry) => entry,
    //         };

    //         let actor = entry.get_component_mut::<Actor>().unwrap();
    //         match actor.next_action.take() {
    //             Some(action) => return Some(action),
    //             None => actor.ai.clone(),
    //         }
    //     };

    //     let ai_fn = ai.current();
    //     ai_fn.next_action(ecs, entity)
    // }

    // #[must_use]
    // pub fn do_next_action(&mut self, ecs: &mut World) -> DoNextActionResult {
    //     let hero_entity = {
    //         let levels = ecs.resources.get::<Levels>().unwrap();
    //         let level = levels.current();
    //         let entity = level.resources.get::<Hero>().unwrap().entity;
    //         entity
    //     };

    //     loop {
    //         match self.tasks.pop() {
    //             None => return DoNextActionResult::Done,
    //             Some(entity) => {
    //                 let is_player = entity == hero_entity;

    //                 match self.get_next_action(entity, ecs) {
    //                     None => continue,
    //                     Some(mut action) => {
    //                         'inner: loop {
    //                             match action.execute(ecs) {
    //                                 ActionResult::Dead(_) => {
    //                                     // no rescedule - entity dead
    //                                     {
    //                                         let mut levels =
    //                                             ecs.resources.get_mut::<Levels>().unwrap();
    //                                         let level = levels.current_mut();
    //                                         level
    //                                             .logger
    //                                             .debug(format!("{:?} - Dead result", entity));
    //                                     }
    //                                     break 'inner;
    //                                 }
    //                                 ActionResult::Done(time) => {
    //                                     // do_debug!("{} - Done result : {}", entity, time);
    //                                     self.tasks.insert(entity, time);

    //                                     break 'inner;
    //                                 }
    //                                 ActionResult::Fail(msg) => {
    //                                     {
    //                                         let mut levels =
    //                                             ecs.resources.get_mut::<Levels>().unwrap();
    //                                         let level = levels.current_mut();
    //                                         level.logger.debug(format!("#[violetred]{}", msg));
    //                                     }
    //                                     self.tasks.unpop(entity); // reschedule in future?
    //                                     break 'inner;
    //                                 }
    //                                 ActionResult::Replace(new_action) => {
    //                                     // do_debug!("{} - Replace result - {:?}", entity, new_action);
    //                                     action = new_action;
    //                                 }
    //                                 ActionResult::WaitForInput => {
    //                                     // debug_msg(format!("{} - Wait for input", entity));
    //                                     self.tasks.unpop(entity); // try again next cycle
    //                                     return DoNextActionResult::Done;
    //                                 }
    //                                 ActionResult::Retry => {
    //                                     self.tasks.unpop(entity);
    //                                     break 'inner;
    //                                 }
    //                                 ActionResult::PushMode(mode) => {
    //                                     self.tasks.unpop(entity); // try again next cycle
    //                                     return DoNextActionResult::PushMode(mode);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 return match is_player {
    //                     true => DoNextActionResult::Hero,
    //                     false => DoNextActionResult::Mob,
    //                 };
    //             }
    //         }
    //     }
    // }
}

#[must_use]
pub fn do_next_task(world: &mut World) -> DoNextTaskResult {
    let task = world.write_resource::<Executor>().pop();

    match task {
        None => DoNextTaskResult::Done,
        Some(task) => {
            let hero_entity = get_hero_entity(world);
            let res = if hero_entity == task.entity {
                DoNextTaskResult::Hero
            } else {
                DoNextTaskResult::Other
            };

            let task_comp = world
                .read_component::<Task>()
                .get(task.entity)
                .map(|t| t.clone());
            match task_comp {
                None => res,
                Some(task_comp) => {
                    let task_fn = match get_task(&task_comp.name) {
                        None => {
                            log(format!("Task Function not found - {}", task_comp.name));
                            // TODO - Remove Task component?
                            return res;
                        } // Do not add me back
                        Some(task_fn) => task_fn,
                    };

                    match task_fn(world, task.entity) {
                        TaskResult::Finished => res,
                        TaskResult::Retry => {
                            // unpop
                            world.write_resource::<Executor>().unpop(task);

                            DoNextTaskResult::Done // Need to break loop regardless of whether or not this is the hero acting
                        }
                        TaskResult::Success(t) => {
                            // insert
                            world.write_resource::<Executor>().insert(task.entity, t);
                            res
                        }
                        TaskResult::PushMode(mode_info) => {
                            // run the given screen and then try this task again...
                            world.write_resource::<Executor>().unpop(task);
                            DoNextTaskResult::PushMode(mode_info)
                        }
                    }
                }
            }
        }
    }
}

// TODO - Move to Level
pub fn get_hero_entity(world: &World) -> Entity {
    let hero = world.read_resource::<Hero>();
    hero.entity
}

pub fn get_hero_entity_point(world: &World) -> (Entity, Point) {
    let entity = get_hero_entity(world);
    let positions = world.read_component::<Position>();

    let point = positions.get(entity).unwrap().point();
    (entity, point)
}

fn execute_task(task: &str, world: &mut World, entity: Entity) -> TaskResult {
    match get_task(task) {
        None => {
            log(format!("Task Function not found - {}", task));
            TaskResult::Finished
        } // Do not add me back
        Some(task_fn) => task_fn(world, entity),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gw_ecs::{Builder, World};

    #[derive(Component)]
    struct Count(u32);

    #[test]
    fn one_task() {
        let mut scheduler = Executor::new();
        assert_eq!(scheduler.is_empty(), true);

        // let mut ecs = World::new();
        let mut world = World::empty(1);
        world.register::<Task>();
        world.register::<Count>();

        let entity = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();

        // fn inc_count(ecs: &mut World, entity: Entity) -> TaskResult {
        //     let mut level = get_current_level_mut(ecs);
        //     let mut entry = level.world.entry_mut(entity).unwrap();
        //     let mut count = entry.get_component_mut::<Count>().unwrap();
        //     count.0 += 1;
        //     TaskResult::Success(100)
        // }
        // register_task("TEST".to_string(), inc_count);

        scheduler.insert(entity, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                assert_eq!(task.entity, entity);
                assert_eq!(scheduler.time(), 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(scheduler.is_empty());
    }

    #[test]
    fn two_actions() {
        let mut scheduler = Executor::new();
        assert_eq!(scheduler.is_empty(), true);

        let mut world = World::empty("TEST");
        world.register::<Task>();
        world.register::<Count>();

        let entity_a = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();
        let entity_b = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();

        // fn inc_count(ecs: &mut World, entity: Entity) -> TaskResult {
        //     let mut level = get_current_level_mut(ecs);
        //     let mut entry = level.world.entry_mut(entity).unwrap();
        //     let mut count = entry.get_component_mut::<Count>().unwrap();
        //     count.0 += 1;
        //     TaskResult::Success(100)
        // }
        // register_task("TEST".to_string(), inc_count);

        scheduler.insert(entity_a, 10);
        scheduler.insert(entity_b, 20);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                assert_eq!(task.entity, entity_a);
                assert_eq!(scheduler.time(), 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time(), 10);
    }

    #[test]
    fn two_actions_last_first() {
        let mut scheduler = Executor::new();
        assert_eq!(scheduler.is_empty(), true);

        let mut world = World::empty("TEST");
        world.register::<Task>();
        world.register::<Count>();

        let entity_a = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();
        let entity_b = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();

        // fn inc_count(ecs: &mut World, entity: Entity) -> TaskResult {
        //     let mut level = get_current_level_mut(ecs);
        //     let mut entry = level.world.entry_mut(entity).unwrap();
        //     let mut count = entry.get_component_mut::<Count>().unwrap();
        //     count.0 += 1;
        //     TaskResult::Success(100)
        // }
        // register_task("TEST".to_string(), inc_count);

        scheduler.insert(entity_b, 20);
        scheduler.insert(entity_a, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                assert_eq!(task.entity, entity_a);
                assert_eq!(scheduler.time(), 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time(), 10);
    }

    #[test]
    fn two_actions_second() {
        let mut scheduler = Executor::new();
        assert_eq!(scheduler.is_empty(), true);

        let mut world = World::empty("TEST");
        world.register::<Task>();
        world.register::<Count>();

        let entity_a = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();
        let entity_b = world
            .create_entity()
            .with(Task::new("TEST".to_string()))
            .with(Count(0))
            .build();

        // fn inc_count(ecs: &mut World, entity: Entity) -> TaskResult {
        //     let mut level = get_current_level_mut(ecs);
        //     let mut entry = level.world.entry_mut(entity).unwrap();
        //     let mut count = entry.get_component_mut::<Count>().unwrap();
        //     count.0 += 1;
        //     TaskResult::Success(100)
        // }
        // register_task("TEST".to_string(), inc_count);

        scheduler.insert(entity_b, 20);
        scheduler.insert(entity_a, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                assert_eq!(task.entity, entity_a);
                assert_eq!(scheduler.time(), 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                assert_eq!(task.entity, entity_b);
                assert_eq!(scheduler.time(), 20);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.time(), 20);
    }
}
