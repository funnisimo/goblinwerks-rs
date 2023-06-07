use gw_bevy::{globals::ReadNonSendGlobal, prelude::*};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[derive(SystemParam)]
struct Data<'w> {
    a: ReadUnique<'w, ResA>,
    b: WriteUnique<'w, ResB>,
    c: ReadNonSendGlobal<'w, MyUnsafe>,
}

#[derive(Debug)]
struct MyUnsafe(*const i8); // System is not thread-safe

impl Default for MyUnsafe {
    fn default() -> Self {
        MyUnsafe(std::ptr::null())
    }
}

fn non_send_system(bundle: Data) {
    println!(
        "non_send_system - a: {:?}, b: {:?}, c:{:?}",
        &*bundle.a, &*bundle.b, &*bundle.c
    );
}

fn print_system(a: ReadUnique<ResA>, mut b: WriteUnique<ResB>) {
    println!("{:?}", &*a);
    println!("{:?}", &*b);

    // We can mutate ResB here because it's `Write`.
    *b = ResB;
}

fn main() {
    let mut world = World::default();
    world.insert_resource(ResA);
    world.insert_resource(ResB);
    world.ensure_global_non_send::<MyUnsafe>();

    let mut schedule = Schedule::new(); // Scheduler is MultiThreaded by default
    schedule.add_system(print_system);
    schedule.add_system(non_send_system);

    schedule.run(&mut world);
    schedule.run(&mut world);
}
