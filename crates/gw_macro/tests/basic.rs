use gw_ecs::{Ecs, Fetch};
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
