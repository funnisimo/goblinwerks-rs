use gw_app::{ecs::Entity, log, Ecs};
use gw_world::{
    action::BoxedAction, level::get_current_level_mut, map::Map, task::TaskResult, tile::Tiles,
};

#[derive(Default, Clone, Debug)]
pub struct Moons {
    phase: usize,
}

static MOON_LOCATIONS: [&str; 8] = [
    "MOON_0", "MOON_1", "MOON_2", "MOON_3", "MOON_4", "MOON_5", "MOON_6", "MOON_7",
];

impl Moons {
    pub fn new() -> Self {
        Moons { phase: 0 }
    }

    pub fn increment(&mut self) {
        self.phase = (self.phase + 1) % (8 * 3);
    }

    pub fn location(&self) -> &'static str {
        let index = self.phase / 3;
        MOON_LOCATIONS[index]
    }

    pub fn destination(&self) -> &'static str {
        let index = self.phase % 8;
        MOON_LOCATIONS[index]
    }
}

/// Try to move toward the hero - will be stopped by the counters.
pub fn move_moongate(ecs: &mut Ecs, _entity: Entity) -> TaskResult {
    let mut moons = ecs.resources.get_mut_or_insert_with(|| Moons::new());
    let location = moons.location();
    moons.increment();
    let new_location = moons.location();
    drop(moons);

    let moongate = {
        let tiles = ecs.resources.get::<Tiles>().unwrap();
        tiles.get("MOONGATE").unwrap()
    };

    let level = get_current_level_mut(ecs);
    let mut map = level.resources.get_mut::<Map>().unwrap();

    let current_idx = map.get_location(location).unwrap();
    map.clear_fixture(current_idx);

    let new_idx = map.get_location(new_location).unwrap();
    map.place_fixture(new_idx, moongate);

    log(format!("Moongate location = {}", new_location));

    TaskResult::Success(500)
}
