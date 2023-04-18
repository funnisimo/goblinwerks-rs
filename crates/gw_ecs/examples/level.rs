use gw_ecs::Ecs;
use gw_ecs::Fetch;
use gw_ecs::{LevelRef, LevelsMut, LevelsRef, Unique, UniqueMut};
// struct Info(u32);

struct Age(u32);

fn main() {
    let ecs = Ecs::new();

    {
        // Build 2 levels
        let mut levels = ecs.fetch_mut::<LevelsMut>();

        let mut level = levels.create();
        println!("Create level = {}", level.index());
        level.insert_unique(Age(10));
        let index = level.index();
        drop(level);
        levels.select(index);

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
        let mut levels = <LevelsMut>::fetch(&ecs);
        println!("Change current level - 1");
        levels.select(1);
    }

    {
        // age is 20
        let (level, age) = <(LevelRef, Unique<Age>)>::fetch(&ecs);
        println!("Current Level - index({})", level.index());
        println!(" - Age: {}", age.0);
    }
}
