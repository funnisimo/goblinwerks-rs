use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::level::Levels;

////////////////////////

#[derive(Debug, Clone)]
pub struct MoongateTravel;

impl MoongateTravel {
    pub fn new() -> Self {
        MoongateTravel
    }
}

impl Effect for MoongateTravel {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log("MoongateTravel");
        EffectResult::Success
    }
}

pub fn parse_moongate_travel(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(MoongateTravel))
    } else {
        Err(format!(
            "MoongateTravel effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
