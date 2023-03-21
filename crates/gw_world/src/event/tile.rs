use super::{EventResult, TileEvent};
use crate::{level::Levels, map::Map, tile::Tiles};
use gw_app::ecs::{Read, ResourceSet, Write};

pub struct ForceTile(String);

impl TileEvent for ForceTile {
    fn fire(
        &self,
        ecs: &mut gw_app::Ecs,
        _entity: gw_app::ecs::Entity,
        pos: gw_util::point::Point,
    ) -> EventResult {
        let (mut levels, tiles) = <(Write<Levels>, Read<Tiles>)>::fetch_mut(&mut ecs.resources);
        let level = levels.current_mut();

        let tile = match tiles.get(&self.0) {
            None => return EventResult::Fail,
            Some(tile) => tile,
        };

        let mut map = level.resources.get_mut::<Map>().unwrap();

        let idx = map.get_wrapped_index(pos.x, pos.y).unwrap();

        // TODO - Flags for clear fixture, required ground, etc...
        map.force_tile(idx, tile);

        EventResult::Success
    }
}
