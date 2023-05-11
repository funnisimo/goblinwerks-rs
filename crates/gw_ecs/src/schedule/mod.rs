use crate::shred::RunNow;
use crate::{Ecs, ResourceId, World};
use std::collections::HashSet;
use std::fmt::Debug;

pub type SystemExec<'b> = Box<dyn for<'a> RunNow<'a> + 'b>;
pub type SystemExecSend<'b> = Box<dyn for<'a> RunNow<'a> + Send + 'b>;

// TODO - Schedulable Item
// struct ScheduleItem { system: SystemExec, run_if: ... }

// TODO - Schedulable
// pub trait IntoRunnable {
//     type Output: for<'a> RunNow<'a>;

//     fn into_runnable(self) -> Box<Self::Output>;
// }

// impl<S: for<'a> System<'a>> IntoRunnable for S {
//     type Output = S;

//     fn into_runnable(self) -> Box<Self::Output> {
//         Box::new(self)
//     }
// }

struct FnRunnable {
    func: Box<dyn Fn(&World) -> () + 'static>,
}

impl FnRunnable {
    fn new(f: impl Fn(&World) -> () + 'static) -> Self {
        FnRunnable { func: Box::new(f) }
    }
}

impl<'a> RunNow<'a> for FnRunnable {
    fn run_now(&mut self, world: &World) {
        (self.func)(world);
    }
}

// impl<F> IntoRunnable for F
// where
//     F: Fn(&World) -> () + 'static,
// {
//     type Output = FnRunnable;

//     fn into_runnable(self) -> Box<Self::Output> {
//         Box::new(FnRunnable {
//             func: Box::new(self),
//         })
//     }
// }

// impl<T> Scheduleable for T where T: for<'a> RunNow<'a> { ... }

pub trait RunCondition {
    fn setup(&mut self, world: &mut World) {
        let _ = world;
    }

    fn should_run(&mut self, world: &mut World) -> bool;
}

impl<F> RunCondition for F
where
    F: Fn(&mut World) -> bool,
{
    fn should_run(&mut self, world: &mut World) -> bool {
        (self)(world)
    }
}

pub fn run_stage_always(_world: &mut World) -> bool {
    true
}

pub trait RunControl {
    fn run_stage(&mut self, stage: &mut StageData, world: &mut World);
}

impl<F> RunControl for F
where
    F: Fn(&mut StageData, &mut World) -> (),
{
    fn run_stage(&mut self, stage: &mut StageData, world: &mut World) {
        self(stage, world);
    }
}

pub fn run_stage_once(stage: &mut StageData, world: &mut World) {
    if stage.maintain_before {
        world.maintain();
    }
    if stage.run_if.should_run(world) {
        for step in stage.steps.iter_mut() {
            step.run_now(world);
        }
    }
    if stage.maintain_after {
        world.maintain();
    }
}

#[derive(Default)]
pub struct StageSet<'b> {
    systems: Vec<SystemExecSend<'b>>,
    reads: HashSet<ResourceId>,
    writes: HashSet<ResourceId>,
}

impl<'b> StageSet<'b> {
    pub fn new(system: SystemExecSend<'b>) -> Self {
        let mut set = StageSet::default();
        set.add_system(system);
        set
    }

    pub fn conflicts(&self, system: &SystemExecSend<'b>) -> bool {
        self.writes.is_disjoint(&system.reads()) && self.reads.is_disjoint(&system.writes())
    }

    pub fn add_system(&mut self, system: SystemExecSend<'b>) {
        for read in system.reads().into_iter() {
            self.reads.insert(read);
        }
        for write in system.writes().into_iter() {
            self.writes.insert(write);
        }
        self.systems.push(system);
    }

    pub fn setup(&mut self, world: &mut World) {
        for sys in self.systems.iter_mut() {
            sys.setup(world)
        }
    }

    pub fn run_now(&mut self, world: &World) {
        for sys in self.systems.iter_mut() {
            sys.run_now(world);
        }
    }
}

pub enum StageStep<'b> {
    ThreadLocal(SystemExec<'b>),
    Set(StageSet<'b>),
    Maintain,
}

impl<'b> StageStep<'b> {
    pub fn conflicts(&self, system: &SystemExecSend<'b>) -> bool {
        match self {
            StageStep::Set(set) => set.conflicts(system),
            _ => true,
        }
    }

    pub fn add_system(&mut self, system: SystemExecSend<'b>) {
        match self {
            StageStep::Set(set) => set.add_system(system),
            _ => panic!("Cannot add system to this type of step"),
        }
    }

    pub fn setup(&mut self, world: &mut World) {
        match self {
            StageStep::Maintain => {}
            StageStep::ThreadLocal(sys) => sys.setup(world),
            StageStep::Set(set) => {
                set.setup(world);
            }
        }
    }

    pub fn run_now(&mut self, world: &mut World) {
        match self {
            StageStep::Maintain => world.maintain(),
            StageStep::ThreadLocal(sys) => sys.run_now(world),
            StageStep::Set(set) => {
                set.run_now(world);
            }
        }
    }
}

pub struct StageData<'b> {
    pub id: String,
    pub steps: Vec<StageStep<'b>>,
    pub run_if: Box<dyn RunCondition>,
    pub maintain_before: bool,
    pub maintain_after: bool,
}

impl<'b> StageData<'b> {
    pub fn new(id: &str) -> Self {
        StageData {
            id: id.to_string(),
            steps: Vec::new(),
            run_if: Box::new(run_stage_always),
            maintain_before: false,
            maintain_after: false,
        }
    }

    // TODO - Group these into SystemSets that can be run in parallel
    //      - Check the reads and writes to make sure there are no conflicts
    pub fn add_system(&mut self, system: SystemExecSend<'b>) {
        for step in self.steps.iter_mut() {
            if !step.conflicts(&system) {
                step.add_system(system);
                return;
            }
        }
        self.steps.push(StageStep::Set(StageSet::new(system)));
    }

    pub fn add_local_system(&mut self, system: SystemExec<'b>) {
        self.steps.push(StageStep::ThreadLocal(system));
    }
}

pub struct Stage<'b> {
    control: Box<dyn RunControl>,
    data: StageData<'b>,
}

impl<'b> Stage<'b> {
    pub fn new(id: &str) -> Self {
        Stage {
            control: Box::new(run_stage_once),
            data: StageData::new(id),
        }
    }

    pub fn id(&self) -> &str {
        self.data.id.as_str()
    }

    pub fn add_system<T: for<'a> RunNow<'a> + Send + 'b>(&mut self, system: T) -> &mut Self {
        let system = Box::new(system);
        self.data.add_system(system);
        self
    }

    pub fn add_local_fn<F: Fn(&World) -> () + 'static>(&mut self, f: F) -> &mut Self {
        let system = Box::new(FnRunnable::new(f));
        self.data.add_local_system(system);
        self
    }

    pub fn add_local_system<T: for<'a> RunNow<'a> + 'b>(&mut self, system: T) -> &mut Self {
        let system = Box::new(system);
        self.data.add_local_system(system);
        self
    }

    // TODO - Support schedule.add_system_chain("UPDATE", (a,b));

    pub fn run_if<C: RunCondition + 'static>(&mut self, c: C) -> &mut Self {
        self.data.run_if = Box::new(c);
        self
    }

    pub fn control<C: RunControl + 'static>(&mut self, c: C) -> &mut Self {
        self.control = Box::new(c);
        self
    }

    pub fn do_maintain_before(&mut self) -> &mut Self {
        self.data.maintain_before = true;
        self
    }

    pub fn do_maintain_after(&mut self) -> &mut Self {
        self.data.maintain_after = true;
        self
    }

    // RUN

    pub fn setup(&mut self, world: &mut World) {
        for step in self.data.steps.iter_mut() {
            step.setup(world);
        }
    }

    pub fn run(&mut self, world: &mut World) {
        let Stage { control, data } = self;
        control.run_stage(data, world);
    }
}

impl<'b> Debug for Stage<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stage({}, len={})", self.data.id, self.data.steps.len())
    }
}

#[derive(Debug)]
pub struct Schedule<'b> {
    stages: Vec<Stage<'b>>,
}

impl<'b> Schedule<'b> {
    pub fn new() -> Self {
        let mut schedule = Schedule::empty();
        schedule.push_stage(Stage::new("PRE_START"));
        schedule.push_stage(Stage::new("START"));
        schedule.push_stage(Stage::new("POST_START"));
        schedule.push_stage(Stage::new("PRE_INPUT"));
        schedule.push_stage(Stage::new("INPUT"));
        schedule.push_stage(Stage::new("POST_INPUT"));
        schedule.push_stage(Stage::new("PRE_UPDATE"));
        schedule.push_stage(Stage::new("UPDATE"));
        schedule.push_stage(Stage::new("POST_UPDATE"));
        schedule.push_stage(Stage::new("PRE_RENDER"));
        schedule.push_stage(Stage::new("RENDER"));
        schedule.push_stage(Stage::new("POST_RENDER"));
        schedule.push_stage(Stage::new("PRE_FINISH"));
        schedule.push_stage(Stage::new("FINISH"));
        schedule.push_stage(Stage::new("POST_FINISH"));
        schedule
    }

    pub fn empty() -> Self {
        Schedule { stages: Vec::new() }
    }

    // STAGES

    pub fn push_stage(&mut self, stage: Stage<'b>) {
        self.stages.push(stage);
    }

    pub fn new_stage_before(&mut self, in_stage: &str, new_stage: &str) -> &mut Stage<'b> {
        self.add_stage_before(in_stage, Stage::new(new_stage));
        self.get_stage_mut(new_stage).unwrap()
    }

    pub fn add_stage_before(&mut self, in_stage: &str, new_stage: Stage<'b>) {
        match self.stages.iter().position(|s| s.id() == in_stage) {
            None => panic!("Failed to find schedule stage: {}", in_stage),
            Some(idx) => self.stages.insert(idx, new_stage),
        }
    }

    pub fn new_stage_after(&mut self, in_stage: &str, new_stage: &str) -> &mut Stage<'b> {
        self.add_stage_after(in_stage, Stage::new(new_stage));
        self.get_stage_mut(new_stage).unwrap()
    }

    pub fn add_stage_after(&mut self, in_stage: &str, new_stage: Stage<'b>) {
        match self.stages.iter().position(|s| s.id() == in_stage) {
            None => panic!("Failed to find schedule stage: {}", in_stage),
            Some(idx) => {
                if idx + 1 == self.stages.len() {
                    self.stages.push(new_stage);
                } else {
                    self.stages.insert(idx + 1, new_stage);
                }
            }
        }
    }

    pub fn get_stage(&self, id: &str) -> Option<&Stage> {
        self.stages.iter().find(|s| s.id() == id)
    }

    pub fn get_stage_mut(&mut self, id: &str) -> Option<&mut Stage<'b>> {
        self.stages.iter_mut().find(|s| s.id() == id)
    }

    // ///////////////////////////////////
    // SYSTEMS

    pub fn with<T: for<'a> RunNow<'a> + Send + 'b>(mut self, in_stage: &str, system: T) -> Self {
        self.add_system(in_stage, system);
        self
    }

    pub fn with_local<T: for<'a> RunNow<'a> + 'b>(mut self, in_stage: &str, system: T) -> Self {
        self.add_local_system(in_stage, system);
        self
    }

    pub fn add_system<T: for<'a> RunNow<'a> + Send + 'b>(&mut self, in_stage: &str, system: T) {
        let system = Box::new(system);
        match self.get_stage_mut(in_stage) {
            None => panic!("Failed to find stage - {}", in_stage),
            Some(stage) => stage.data.add_system(system),
        }
    }

    // Making funcs that take world local is the only way to do the read/write safety correctly.
    pub fn add_local_fn<F: Fn(&World) -> () + 'static>(&mut self, in_stage: &str, f: F) {
        let system = Box::new(FnRunnable::new(f));
        match self.get_stage_mut(in_stage) {
            None => panic!("Failed to find stage - {}", in_stage),
            Some(stage) => stage.data.add_local_system(system),
        }
    }

    pub fn add_local_system<T: for<'a> RunNow<'a> + 'b>(&mut self, in_stage: &str, system: T) {
        let system = Box::new(system);
        match self.get_stage_mut(in_stage) {
            None => panic!("Failed to find stage - {}", in_stage),
            Some(stage) => stage.data.add_local_system(system),
        }
    }

    // ///////////////////////////////////

    pub fn setup(&mut self, world: &mut World) {
        for stage in self.stages.iter_mut() {
            stage.setup(world);
        }
    }

    pub fn setup_current(&mut self, ecs: &mut Ecs) {
        let world = ecs.current_world_mut();
        self.setup(world);
    }

    pub fn run(&mut self, world: &mut World) {
        for stage in self.stages.iter_mut() {
            stage.run(world);
        }
    }

    pub fn run_current(&mut self, ecs: &mut Ecs) {
        let world = ecs.current_world_mut();
        self.run(world);
    }
}

impl<'b> Default for Schedule<'b> {
    fn default() -> Self {
        Schedule::new()
    }
}

#[cfg(test)]
mod test {
    use std::sync::{atomic::AtomicUsize, Arc};

    use super::*;

    #[test]
    fn create() {
        let mut schedule = Schedule::default();

        assert!(schedule.get_stage("PRE_INPUT").is_some());
        assert!(schedule.get_stage("RENDER").is_some());
        assert!(schedule.get_stage_mut("POST_UPDATE").is_some());
        assert!(schedule.get_stage_mut("START").is_some());

        assert!(schedule.get_stage("NONE").is_none());
        assert!(schedule.get_stage_mut("NONE").is_none());
    }

    #[test]
    fn empty() {
        let mut schedule = Schedule::empty();

        assert!(schedule.get_stage("PRE_INPUT").is_none());
        assert!(schedule.get_stage("RENDER").is_none());
        assert!(schedule.get_stage_mut("POST_UPDATE").is_none());
        assert!(schedule.get_stage_mut("START").is_none());

        assert!(schedule.get_stage("NONE").is_none());
        assert!(schedule.get_stage_mut("NONE").is_none());
    }

    struct RunNow;

    #[test]
    fn run_if() {
        let mut world = World::empty(1);
        let mut schedule = Schedule::default();

        let run_if_count = Arc::new(AtomicUsize::new(0));
        let run_if_count_clone = Arc::clone(&run_if_count);
        schedule
            .new_stage_after("UPDATE", "MY_STAGE")
            .run_if(move |w: &mut World| {
                run_if_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                w.has_resource::<RunNow>()
            });

        let sys_count = Arc::new(AtomicUsize::new(0));
        let sys_count_clone = Arc::clone(&sys_count);
        schedule.add_local_fn("MY_STAGE", move |_w: &World| {
            sys_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });

        schedule.setup(&mut world);
        schedule.run(&mut world);

        assert_eq!(run_if_count.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(sys_count.load(std::sync::atomic::Ordering::Relaxed), 0);

        world.insert_resource(RunNow);

        schedule.run(&mut world);

        assert_eq!(run_if_count.load(std::sync::atomic::Ordering::Relaxed), 2);
        assert_eq!(sys_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
