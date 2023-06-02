use gw_bevy::{
    prelude::*,
    schedule::{ExecutorKind, ScheduleBuildSettings},
};
use rand::*;
use tracing::{info, trace, Level};
use tracing_subscriber::FmtSubscriber;

///////////////////////////////////////////////////////////////////////////

/// The names of the default [`App`] schedules.
///
/// The corresponding [`Schedule`](bevy_ecs::schedule::Schedule) objects are added by [`App::add_default_schedules`].
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoreSchedule {
    /// The schedule that runs once when the app starts.
    Startup,
    /// The schedule that contains the app logic that is evaluated each tick of [`App::update()`].
    Main,
    /// The schedule that controls which schedules run.
    ///
    /// This is typically created using the [`CoreSchedule::outer_schedule`] method,
    /// and does not need to manipulated during ordinary use.
    Outer,
    /// The schedule that contains systems which only run after a fixed period of time has elapsed.
    ///
    /// The exclusive `run_fixed_update_schedule` system runs this schedule during the [`CoreSet::FixedUpdate`] system set.
    FixedUpdate,
}

impl CoreSchedule {
    /// An exclusive system that controls which schedule should be running.
    ///
    /// [`CoreSchedule::Main`] is always run.
    ///
    /// If this is the first time this system has been run, [`CoreSchedule::Startup`] will run before [`CoreSchedule::Main`].
    pub fn outer_loop(world: &mut World, mut run_at_least_once: Local<bool>) {
        if !*run_at_least_once {
            world.run_schedule(CoreSchedule::Startup);
            *run_at_least_once = true;
        }

        world.run_schedule(CoreSchedule::Main);
    }

    /// Initializes a single threaded schedule for [`CoreSchedule::Outer`] that contains the [`outer_loop`](CoreSchedule::outer_loop) system.
    pub fn outer_schedule() -> Schedule {
        let mut schedule = Schedule::new();
        schedule.set_executor_kind(gw_bevy::schedule::ExecutorKind::SingleThreaded);
        schedule.add_system(Self::outer_loop);
        schedule
    }
}

/// The names of the default [`App`] system sets.
///
/// These are ordered in the same order they are listed.
///
/// The corresponding [`SystemSets`](bevy_ecs::schedule::SystemSet) are added by [`App::add_default_schedules`].
///
/// The `*Flush` sets are assigned to the copy of [`apply_system_buffers`]
/// that runs immediately after the matching system set.
/// These can be useful for ordering, but you almost never want to add your systems to these sets.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum CoreSet {
    /// Runs before all other members of this set.
    First,
    /// The copy of [`apply_system_buffers`] that runs immediately after `First`.
    FirstFlush,
    /// Runs before [`CoreSet::Update`].
    PreUpdate,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PreUpdate`.
    PreUpdateFlush,
    /// Applies [`State`](bevy_ecs::schedule::State) transitions
    StateTransitions,
    /// Runs systems that should only occur after a fixed period of time.
    ///
    /// The `run_fixed_update_schedule` system runs the [`CoreSchedule::FixedUpdate`] system in this system set.
    FixedUpdate,
    /// Responsible for doing most app logic. Systems should be registered here by default.
    Update,
    /// The copy of [`apply_system_buffers`] that runs immediately after `Update`.
    UpdateFlush,
    /// Runs after [`CoreSet::Update`].
    PostUpdate,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PostUpdate`.
    PostUpdateFlush,
    /// Runs after all other members of this set.
    Last,
    /// The copy of [`apply_system_buffers`] that runs immediately after `Last`.
    LastFlush,
}

impl CoreSet {
    /// Sets up the base structure of [`CoreSchedule::Main`].
    ///
    /// The sets defined in this enum are configured to run in order,
    /// and a copy of [`apply_system_buffers`] is inserted into each `*Flush` set.
    pub fn base_schedule() -> Schedule {
        use CoreSet::*;
        let mut schedule = Schedule::new();

        // Create "stage-like" structure using buffer flushes + ordering
        schedule
            .set_default_base_set(Update)
            .add_system(apply_system_buffers.in_base_set(FirstFlush))
            .add_system(apply_system_buffers.in_base_set(PreUpdateFlush))
            .add_system(apply_system_buffers.in_base_set(UpdateFlush))
            .add_system(apply_system_buffers.in_base_set(PostUpdateFlush))
            .add_system(apply_system_buffers.in_base_set(LastFlush))
            .configure_sets(
                (
                    First,
                    FirstFlush,
                    PreUpdate,
                    PreUpdateFlush,
                    StateTransitions,
                    FixedUpdate,
                    Update,
                    UpdateFlush,
                    PostUpdate,
                    PostUpdateFlush,
                    Last,
                    LastFlush,
                )
                    .chain(),
            );
        schedule
    }
}

/// The names of the default [`App`] startup sets, which live in [`CoreSchedule::Startup`].
///
/// The corresponding [`SystemSets`](bevy_ecs::schedule::SystemSet) are added by [`App::add_default_schedules`].
///
/// The `*Flush` sets are assigned to the copy of [`apply_system_buffers`]
/// that runs immediately after the matching system set.
/// These can be useful for ordering, but you almost never want to add your systems to these sets.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum StartupSet {
    /// Runs once before [`StartupSet::Startup`].
    PreStartup,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PreStartup`.
    PreStartupFlush,
    /// Runs once when an [`App`] starts up.
    Startup,
    /// The copy of [`apply_system_buffers`] that runs immediately after `Startup`.
    StartupFlush,
    /// Runs once after [`StartupSet::Startup`].
    PostStartup,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PostStartup`.
    PostStartupFlush,
}

impl StartupSet {
    /// Sets up the base structure of [`CoreSchedule::Startup`].
    ///
    /// The sets defined in this enum are configured to run in order,
    /// and a copy of [`apply_system_buffers`] is inserted into each `*Flush` set.
    pub fn base_schedule() -> Schedule {
        use StartupSet::*;
        let mut schedule = Schedule::new();
        schedule.set_default_base_set(Startup);

        // Create "stage-like" structure using buffer flushes + ordering
        schedule.add_system(apply_system_buffers.in_base_set(PreStartupFlush));
        schedule.add_system(apply_system_buffers.in_base_set(StartupFlush));
        schedule.add_system(apply_system_buffers.in_base_set(PostStartupFlush));

        schedule.configure_set(PreStartup.before(PreStartupFlush));
        schedule.configure_set(Startup.after(PreStartupFlush).before(StartupFlush));
        schedule.configure_set(PostStartup.after(StartupFlush).before(PostStartupFlush));

        schedule
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Component)]
struct Stock {
    name: &'static str,
    price: f32,
}

impl Stock {
    pub fn new(name: &'static str) -> Self {
        Stock { name, price: 10.00 }
    }
}

struct Split(Entity);

fn split_system(mut splits: EventReader<Split>, mut stocks: WriteComp<Stock>) {
    for ev in splits.iter() {
        if let Some(stock) = stocks.get_mut(ev.0) {
            let was = stock.price;
            stock.price = stock.price / 2.0;
            println!("Split - {} : {:.2} -> {:.2}", stock.name, was, stock.price);
        }
    }
}

struct BuyBack(Entity);

fn buy_back_system(mut buy_backs: EventReader<BuyBack>, mut stocks: WriteComp<Stock>) {
    for ev in buy_backs.iter() {
        if let Some(stock) = stocks.get_mut(ev.0) {
            let was = stock.price;
            stock.price = stock.price * 2.0;
            println!(
                "Buy Back - {} : {:.2} -> {:.2}",
                stock.name, was, stock.price
            );
        }
    }
}

fn price_change_system(
    entities: Entities,
    mut stocks: WriteComp<Stock>,
    mut buy_backs: EventWriter<BuyBack>,
    mut splits: EventWriter<Split>,
) {
    let mut rng = thread_rng();

    for (entity, stock) in (&entities, &mut stocks).join() {
        let delta = 1.0 - ((rng.next_u32() as f32 / u32::MAX as f32) * 2.0); // -1.0 -> 1.0
        stock.price += delta;

        if stock.price <= 5.0 {
            buy_backs.send(BuyBack(entity));
        } else if stock.price >= 20.0 {
            splits.send(Split(entity));
        }
    }
}

fn print_system(stocks: ReadComp<Stock>) {
    println!("[==================]");
    for stock in stocks.join() {
        println!("{} - {:.2}", stock.name, stock.price);
    }
}

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Hello");

    let mut world = World::default();

    // world.register::<Stock>();
    world.register_event::<BuyBack>();
    world.register_event::<Split>();

    let mut schedule = CoreSet::base_schedule();
    // schedule.set_executor_kind(ExecutorKind::SingleThreaded);

    schedule.add_system(price_change_system.in_base_set(CoreSet::Update));
    schedule.add_system(
        buy_back_system
            .after(price_change_system)
            .in_base_set(CoreSet::PostUpdate),
    );
    schedule.add_system(
        split_system
            .after(buy_back_system)
            .in_base_set(CoreSet::PostUpdate),
    );
    schedule.add_system(print_system.in_base_set(CoreSet::Last));

    for _ in 0..100 {
        schedule.run(&mut world);
        world.maintain();
    }

    println!("<<DONE>>");
}
