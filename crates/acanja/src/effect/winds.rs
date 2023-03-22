use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::level::Levels;

////////////////////////

#[derive(Debug, Clone)]
pub struct Winds;

impl Winds {
    pub fn new() -> Self {
        Winds
    }
}

impl Effect for Winds {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log("Winds");
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
