use gw_ecs::{
    schedule::Schedule,
    shred::{SetupDefault, SetupHandler},
    ReadRes, System, World, WriteRes,
};

#[derive(Debug, Default)]
struct ResA;

// `ResB` does not implement `Default`.
#[derive(Debug)]
struct ResB;

struct ResWithoutSensibleDefault {
    magic_number_that_we_cant_compute: u32,
}

impl SetupHandler<ResWithoutSensibleDefault> for ResWithoutSensibleDefault {
    fn setup(world: &mut World) {
        let res = ResWithoutSensibleDefault {
            magic_number_that_we_cant_compute: 32,
        };
        world.insert_resource(res);
    }
}

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    // We can simply use `Option<Read>` or `Option<Write>` if a resource
    // isn't strictly required or can't be created (by a `Default` implementation).
    type SystemData = (
        ReadRes<'a, ResA, SetupDefault>,
        Option<WriteRes<'a, ResB, SetupDefault>>,
        // WARNING: using `ReadExpect` might lead to a panic!
        // If `ResWithoutSensibleDefault` does not exist, fetching will `panic!`.
        ReadRes<'a, ResWithoutSensibleDefault, ResWithoutSensibleDefault>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b, expected) = data;

        println!("{:?}", &*a);

        if let Some(ref mut x) = b {
            println!("{:?}", &**x);

            **x = ResB;
        }

        println!(
            "Yeah, we have our magic number: {}",
            expected.magic_number_that_we_cant_compute
        );
    }
}

fn main() {
    let mut resources = World::empty("MAIN");
    let mut dispatcher = Schedule::new().with("UPDATE", PrintSystem); // Adds a system "print" without dependencies

    // Will automatically insert `ResB` (the only one that has a default provider).
    dispatcher.setup(&mut resources);
    resources.insert_resource(ResWithoutSensibleDefault {
        magic_number_that_we_cant_compute: 42,
    });

    // `ResB` is not in resources, but `PrintSystem` still works.
    dispatcher.run(&mut resources);

    resources.insert_resource(ResB);

    // Now `ResB` can be printed, too.
    dispatcher.run(&mut resources);
}
