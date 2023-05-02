use gw_ecs::ecs::Ecs;
use gw_ecs::shred::DispatcherBuilder;
use gw_ecs::shred::System;
use gw_ecs::shred::{ReadRes, WriteRes};

#[derive(Debug, Default)]
struct UniqueA(u32);

// A resource usually has a `Default` implementation
// which will be used if the resource has not been added.
#[derive(Debug, Default)]
struct UniqueB(u32);

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (ReadRes<'a, UniqueA>, WriteRes<'a, UniqueB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("PrintSystem = {:?} + {:?}", &*a, &*b);

        *b = UniqueB(32); // We can mutate UniqueB here
                          // because it's `Write`.
    }
}

fn main() {
    let mut ecs = Ecs::default();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();
    dispatcher.setup(ecs.current_world_mut());

    // Dispatch as often as you want to
    dispatcher.dispatch(ecs.current_world());
    dispatcher.dispatch(ecs.current_world());
    // ...
}
