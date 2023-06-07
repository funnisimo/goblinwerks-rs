use gw_bevy::{prelude::*, resources::ReadNonSendUnique};

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

fn print_system(a: ReadUnique<UniqueA>, mut b: WriteUnique<UniqueB>) {
    println!("PrintSystem = {:?} + {:?}", &*a, &*b);

    *b = UniqueB(32); // We can mutate UniqueB here
                      // because it's `Write`.
}

fn not_sync_system(not_sync: ReadNonSendUnique<NotSync>, mut b: WriteUnique<UniqueB>) {
    println!("NotSync = {:?} + {:?}", not_sync.ptr, b.0);

    *b = UniqueB(16); // We can mutate UniqueB here
                      // because it's `Write`.
}

fn main() {
    let mut ecs = Ecs::default();

    let mut schedule = Schedule::new();
    schedule.add_system(print_system);
    schedule.add_system(not_sync_system);

    let world = ecs.current_world_mut();
    world.ensure_resource::<UniqueA>();
    world.ensure_resource::<UniqueB>();
    world.ensure_resource_non_send::<NotSync>();

    // Dispatch as often as you want to
    schedule.run(world);
    world.maintain();
    schedule.run(world);
    world.maintain();
    // ...
}
