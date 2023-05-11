use std::collections::HashSet;

use gw_ecs::{ReadRes, ResourceId, SystemData, World, WriteRes};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

struct ExampleBundle<'a> {
    a: ReadRes<'a, ResA>,
    b: WriteRes<'a, ResB>,
}

impl<'a> SystemData<'a> for ExampleBundle<'a> {
    fn setup(res: &mut World) {
        res.ensure_resource::<ResA>();
        res.ensure_resource::<ResB>();
    }

    fn fetch(res: &'a World) -> Self {
        ExampleBundle {
            a: SystemData::fetch(res),
            b: SystemData::fetch(res),
        }
    }

    fn reads() -> HashSet<ResourceId> {
        let mut reads = HashSet::new();
        reads.insert(ResourceId::new::<ResA>());
        reads
    }

    fn writes() -> HashSet<ResourceId> {
        let mut writes = HashSet::new();
        writes.insert(ResourceId::new::<ResB>());
        writes
    }
}

fn main() {
    let mut res = World::empty(0);
    res.insert_resource(ResA);
    res.insert_resource(ResB);

    let mut bundle = ExampleBundle::fetch(&res);
    *bundle.b = ResB;

    println!("{:?}", *bundle.a);
    println!("{:?}", *bundle.b);
}
