use gw_ecs::ecs::Ecs;
use gw_ecs::schedule::Schedule;
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
    #[allow(dead_code)]
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
    let mut dispatcher = Schedule::new()
        .with("UPDATE", PrintSystem) // Adds a system "print" without dependencies
        // .with(NotSyncSystem, "not_sync", &[]) // Adds a system "print" without dependencies
        .with_local("UPDATE", NotSyncSystem);
    dispatcher.setup(ecs.current_world_mut());

    // Dispatch as often as you want to
    dispatcher.run(ecs.current_world_mut());
    dispatcher.run(ecs.current_world_mut());
    // ...
}
