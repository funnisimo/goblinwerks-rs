use gw_ecs::{Ecs, Level, LevelMut, LevelRef, Levels, UniMut, UniRef};

// struct Info(u32);

struct Age(u32);

fn main() {
    let ecs = Ecs::new();

    {
        let mut levels = ecs.res_mut::<Levels>().unwrap();
        let mut level = Level::new();
        level.insert_unique(Age(10));
        levels.insert(level);
        let mut level = Level::new();
        level.insert_unique(Age(20));
        levels.insert(level);
    }

    {
        let level = ecs.fetch::<LevelRef>();
        println!("Level: index({})", level.index());
    }

    {
        let mut age = ecs.fetch::<UniMut<Age>>();
        println!("Direct Fetch: Age: {}", age.0);
        age.0 += 1;
        println!(" - Increment");
    }

    {
        let level = ecs.fetch::<LevelMut>();
        println!("Level: index({})", level.index());
        let age = level.unique::<Age>().unwrap();
        println!(" - Age: {}", age.0);
    }

    {
        let mut levels = ecs.res_mut::<Levels>().unwrap();
        levels.select(1);
    }

    {
        let age = ecs.fetch::<UniRef<Age>>();
        println!("Age: {}", age.0);
    }
}
