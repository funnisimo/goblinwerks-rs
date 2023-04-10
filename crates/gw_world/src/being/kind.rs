use std::sync::Arc;

use super::{Being, BeingKindBuilder, BeingKindFlags};
use crate::hero::Hero;
use crate::level::Level;
use crate::map::Map;
use crate::position::Position;
use crate::sprite::Sprite;
use crate::task::Task;
use gw_app::ecs::Entity;
use gw_util::point::Point;

#[derive(Debug, Clone)]
pub struct BeingKind {
    pub id: String,
    pub sprite: Sprite,
    pub being: Being,
    pub flags: BeingKindFlags,
    pub task: String,
}

impl BeingKind {
    pub fn builder(id: &str) -> BeingKindBuilder {
        BeingKindBuilder::new(id)
    }

    pub(super) fn new(builder: BeingKindBuilder) -> Self {
        BeingKind {
            id: builder.id,
            sprite: builder.sprite,
            being: builder.info,
            flags: builder.flags,
            task: builder.task,
        }
    }
}

pub fn spawn_being(kind: &Arc<BeingKind>, level: &mut Level, point: Point) -> Entity {
    let index = level
        .resources
        .get::<Map>()
        .unwrap()
        .get_index(point.x, point.y);
    if let Some(idx) = index {
        let pos = Position::from(point).with_blocking(true);

        println!("spawn being({}) - task={}", kind.id, kind.task);

        let entity = level.world.push((
            kind.being.clone(),
            pos,
            kind.sprite.clone(),
            Task::new(kind.task.clone()),
        ));

        if kind.flags.contains(BeingKindFlags::HERO) {
            level.resources.insert(Hero::new(entity));
        }

        // make map aware of actor
        let mut map = level.resources.get_mut::<Map>().unwrap();
        map.add_being(idx, entity, true);

        // Add to schedule
        level.executor.insert(entity, kind.being.act_time as u64);

        return entity;
    }

    panic!(
        "Trying to add actor to position that does not exist! kind={}, pos={},{}",
        kind.id, point.x, point.y
    );
}
