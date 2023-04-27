use gw_specs::ecs::Ecs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct GlobalA(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct GlobalB(u32);

fn main() {
    let mut ecs = Ecs::new();

    assert!(ecs.try_get_global::<GlobalA>().is_none());
    assert!(!ecs.has_global::<GlobalA>());

    ecs.insert_global(GlobalA(32));

    assert!(ecs.has_global::<GlobalA>());
    assert_eq!(ecs.get_global::<GlobalA>().0, 32);

    assert!(ecs.try_get_global::<GlobalA>().is_some());
}
