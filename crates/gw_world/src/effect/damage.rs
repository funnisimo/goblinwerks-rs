use super::{BoxedEffect, Effect, EffectResult};
use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Damage(i32);

impl Damage {
    pub fn new(amount: i32) -> Self {
        Damage(amount)
    }
}

impl Effect for Damage {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(format!("Damage - {}", self.0));
        EffectResult::Success
    }
}

pub(super) fn parse_damage(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_int() {
        Ok(Box::new(Damage(value.as_int().unwrap() as i32)))
    } else {
        Err(format!(
            "Damage tile events can only receive int values.  Received: {:?}",
            value
        ))
    }
}
