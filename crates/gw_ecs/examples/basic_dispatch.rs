use gw_ecs::ecs::Ecs;
use gw_ecs::schedule::Schedule;
use gw_ecs::shred::System;
use gw_ecs::shred::{ReadRes, WriteRes};

#[derive(Debug, Default)]
struct UniqueA;

// A resource usually has a `Default` implementation
// which will be used if the resource has not been added.
#[derive(Debug, Default)]
struct UniqueB;

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (ReadRes<'a, UniqueA>, WriteRes<'a, UniqueB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("PrintSystem = {:?} + {:?}", &*a, &*b);

        *b = UniqueB; // We can mutate UniqueB here
                      // because it's `Write`.
    }
}

fn main() {
    let mut ecs = Ecs::default();
    let mut schedule = Schedule::new();
    schedule.add_system("UPDATE", PrintSystem);

    schedule.setup(ecs.current_world_mut());

    // Dispatch as often as you want to
    schedule.run(ecs.current_world_mut());
    schedule.run(ecs.current_world_mut());
    // ...
}
