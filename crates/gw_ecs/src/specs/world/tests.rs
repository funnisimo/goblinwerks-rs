// use super::{WorldExt, *};
use crate::specs::world::{Builder, EntitiesRes};
use crate::specs::{join::Join, storage::VecStorage};
use crate::specs::{Component, LazyUpdate};
use crate::World;

struct Pos;

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct Vel;

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[test]
fn delete_all() {
    let mut world = World::default();

    world.register::<Pos>();
    world.register::<Vel>();

    world.create_entity().build();
    let b = world.create_entity().with(Pos).with(Vel).build();
    world.create_entity().with(Pos).with(Vel).build();

    assert_eq!(world.entities().join().count(), 3);

    world.delete_all();

    assert_eq!(world.entities().join().count(), 0);
    assert!(world.read_component::<Pos>().get(b).is_none());
}

#[test]
fn lazy_insertion() {
    let mut world = World::default();
    world.register::<Pos>();
    world.register::<Vel>();

    let e1;
    let e2;
    {
        let entities = world.fetch::<EntitiesRes>();
        let lazy = world.fetch::<LazyUpdate>();

        e1 = entities.create();
        e2 = entities.create();
        lazy.insert(e1, Pos);
        lazy.insert_all(vec![(e1, Vel), (e2, Vel)]);
    }

    world.maintain();
    assert!(world.read_component::<Pos>().get(e1).is_some());
    assert!(world.read_component::<Vel>().get(e1).is_some());
    assert!(world.read_component::<Vel>().get(e2).is_some());
}

#[test]
fn lazy_removal() {
    let mut world = World::default();
    world.register::<Pos>();

    let e = world.create_entity().with(Pos).build();
    {
        let lazy = world.fetch::<LazyUpdate>();
        lazy.remove::<Pos>(e);
    }

    world.maintain();
    assert!(world.read_component::<Pos>().get(e).is_none());
}

#[test]
fn super_lazy_execution() {
    let mut world = World::default();
    world.register::<Pos>();

    let e = {
        let entity_res = world.fetch::<EntitiesRes>();
        entity_res.create()
    };
    world.fetch::<LazyUpdate>().exec(move |world| {
        world.fetch::<LazyUpdate>().exec(move |world| {
            if let Err(err) = world.write_component::<Pos>().insert(e, Pos) {
                panic!("Unable to lazily insert component! {:?}", err);
            }
        });
        assert!(world.read_component::<Pos>().get(e).is_none());
    });
    world.maintain();
    assert!(world.read_component::<Pos>().get(e).is_some());
}

#[test]
fn lazy_execution() {
    let mut world = World::default();
    world.register::<Pos>();

    let e = {
        let entity_res = world.fetch::<EntitiesRes>();
        entity_res.create()
    };
    {
        let lazy = world.fetch::<LazyUpdate>();
        lazy.exec(move |world| {
            if let Err(err) = world.write_component::<Pos>().insert(e, Pos) {
                panic!("Unable to lazily insert component! {:?}", err);
            }
        });
    }

    world.maintain();
    assert!(world.read_component::<Pos>().get(e).is_some());
}

#[test]
fn lazy_execution_order() {
    let mut world = World::default();
    world.insert(Vec::<u32>::new());
    {
        let lazy = world.fetch::<LazyUpdate>();
        lazy.exec(move |world| {
            let mut v = world.fetch_mut::<Vec<u32>>();
            v.push(1);
        });
        lazy.exec(move |world| {
            let mut v = world.fetch_mut::<Vec<u32>>();
            v.push(2);
        });
    }
    world.maintain();
    let v = world.fetch::<Vec<u32>>();
    assert_eq!(&**v, &[1, 2]);
}

#[test]
fn delete_twice() {
    let mut world = World::default();

    let e = world.create_entity().build();

    world.delete_entity(e).unwrap();
    assert!(world.entities().delete(e).is_err());
}

#[test]
fn delete_and_lazy() {
    let mut world = World::default();
    {
        let lazy_update = world.fetch_mut::<crate::specs::LazyUpdate>();
        lazy_update.exec(|world| {
            world.entities().create();
        })
    }

    world.maintain();
    {
        let lazy_update = world.fetch_mut::<crate::specs::LazyUpdate>();
        lazy_update.exec(|world| {
            world.entities().create();
        })
    }

    world.delete_all();
}