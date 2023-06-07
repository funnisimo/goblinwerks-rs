use crate::log::Logger;

use super::{BoxedEffect, Effect, EffectResult};
use gw_ecs::prelude::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Poison;

impl Effect for Poison {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();

        logger.log(format!("Poisoned!"));
        EffectResult::Success
    }
}

pub(super) fn parse_poison(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Poison))
    } else {
        Err(format!(
            "Poison tile events can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
