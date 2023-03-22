use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::level::Levels;

////////////////////////

#[derive(Debug, Clone)]
pub struct Mark(String);

impl Mark {
    pub fn new(name: String) -> Self {
        Mark(name)
    }
}

impl Effect for Mark {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(format!("Mark = {}", self.0));
        EffectResult::Success
    }
}

pub fn parse_mark(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(Mark::new(value.to_string())))
    } else {
        Err(format!(
            "Mark effects can only receive string values.  Received: {:?}",
            value
        ))
    }
}
