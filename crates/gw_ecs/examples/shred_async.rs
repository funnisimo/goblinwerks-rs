use gw_ecs::{DispatcherBuilder, ReadRes, ResourceId, System, SystemData, World, WriteRes};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[derive(SystemData)]
struct Data<'a> {
    a: ReadRes<'a, ResA>,
    b: WriteRes<'a, ResB>,
}

struct EmptySystem(*mut i8); // System is not thread-safe

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("thread local: {:?}", &*bundle.a);
        println!("thread local: {:?}", &*bundle.b);
    }
}

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (ReadRes<'a, ResA>, WriteRes<'a, ResB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("{:?}", &*a);
        println!("{:?}", &*b);

        // We can mutate ResB here because it's `Write`.
        *b = ResB;
    }
}

fn main() {
    let mut x = 5;

    let mut resources = World::empty();
    resources.insert_resource(ResA);
    resources.insert_resource(ResB);
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .with_thread_local(EmptySystem(&mut x))
        .build_async(resources);

    dispatcher.dispatch();
    dispatcher.wait();

    dispatcher.dispatch();
    dispatcher.wait();
}
