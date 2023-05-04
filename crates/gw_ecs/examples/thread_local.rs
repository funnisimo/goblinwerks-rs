use gw_ecs::{DispatcherBuilder, ReadRes, ResourceId, System, SystemData, World, WriteRes};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[cfg(feature = "derive")]
#[derive(SystemData)]
struct Data<'a> {
    a: ReadRes<'a, ResA>,
    b: WriteRes<'a, ResB>,
}

struct EmptySystem(*mut i8); // System is not thread-safe

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut x = 5;

    let mut resources = World::empty(0);
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(EmptySystem(&mut x))
        .build();
    dispatcher.setup(&mut resources);

    dispatcher.dispatch(&resources);
}

// The following is required for the example to compile without the
// `shred-derive` feature.

#[cfg(not(feature = "derive"))]
struct Data<'a> {
    a: ReadRes<'a, ResA>,
    b: WriteRes<'a, ResB>,
}

#[cfg(not(feature = "derive"))]
impl<'a> SystemData<'a> for Data<'a> {
    fn setup(world: &mut World) {
        ReadRes::<'_, ResA>::setup(world);
        WriteRes::<'_, ResB>::setup(world);
    }

    fn fetch(world: &'a World) -> Self {
        Self {
            a: ReadRes::<'_, ResA>::fetch(world),
            b: WriteRes::<'_, ResB>::fetch(world),
        }
    }

    fn reads() -> Vec<ResourceId> {
        ReadRes::<'_, ResA>::reads()
    }

    fn writes() -> Vec<ResourceId> {
        WriteRes::<'_, ResB>::writes()
    }
}
