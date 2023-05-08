use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::{
    effect::{BoxedEffect, Effect, EffectResult},
    log::Logger,
};

////////////////////////

#[derive(Debug, Clone)]
pub struct Gremlins;

impl Gremlins {
    pub fn new() -> Self {
        Gremlins
    }
}

impl Effect for Gremlins {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_resource::<Logger>();
        logger.log("Gremlins");
        EffectResult::Success
    }
}

pub fn parse_gremlins(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Gremlins))
    } else {
        Err(format!(
            "Gremlins effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
