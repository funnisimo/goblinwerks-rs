use gw_bevy::prelude::*;
use tracing::{event, Level};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Default, Debug)]
struct GlobA(u32);

#[derive(Default, Debug)]
struct GlobB(u32);

#[derive(Default, Debug)]
struct ResA(u32);

#[derive(Default, Debug)]
struct ResB(u32);

#[derive(Component, Debug)]
struct CompA(u32);

#[derive(Component, Debug)]
struct CompB(u32);

fn system_a(mut commands: Commands) {
    commands.spawn_empty();
}

fn system_b(res_a: ReadUnique<ResA>, res_b: ReadUnique<ResB>) {
    println!("{}, {}", res_a.0, res_b.0);
}

fn system_c(comp_a: ReadComp<CompA>, comp_b: ReadComp<CompB>) {
    for (a, b) in (&comp_a, &comp_b).join() {
        println!("comp - {} + {}", a.0, b.0);
    }
}

fn system_d(mut comp_a: WriteComp<CompA>, comp_b: ReadComp<CompB>) {
    for (a, b) in (&mut comp_a, &comp_b).join() {
        println!("comp - {} + {}", a.0, b.0);
    }
}

fn system_e(comp_a: ReadComp<CompA>, mut comp_b: WriteComp<CompB>) {
    for (a, b) in (&comp_a, &mut comp_b).join() {
        println!("comp - {} + {}", a.0, b.0);
    }
}

fn main() {
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::ACTIVE)
        // build but do not install the subscriber.
        .finish();

    tracing::subscriber::set_global_default(collector).unwrap();

    event!(Level::TRACE, "HELLO");

    let mut world = World::default();
    world.register::<CompA>();
    world.register::<CompB>();
    world.ensure_global::<GlobA>();
    world.ensure_global::<GlobB>();
    world.ensure_resource::<ResA>();
    world.ensure_resource::<ResB>();

    let mut schedule = Schedule::new();
    schedule.set_executor_kind(gw_bevy::schedule::ExecutorKind::MultiThreaded);

    schedule.add_system(system_a);
    schedule.add_system(system_b);
    schedule.add_system(system_c);
    schedule.add_system(system_d);
    schedule.add_system(system_e);

    let _ = schedule.initialize(&mut world);

    schedule.run(&mut world);
}
