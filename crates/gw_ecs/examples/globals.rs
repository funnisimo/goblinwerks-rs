use gw_ecs::Ecs;
use gw_ecs::{BorrowMut, BorrowRef, Global, GlobalMut};

struct Info(u32);

struct Age(u32);

fn main() {
    let mut ecs = Ecs::new();

    {
        // Insert a global
        ecs.insert_global(Info(4));
        let info = ecs.get_global::<Info>().unwrap();
        println!("Info = {}", info.0);
    }

    {
        // Insert another
        ecs.insert_global(Age(5));
        let age = Global::<Age>::borrow(&mut ecs);
        println!("Age = {}", age.0);
    }

    {
        // // Increment the info
        // let (mut info, age) = ecs.fetch_mut::<(GlobalMut<Info>, Global<Age>)>();
        let mut info = GlobalMut::<Info>::borrow_mut(&ecs);
        info.0 = info.0 + 1;

        println!("After increment: Info({})", info.0);
    }

    // {
    //     // Read it out again
    //     let (info, age) = ecs.fetch::<(Global<Info>, Global<Age>)>();
    //     println!("Later: Info({}), Age({})", info.0, age.0);
    // }

    // {
    //     let (mut info, mut age) = ecs.fetch_mut::<(GlobalMut<Info>, GlobalMut<Age>)>();
    //     info.0 += 1;
    //     age.0 += 10;
    //     println!("Later: Info({}), Age({})", info.0, age.0);
    // }
}
