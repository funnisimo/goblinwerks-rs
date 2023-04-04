use gw_ecs::{Ecs, Level, LevelMut, LevelRef, Levels};

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
        let level = ecs.fetch::<LevelMut>();
        println!("Level: index({})", level.index());
    }
}
