use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::log::Logger;

////////////////////////

#[derive(Debug, Clone)]
pub struct Winds;

impl Winds {
    pub fn new() -> Self {
        Winds
    }
}

impl Effect for Winds {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_resource::<Logger>();
        logger.log("Winds");
        EffectResult::Success
    }
}

pub fn parse_winds(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Winds))
    } else {
        Err(format!(
            "Message effects can only receive string values.  Received: {:?}",
            value
        ))
    }
}
