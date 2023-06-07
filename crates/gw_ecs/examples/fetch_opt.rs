use gw_ecs::prelude::*;

#[derive(Debug, Default)]
struct ResA;

// `ResB` does not implement `Default`.
#[derive(Debug)]
struct ResB;

struct ResWithoutDefault {
    magic_value: u32,
}

impl FromWorld for ResWithoutDefault {
    fn from_world(_world: &mut World) -> Self {
        ResWithoutDefault { magic_value: 32 }
    }
}

fn print_system(
    a: ReadUnique<ResA>,
    mut b: Option<WriteUnique<ResB>>,
    c: ReadUnique<ResWithoutDefault>,
) {
    println!("[PRINT SYSTEM]");
    println!("A = {:?}", &*a);

    if let Some(ref mut x) = b {
        println!("B = {:?}", &**x);

        **x = ResB;
    }

    println!("Yeah, we have our magic number: {}", c.magic_value);
}

fn main() {
    let mut world = World::default();

    world.ensure_resource::<ResA>();
    world.ensure_resource::<ResWithoutDefault>();

    let mut schedule = Schedule::new();
    schedule.add_system(print_system);

    // `ResB` is not in resources, but `PrintSystem` still works.
    schedule.run(&mut world);

    world.insert_resource(ResB);

    // Now `ResB` can be printed, too.
    schedule.run(&mut world);
}
