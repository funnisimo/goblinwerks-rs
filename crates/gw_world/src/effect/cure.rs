use crate::log::Logger;

use super::{BoxedEffect, Effect, EffectResult};
use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Cure;

impl Effect for Cure {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        // TODO - Logger is a global now!!!!
        let mut logger = world.write_global::<Logger>();
        logger.log(format!("Cured!"));
        EffectResult::Success
    }
}

pub(super) fn parse_cure(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Cure))
    } else {
        Err(format!(
            "Cure effect can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
