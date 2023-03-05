use gw_app::ecs::Entity;

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
