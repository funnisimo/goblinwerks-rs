use super::{Actor, ActorKindBuilder};
use crate::level::Level;
use crate::position::Position;
use crate::sprite::Sprite;
use gw_app::ecs::Entity;
use gw_util::point::Point;

#[derive(Debug)]
pub struct ActorKind {
    pub id: String,
    pub sprite: Sprite,
    pub info: Actor,
}

impl ActorKind {
    pub fn builder(id: &str) -> ActorKindBuilder {
        ActorKindBuilder::new(id)
    }

    pub(super) fn new(builder: ActorKindBuilder) -> Self {
        ActorKind {
            id: builder.id,
            sprite: builder.sprite,
            info: builder.info,
        }
    }

    pub fn spawn(&self, level: &mut Level, point: Point) -> Entity {
        let pos: Position = point.into();
        level
            .world
            .push((Actor::new(), pos, self.sprite.clone(), self.info.clone()))
    }
}
