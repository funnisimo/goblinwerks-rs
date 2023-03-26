use gw_ecs::{Ecs, Res, ResMut};

struct Info(u32);

struct Age(u32);

fn main() {
    let mut ecs = Ecs::default();

    {
        ecs.insert(Info(4));
        let info = ecs.get::<Info>().unwrap();
        println!("Info = {}", info.0);
    }

    {
        ecs.insert(Age(5));
        let age = ecs.get::<Age>().unwrap();
        println!("Age = {}", age.0);
    }

    {
        let (mut info, age) = ecs.fetch_mut::<(ResMut<Info>, Res<Age>)>();
        info.0 = info.0 + 1;
        println!("Set: Info({}), Age({})", info.0, age.0);
    }

    {
        let (info, age) = ecs.fetch::<(Res<Info>, Res<Age>)>();
        println!("Later: Info({}), Age({})", info.0, age.0);
    }
}
