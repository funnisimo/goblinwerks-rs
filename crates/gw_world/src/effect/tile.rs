use super::{BoxedEffect, Effect, EffectResult};
use crate::{level::Levels, map::Map, tile::Tiles};
use gw_app::{
    ecs::{Entity, Read, ResourceSet, Write},
    Ecs,
};
use gw_util::{point::Point, value::Value};

#[derive(Debug, Clone)]
pub struct ForceTile(String);

impl ForceTile {
    pub fn new(id: String) -> Self {
        ForceTile(id)
    }
}

impl Effect for ForceTile {
    fn fire(&self, ecs: &mut Ecs, pos: Point, _entity: Option<Entity>) -> EffectResult {
        let (mut levels, tiles) = <(Write<Levels>, Read<Tiles>)>::fetch_mut(&mut ecs.resources);
        let level = levels.current_mut();

        let tile = match tiles.get(&self.0) {
            None => return EffectResult::Fail,
            Some(tile) => tile,
        };

        let mut map = level.resources.get_mut::<Map>().unwrap();

        let idx = map.get_wrapped_index(pos.x, pos.y).unwrap();

        // TODO - Flags for clear fixture, required ground, etc...
        map.force_tile(idx, tile);

        EffectResult::Success
    }
}

pub(super) fn parse_tile(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(ForceTile::new(value.to_string())))
    } else {
        Err(format!(
            "Tile effects can only receive string values.  Received: {:?}",
            value
        ))
    }
}

///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct ResetTiles(String);

impl Effect for ResetTiles {
    fn fire(&self, ecs: &mut Ecs, pos: Point, _entity: Option<Entity>) -> EffectResult {
        let (mut levels, tiles) = <(Write<Levels>, Read<Tiles>)>::fetch_mut(&mut ecs.resources);
        let level = levels.current_mut();

        let tile = match tiles.get(&self.0) {
            None => return EffectResult::Fail,
            Some(tile) => tile,
        };

        let mut map = level.resources.get_mut::<Map>().unwrap();

        let idx = map.get_wrapped_index(pos.x, pos.y).unwrap();

        // TODO - Flags for clear fixture, required ground, etc...
        map.reset_tiles(idx, tile);

        EffectResult::Success
    }
}

#[derive(Debug, Clone)]
pub struct ForceFixture(String);

impl ForceFixture {
    pub fn new(id: String) -> Self {
        ForceFixture(id)
    }
}

impl Effect for ForceFixture {
    fn fire(&self, ecs: &mut Ecs, pos: Point, _entity: Option<Entity>) -> EffectResult {
        let (mut levels, tiles) = <(Write<Levels>, Read<Tiles>)>::fetch_mut(&mut ecs.resources);
        let level = levels.current_mut();

        let tile = match tiles.get(&self.0) {
            None => return EffectResult::Fail,
            Some(tile) => tile,
        };

        let mut map = level.resources.get_mut::<Map>().unwrap();

        let idx = map.get_wrapped_index(pos.x, pos.y).unwrap();

        // TODO - Flags for clear fixture, required ground, etc...
        map.force_fixture(idx, tile);

        EffectResult::Success
    }
}

pub(super) fn parse_fixture(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(ForceFixture::new(value.to_string())))
    } else {
        Err(format!(
            "Fixture effects can only receive string values.  Received: {:?}",
            value
        ))
    }
}

///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct ClearFixture;

impl Effect for ClearFixture {
    fn fire(
        &self,
        ecs: &mut gw_app::Ecs,
        pos: gw_util::point::Point,
        _entity: Option<Entity>,
    ) -> EffectResult {
        let (mut levels,) = <(Write<Levels>,)>::fetch_mut(&mut ecs.resources);
        let level = levels.current_mut();

        let mut map = level.resources.get_mut::<Map>().unwrap();

        let idx = map.get_wrapped_index(pos.x, pos.y).unwrap();

        // TODO - Anything else?
        map.clear_fixture(idx);

        EffectResult::Success
    }
}
