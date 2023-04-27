use gw_specs::ecs::Ecs;
use gw_specs::shred::DispatcherBuilder;
use gw_specs::shred::System;
use gw_specs::shred::{Read, Write};

#[derive(Debug, Default)]
struct UniqueA;

// A resource usually has a `Default` implementation
// which will be used if the resource has not been added.
#[derive(Debug, Default)]
struct UniqueB;

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (Read<'a, UniqueA>, Write<'a, UniqueB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("PrintSystem = {:?} + {:?}", &*a, &*b);

        *b = UniqueB; // We can mutate UniqueB here
                      // because it's `Write`.
    }
}

fn main() {
    let mut ecs = Ecs::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();
    dispatcher.setup(ecs.current_world_mut());

    // Dispatch as often as you want to
    dispatcher.dispatch(ecs.current_world());
    dispatcher.dispatch(ecs.current_world());
    // ...
}
