use gw_ecs::prelude::*;

#[derive(Debug, Default)]
struct ResA(u32);

#[derive(Debug, Default)]
struct ResB(u32);

#[derive(SystemParam)]
struct ExampleBundle<'w> {
    a: ReadUnique<'w, ResA>,
    b: WriteUnique<'w, ResB>,
}

fn main() {
    let mut world = World::default();
    world.insert_resource(ResA(12));
    world.insert_resource(ResB(0));

    world.exec(|mut bundle: ExampleBundle| bundle.b.0 = bundle.a.0);

    let res_a = world.read_resource::<ResA>();
    let res_b = world.read_resource::<ResB>();
    assert_eq!(res_a.0, res_b.0);
}
