use std::fmt::Debug;

use crate::{
    action::{dead::DeadAction, idle::IdleAction, ActionResult, BoxedAction},
    ai::Actor,
    being::Being,
    hero::Hero,
    level::{get_current_level_mut, with_current_level, with_current_level_mut},
};
use gw_app::{ecs::Entity, screen::BoxedScreen, Ecs};

pub enum DoNextActionResult {
    Hero,
    Mob,
    Other,
    Done,
    PushMode(BoxedScreen),
}

pub type TaskFn = dyn Fn(&mut Ecs) -> DoNextActionResult;

pub type BoxedTask = Box<TaskFn>;

struct TaskEntry {
    // pub entity: Entity,
    pub task: BoxedTask,
    pub time: u64,
}

impl TaskEntry {
    // pub fn new(entity: Entity, time: u64) -> TaskEntry {
    //     TaskEntry { entity, time }
    // }
    pub fn new(task: BoxedTask, time: u64) -> TaskEntry {
        TaskEntry { task, time }
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
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn insert(&mut self, task: BoxedTask, in_time: u32) -> () {
        let task = TaskEntry::new(task, in_time as u64 + self.time);
        match self.tasks.iter().position(|t| t.time > task.time) {
            Some(idx) => {
                self.tasks.insert(idx, task);
            }
            None => {
                self.tasks.push(task);
            }
        }
    }

    // pub fn remove(&mut self, entity: Entity) {
    //     self.tasks = self
    //         .tasks
    //         .iter()
    //         .filter(|&task| task.entity != entity)
    //         .cloned()
    //         .collect();
    // }

    pub fn pop(&mut self) -> Option<BoxedTask> {
        if self.tasks.len() < 1 {
            return None;
        }

        let res = self.tasks.remove(0);
        self.time = res.time;
        Some(res.task)
    }

    pub fn unpop(&mut self, task: BoxedTask) -> () {
        self.tasks.insert(0, TaskEntry::new(task, self.time));
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

#[derive(Debug)]
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

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn insert_actor(&mut self, entity: Entity, in_time: u32) {
        let task = Box::new(move |ecs: &mut Ecs| do_entity_action(entity, ecs));
        self.tasks.insert(task, in_time)
    }

    pub fn insert(&mut self, task: BoxedTask, in_time: u32) {
        self.tasks.insert(task, in_time)
    }

    // pub fn remove(&mut self, entity: Entity) {
    //     self.tasks.remove(entity);
    // }

    fn pop(&mut self) -> Option<BoxedTask> {
        self.tasks.pop()
    }

    fn unpop(&mut self, task: BoxedTask) {
        self.tasks.unpop(task);
    }

    // pub fn get_next_action(&self, entity: Entity, ecs: &mut Ecs) -> Option<BoxedAction> {
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
    // pub fn do_next_action(&mut self, ecs: &mut Ecs) -> DoNextActionResult {
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
fn get_next_action(entity: Entity, ecs: &mut Ecs) -> BoxedAction {
    let (ai_fn, idle_time) = {
        let mut level = get_current_level_mut(ecs);
        let mut entry = match level.world.entry(entity) {
            None => return Box::new(DeadAction::new(entity)),
            Some(entry) => entry,
        };

        let actor = entry.get_component_mut::<Actor>().unwrap();
        match actor.next_action.take() {
            Some(action) => return action,
            None => (actor.ai.current(), actor.act_time),
        }
    };

    ai_fn(ecs, entity).unwrap_or(Box::new(IdleAction::new(entity, idle_time)))
}

#[must_use]
pub fn do_next_action(ecs: &mut Ecs) -> DoNextActionResult {
    let task = with_current_level_mut(ecs, |level| level.executor.pop());

    match task {
        None => DoNextActionResult::Done,
        Some(task) => task(ecs),
    }
}

fn do_entity_action(entity: Entity, ecs: &mut Ecs) -> DoNextActionResult {
    let hero_entity = with_current_level(ecs, |level| {
        let entity = level.resources.get::<Hero>().unwrap().entity;
        entity
    });

    let is_player = entity == hero_entity;

    let mut action = get_next_action(entity, ecs);

    'inner: loop {
        match action.execute(ecs) {
            ActionResult::Dead(_) => {
                // no rescedule - entity dead
                with_current_level_mut(ecs, |level| {
                    level.logger.debug(format!("{:?} - Dead result", entity));
                });
                break 'inner;
            }
            ActionResult::Done(time) => {
                // do_debug!("{} - Done result : {}", entity, time);
                with_current_level_mut(ecs, |level| {
                    level.executor.insert_actor(entity, time);
                });

                break 'inner;
            }
            ActionResult::Fail(msg) => {
                with_current_level_mut(ecs, |level| {
                    level.logger.debug(format!("#[violetred]{}", msg));
                    let task: BoxedTask = Box::new(move |ecs| do_entity_action(entity, ecs));
                    level.executor.unpop(task);
                });
                break 'inner;
            }
            ActionResult::Replace(new_action) => {
                // do_debug!("{} - Replace result - {:?}", entity, new_action);
                action = new_action;
            }
            ActionResult::WaitForInput => {
                // debug_msg(format!("{} - Wait for input", entity));
                with_current_level_mut(ecs, |level| {
                    let ent = entity;
                    let task: BoxedTask = Box::new(move |ecs_a| do_entity_action(ent, ecs_a));
                    level.executor.unpop(task);
                });
                return DoNextActionResult::Done;
            }
            ActionResult::Retry => {
                with_current_level_mut(ecs, |level| {
                    let task: BoxedTask = Box::new(move |ecs_a| do_entity_action(entity, ecs_a));
                    level.executor.unpop(task);
                });
                break 'inner;
            }
            ActionResult::PushMode(mode) => {
                with_current_level_mut(ecs, |level| {
                    let task: BoxedTask = Box::new(move |ecs_a| do_entity_action(entity, ecs_a));
                    level.executor.unpop(task);
                });
                return DoNextActionResult::PushMode(mode);
            }
        }
    }

    return match is_player {
        true => DoNextActionResult::Hero,
        false => DoNextActionResult::Mob,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use gw_app::ecs::Ecs;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    };

    #[test]
    fn one_task() {
        let mut scheduler = TaskList::new();
        assert_eq!(scheduler.is_empty(), true);

        let mut ecs = Ecs::new();

        let count = Arc::new(AtomicI32::new(0));
        let c2 = count.clone();
        let task: BoxedTask = Box::new(move |_| {
            c2.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        scheduler.insert(task, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                task(&mut ecs);
                assert_eq!(count.load(Ordering::Acquire), 1);
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

        let mut ecs: Ecs = Ecs::new();

        let count = Arc::new(AtomicI32::new(0));
        let c2 = count.clone();
        let task_a: BoxedTask = Box::new(move |_| {
            c2.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        let c3 = count.clone();
        let task_b: BoxedTask = Box::new(move |_| {
            c3.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        scheduler.insert(task_a, 10);
        scheduler.insert(task_b, 20);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                task(&mut ecs);
                assert_eq!(scheduler.time, 10);
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

        let mut ecs: Ecs = Ecs::new();

        let count = Arc::new(AtomicI32::new(0));
        let c2 = count.clone();
        let task_a: BoxedTask = Box::new(move |_| {
            c2.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        let c3 = count.clone();
        let task_b: BoxedTask = Box::new(move |_| {
            c3.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        scheduler.insert(task_b, 20);
        scheduler.insert(task_a, 10);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                task(&mut ecs);
                assert_eq!(scheduler.time, 10);
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

        let mut ecs: Ecs = Ecs::new();

        let count = Arc::new(AtomicI32::new(0));
        let c2 = count.clone();
        let task_a: BoxedTask = Box::new(move |_| {
            c2.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        let c3 = count.clone();
        let task_b: BoxedTask = Box::new(move |_| {
            c3.fetch_add(1, Ordering::Acquire);
            DoNextActionResult::Done
        });

        scheduler.insert(task_a, 10);
        scheduler.insert(task_b, 20);

        assert_eq!(scheduler.is_empty(), false);
        match scheduler.pop() {
            Some(task) => {
                task(&mut ecs);
                assert_eq!(scheduler.time, 10);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.time, 10);

        match scheduler.pop() {
            Some(task) => {
                task(&mut ecs);
                assert_eq!(scheduler.time, 20);
            }
            None => {
                panic!("Should be there.");
            }
        }

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.time, 20);
    }
}
