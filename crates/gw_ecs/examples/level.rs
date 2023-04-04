use gw_ecs::{Ecs, Level, LevelMut, LevelRef, Levels, UniRef};

// struct Info(u32);

struct Age(u32);

fn main() {
    let ecs = Ecs::new();

    {
        let mut levels = ecs.res_mut::<Levels>().unwrap();
        let level = Level::new();
        levels.insert(level);
        let level = Level::new();
        levels.insert(level);
    }

    {
        let level = ecs.fetch::<LevelRef>();
        println!("Level: index({})", level.index());
    }

    {
        let mut levels = ecs.res_mut::<Levels>().unwrap();
        levels.select(1);
    }

    {
        let mut level = ecs.fetch::<LevelMut>();
        println!("Level: index({})", level.index());
        level.insert_unique(Age(12));
    }

    {
        let age = ecs.fetch::<UniRef<Age>>();
        println!("Age: {}", age.0);
    }
}
