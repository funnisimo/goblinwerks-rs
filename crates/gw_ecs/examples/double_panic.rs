use gw_ecs::prelude::*;
use gw_ecs_macros::Component;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
struct A(usize);
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
struct B(usize);

#[derive(Component, Copy, Clone, PartialEq, Eq, Debug)]
// #[component(storage = "Table")]
struct TableStored(&'static str);

fn main() {
    println!("RUNNING TEST");

    let mut world = World::default();
    world.register::<B>();
    world.register::<TableStored>();
    let e = world.spawn((TableStored("abc"), A(123)));
    assert!(world.write_component::<B>().remove(e).is_none());
}
