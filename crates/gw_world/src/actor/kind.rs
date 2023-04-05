use std::sync::Arc;

use super::{Actor, ActorKindBuilder, ActorKindFlags};
use crate::hero::Hero;
use crate::level::Level;
use crate::map::Map;
use crate::position::Position;
use crate::sprite::Sprite;
use gw_app::ecs::Entity;
use gw_util::point::Point;

#[derive(Debug, Clone)]
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
    let index = level
        .resources
        .get::<Map>()
        .unwrap()
        .get_index(point.x, point.y);
    if let Some(idx) = index {
        let pos = Position::from(point).with_blocking(true);
        let entity = level.world.push((
            kind.info.clone(),
            pos,
            kind.sprite.clone(), /* kind.clone() */
        ));

        if kind.flags.contains(ActorKindFlags::HERO) {
            level.resources.insert(Hero::new(entity));
        }

        // make map aware of actor
        let mut map = level.resources.get_mut::<Map>().unwrap();
        map.add_actor(idx, entity, true);

        // Add to schedule
        level.executor.insert_actor(entity, kind.info.act_time);

        return entity;
    }

    panic!(
        "Trying to add actor to position that does not exist! kind={}, pos={},{}",
        kind.id, point.x, point.y
    );
}
