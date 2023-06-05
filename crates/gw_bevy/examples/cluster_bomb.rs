use gw_bevy::prelude::*;
use gw_bevy::storage::{HashMapStorage, VecStorage};
use rand::prelude::*;

const TAU: f32 = 2. * std::f32::consts::PI;

#[derive(Debug)]
struct ClusterBomb {
    fuse: usize,
}
impl Component for ClusterBomb {
    // This uses `HashMapStorage`, because only some entities are cluster bombs.
    type Storage = HashMapStorage<Self>;
}

#[derive(Debug)]
struct Shrapnel {
    durability: usize,
}
impl Component for Shrapnel {
    // This uses `HashMapStorage`, because only some entities are shrapnels.
    type Storage = HashMapStorage<Self>;
}

#[derive(Debug, Clone)]
struct Pos(f32, f32);
impl Component for Pos {
    // This uses `VecStorage`, because all entities have a position.
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Component)]
struct Vel(f32, f32);

fn cluster_bomb_system(
    entities: Entities,
    mut bombs: WriteComp<ClusterBomb>,
    positions: ReadComp<Pos>,
    mut updater: Commands,
) {
    use rand::distributions::Uniform;

    let durability_range = Uniform::new(10, 20);
    let update_position =
        |(entity, mut bomb, position): (Entity, CompMut<ClusterBomb>, CompRef<Pos>)| {
            let mut rng = rand::thread_rng();

            if bomb.fuse == 0 {
                let _ = entities.delete(entity);
                for _ in 0..9 {
                    let shrapnel = entities.create();
                    let mut update = updater.entity(shrapnel);
                    update.insert(Shrapnel {
                        durability: durability_range.sample(&mut rng),
                    });
                    update.insert(position.clone());
                    let angle: f32 = rng.gen::<f32>() * TAU;
                    update.insert(Vel(angle.sin(), angle.cos()));
                }
            } else {
                bomb.fuse -= 1;
            }
        };

    // Join components in potentially parallel way using rayon.
    {
        (&entities, &mut bombs, &positions)
            .join()
            .for_each(update_position);
    }
}

fn physics_system(mut pos: WriteComp<Pos>, vel: ReadComp<Vel>) {
    (&mut pos, &vel).join().for_each(|(mut pos, vel)| {
        pos.0 += vel.0;
        pos.1 += vel.1;
    });
}

fn shrapnel_system(entities: Entities, mut shrapnels: WriteComp<Shrapnel>) {
    (&entities, &mut shrapnels)
        .join()
        .for_each(|(entity, mut shrapnel)| {
            if shrapnel.durability == 0 {
                let _ = entities.delete(entity);
            } else {
                shrapnel.durability -= 1;
            }
        });
}

fn main() {
    let mut world = World::empty(0);
    world.register::<Shrapnel>();
    world.register::<ClusterBomb>();
    world.register::<Pos>();
    world.register::<Vel>();

    let mut dispatcher = Schedule::new();

    dispatcher.add_systems((physics_system, cluster_bomb_system, shrapnel_system).chain());

    world
        .create_entity()
        .with(Pos(0., 0.))
        .with(ClusterBomb { fuse: 3 })
        .id();

    let mut step = 0;
    loop {
        step += 1;
        let mut entities = 0;
        {
            // Simple console rendering
            let positions = world.read_component::<Pos>();
            const WIDTH: usize = 10;
            const HEIGHT: usize = 10;
            const SCALE: f32 = 1. / 4.;
            let mut screen = [[0; WIDTH]; HEIGHT];
            for entity in world.entities().join() {
                if let Some(pos) = positions.get(entity) {
                    let x = (pos.0 * SCALE + WIDTH as f32 / 2.).floor() as usize;
                    let y = (pos.1 * SCALE + HEIGHT as f32 / 2.).floor() as usize;
                    if x < WIDTH && y < HEIGHT {
                        screen[x][y] += 1;
                    }
                }
                entities += 1;
            }
            println!("Step: {}, Entities: {}", step, entities);
            for row in &screen {
                for cell in row {
                    print!("{}", cell);
                }
                println!();
            }
            println!();
        }
        if entities == 0 {
            break;
        }

        dispatcher.run(&mut world);

        world.maintain();
    }
}
