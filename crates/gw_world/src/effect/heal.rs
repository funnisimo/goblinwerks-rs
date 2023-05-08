use crate::log::Logger;

use super::{BoxedEffect, Effect, EffectResult};
use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Heal;

impl Effect for Heal {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();
        logger.log(format!("Healed!"));
        EffectResult::Success
    }
}

pub(super) fn parse_heal(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Heal))
    } else {
        Err(format!(
            "Heal effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
