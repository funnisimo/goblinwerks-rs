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

#[derive(Debug)]
struct NotSync {
    ptr: *const u8,
}

impl Default for NotSync {
    fn default() -> Self {
        NotSync {
            ptr: std::ptr::null(),
        }
    }
}

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

struct NotSyncSystem;

impl<'a> System<'a> for NotSyncSystem {
    type SystemData = (ReadRes<'a, NotSync>, WriteRes<'a, UniqueB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("NotSync = {:?} + {:?}", &*a, &*b);

        *b = UniqueB(16); // We can mutate UniqueB here
                          // because it's `Write`.
    }
}

fn main() {
    let mut ecs = Ecs::default();
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        // .with(NotSyncSystem, "not_sync", &[]) // Adds a system "print" without dependencies
        .with_thread_local(NotSyncSystem)
        .build();
    dispatcher.setup(ecs.current_world_mut());

    // Dispatch as often as you want to
    dispatcher.dispatch(ecs.current_world());
    dispatcher.dispatch(ecs.current_world());
    // ...
}
