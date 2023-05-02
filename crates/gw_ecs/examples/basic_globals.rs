use gw_ecs::{
    ecs::Ecs,
    globals::ReadGlobal,
    shred::{DispatcherBuilder, ReadRes, System},
    World,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct UniqueA(u32);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct GlobalA(u32);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct GlobalB(u32);

struct GlobalSystem;

impl<'a> System<'a> for GlobalSystem {
    type SystemData = (
        ReadGlobal<'a, GlobalA>,
        ReadGlobal<'a, GlobalB>,
        ReadRes<'a, UniqueA>,
    );

    fn run(&mut self, (global_a, global_b, unique_a): Self::SystemData) {
        println!(
            "System = A:{:?}, B:{:?} + U:{:?}",
            global_a.0, global_b.0, unique_a.0
        );
    }
}
fn main() {
    let mut ecs = Ecs::default();

    assert!(ecs.try_read_global::<GlobalA>().is_none());
    assert!(!ecs.has_global::<GlobalA>());

    ecs.insert_global(GlobalA(32));
    ecs.insert_global(GlobalB(64));
    ecs.insert_unique(UniqueA(1));

    assert!(ecs.has_global::<GlobalA>());
    assert_eq!(ecs.read_global::<GlobalA>().0, 32);

    assert!(ecs.try_read_global::<GlobalA>().is_some());

    let mut dispatcher = DispatcherBuilder::new()
        .with(GlobalSystem, "global", &[])
        .build();

    dispatcher.dispatch(&ecs.current_world().as_unsafe());

    let mut world = World::empty();
    world.insert_resource(UniqueA(2));

    let index = ecs.push_world(world);
    ecs.set_current_index(index);

    dispatcher.dispatch(&ecs.current_world().as_unsafe());
}
