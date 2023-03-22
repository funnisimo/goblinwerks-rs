use super::{BoxedEffect, Effect, EffectResult};
use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Cure;

impl Effect for Cure {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(format!("Cured!"));
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
