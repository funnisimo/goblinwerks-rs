use super::{BoxedEffect, Effect, EffectResult};
use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::dice::Dice;
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Damage(Dice);

impl Damage {
    pub fn new(dice: Dice) -> Self {
        Damage(dice)
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

pub(crate) fn parse_damage(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_int() {
        Ok(Box::new(Damage(Dice::simple(
            0,
            0,
            value.as_int().unwrap() as i32,
        ))))
    } else if value.is_string() {
        let dice: Dice = match value.to_string().parse() {
            Err(_) => return Err(format!("Failed to parse dice - {:?}", value)),
            Ok(d) => d,
        };
        Ok(Box::new(Damage(dice)))
    } else {
        Err(format!(
            "Damage tile events can only receive int values.  Received: {:?}",
            value
        ))
    }
}
