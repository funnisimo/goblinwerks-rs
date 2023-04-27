use gw_specs::dispatch::DispatcherBuilder;
use gw_specs::ecs::Ecs;
use gw_specs::system::System;
use gw_specs::world::{Global, GlobalMut};

#[derive(Debug, Default)]
struct GlobalA;

// A resource usually has a `Default` implementation
// which will be used if the resource has not been added.
#[derive(Debug, Default)]
struct GlobalB;

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (Global<'a, GlobalA>, GlobalMut<'a, GlobalB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("{:?}", &*a);
        println!("{:?}", &*b);

        *b = GlobalB; // We can mutate GlobalB here
                      // because it's `Write`.
    }
}

fn main() {
    let mut ecs = Ecs::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();
    dispatcher.setup(&mut ecs);

    // Dispatch as often as you want to
    dispatcher.dispatch(&ecs);
    dispatcher.dispatch(&ecs);
    // ...
}
