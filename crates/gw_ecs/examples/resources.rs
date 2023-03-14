use gw_ecs::resources::{Mut, Ref, ResourceSet, Resources};

struct Info(u32);

struct Age(u32);

fn main() {
    let mut res = Resources::default();

    {
        res.insert(Info(4));
        let info = res.get::<Info>().unwrap();
        println!("Info = {}", info.0);
    }

    {
        res.insert(Age(5));
        let age = res.get::<Age>().unwrap();
        println!("Age = {}", age.0);
    }

    {
        let (mut info, age) = <(Mut<Info>, Ref<Age>)>::fetch_mut(&mut res);
        info.0 = info.0 + 1;
        println!("Set: Info({}), Age({})", info.0, age.0);
    }

    {
        let (info, age) = <(Ref<Info>, Ref<Age>)>::fetch(&res);
        println!("Later: Info({}), Age({})", info.0, age.0);
    }
}
