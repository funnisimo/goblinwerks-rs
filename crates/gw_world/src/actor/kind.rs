use std::sync::Arc;

use super::{Actor, ActorKindBuilder, ActorKindFlags};
use crate::hero::Hero;
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
    pub flags: ActorKindFlags,
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
            flags: builder.flags,
        }
    }
}

pub fn spawn_actor(kind: &Arc<ActorKind>, level: &mut Level, point: Point) -> Entity {
    let pos: Position = point.into();
    let entity = level.world.push((
        kind.info.clone(),
        pos,
        kind.sprite.clone(), /* kind.clone() */
    ));

    if kind.flags.contains(ActorKindFlags::HERO) {
        level.resources.insert(Hero::new(entity));
    }

    entity
}
