use super::{BoxedEffect, Effect, EffectResult};
use crate::log::Logger;
use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Treasure;

impl Effect for Treasure {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_resource::<Logger>();

        logger.log(format!("Treasured!"));
        EffectResult::Success
    }
}

pub(super) fn parse_treasure(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Treasure))
    } else {
        Err(format!(
            "Treasure effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
