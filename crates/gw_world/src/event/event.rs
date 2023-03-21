use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;

pub enum EventResult {
    Success,
    Fail,
    Stop,
}

pub trait TileEvent {
    fn fire(&self, ecs: &mut Ecs, entity: Entity, pos: Point) -> EventResult;
}

////////////////////////

pub struct Message(String);

impl TileEvent for Message {
    fn fire(&self, ecs: &mut Ecs, _entity: Entity, _pos: Point) -> EventResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(self.0.clone());
        EventResult::Success
    }
}
