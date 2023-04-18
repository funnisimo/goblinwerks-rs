use gw_ecs::{Ecs, Fetch, GlobalMut, TryGlobal};
use gw_macro::system;

struct Count(u32);

#[system]
fn increment(ecs: &Ecs) {
    let mut count = ecs.get_global_mut::<Count>().unwrap();
    count.0 += 1;
}

#[test]
fn basic_ecs() {
    let mut ecs = Ecs::new();
    ecs.insert_global(Count(0));

    increment(&ecs);

    {
        let count = ecs.get_global::<Count>().unwrap();
        assert_eq!(count.0, 1);
    }

    increment_system(&ecs);

    {
        let count = ecs.get_global::<Count>().unwrap();
        assert_eq!(count.0, 2);
    }
}

#[system]
fn increment_global(mut global: GlobalMut<Count>) {
    global.0 += 1;
}

#[test]
fn basic_global() {
    let mut ecs = Ecs::new();
    ecs.insert_global(Count(0));

    increment_global_system(&ecs);

    {
        let count = ecs.get_global::<Count>().unwrap();
        assert_eq!(count.0, 1);
    }
}

struct Age(u32);

#[system]
fn try_double(try_age: TryGlobal<Age>, mut count: GlobalMut<Count>) {
    if try_age.is_none() {
        count.0 += 1;
    }
}

#[test]
fn basic_double() {
    let mut ecs = Ecs::new();
    ecs.insert_global(Count(0));

    try_double_system(&ecs);

    {
        let count = ecs.get_global::<Count>().unwrap();
        assert_eq!(count.0, 1);
    }
}
