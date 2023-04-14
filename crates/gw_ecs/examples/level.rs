use gw_ecs::{Ecs, LevelRef, Levels, LevelsMut, LevelsRef, UniqueMut, UniqueRef};

// struct Info(u32);

struct Age(u32);

fn main() {
    let mut ecs = Ecs::new();

    {
        // Build 2 levels
        let mut levels = ecs.fetch_mut::<LevelsMut>();

        let mut level = levels.create();
        println!("Create level = {}", level.index());
        level.insert_unique(Age(10));
        drop(level);

        let mut level = levels.create();
        println!("Create level = {}", level.index());
        level.insert_unique(Age(20));
    }

    {
        // Show current is first one added
        let level = ecs.fetch::<LevelRef>();
        println!("Current Level: index({})", level.index());
        println!(" - Age: {}", level.get_unique::<Age>().unwrap().0);
    }

    {
        // Increment age
        let mut age = ecs.fetch_mut::<UniqueMut<Age>>();
        println!("Direct Unique Fetch: Age: {}", age.0);
        age.0 += 1;
        println!(" - Increment");
    }

    {
        // can get level via fetch
        let (levels, level) = ecs.fetch::<(LevelsRef, LevelRef)>();
        println!("Levels - current index = {}", levels.current_index());
        println!("Level: index({})", level.index());
        let age = level.get_unique::<Age>().unwrap();
        println!(" - Age: {}", age.0);
    }

    {
        // Change to next level
        let mut levels = ecs.get_global_mut::<Levels>().unwrap();
        println!("Change current level - 1");
        levels.select(1);
    }

    {
        // age is 20
        let (level, age) = ecs.fetch::<(LevelRef, UniqueRef<Age>)>();
        println!("Current Level - index({})", level.index());
        println!(" - Age: {}", age.0);
    }
}
