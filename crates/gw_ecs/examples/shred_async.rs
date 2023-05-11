use gw_ecs::{schedule::Schedule, ReadRes, ResourceId, System, SystemData, World, WriteRes};

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

    let mut resources = World::empty(123);
    resources.insert_resource(ResA);
    resources.insert_resource(ResB);
    let mut dispatcher = Schedule::new()
        .with("UPDATE", PrintSystem) // Adds a system "print" without dependencies
        .with_local("UPDATE", EmptySystem(&mut x));

    dispatcher.run(&mut resources);
    dispatcher.run(&mut resources);
}
