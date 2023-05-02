use gw_ecs::{ReadRes, ResourceId, SystemData, UnsafeWorld, World, WriteRes};

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

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<ResA>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<ResB>()]
    }
}

fn main() {
    let mut res = World::empty();
    res.insert_resource(ResA);
    res.insert_resource(ResB);

    let mut bundle = ExampleBundle::fetch(&res);
    *bundle.b = ResB;

    println!("{:?}", *bundle.a);
    println!("{:?}", *bundle.b);
}
