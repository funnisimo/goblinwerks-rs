use std::sync::Arc;

use super::{Being, BeingKindBuilder, BeingKindFlags, Stats};
use crate::hero::Hero;
use crate::map::Map;
use crate::position::Position;
use crate::sprite::Sprite;
use crate::task::Task;
use crate::{combat::Melee, task::Executor};
use gw_app::{ecs::Entity, log};
use gw_ecs::{specs::LazyUpdate, Builder, Entities, ReadRes, SystemData, World, WriteRes};
use gw_util::point::Point;

#[derive(Debug, Clone)]
pub struct BeingKind {
    pub id: String,
    pub sprite: Sprite,
    pub being: Being,
    pub task: String,
    pub melee: Option<Melee>,
    pub stats: Stats,
}

impl BeingKind {
    pub fn builder(id: &str) -> BeingKindBuilder {
        BeingKindBuilder::new(id)
    }

    pub(super) fn new(builder: BeingKindBuilder) -> Self {
        BeingKind {
            id: builder.id,
            sprite: builder.sprite,
            being: builder.being,
            task: builder.task,
            melee: builder.melee,
            stats: builder.stats,
        }
    }
}

pub fn spawn_being(kind: &Arc<BeingKind>, world: &World, point: Point) -> Entity {
    let (mut map, lazy_update, entities, mut executor) = <(
        WriteRes<Map>,
        ReadRes<LazyUpdate>,
        Entities,
        WriteRes<Executor>,
    )>::fetch(world);

    let index = map.get_index(point.x, point.y);

    if let Some(idx) = index {
        let pos = Position::from(point).with_blocking(true);

        println!("spawn being({}) - task={}", kind.id, kind.task);

        let mut builder = lazy_update
            .create_entity(&entities)
            .with(kind.being.clone())
            .with(pos)
            .with(kind.sprite.clone())
            .with(Task::new(kind.task.clone()))
            .with(kind.stats.clone());

        if let Some(ref melee) = kind.melee {
            builder = builder.with(melee.clone());
            log(format!("SPAWN MELEE!!!"));
        }
        let entity = builder.build();

        if kind.being.kind_flags.contains(BeingKindFlags::HERO) {
            let mut hero = world.write_resource::<Hero>();
            hero.entity = entity;
        }

        // make map aware of actor
        map.add_being(idx, entity, true);

        // Add to schedule
        executor.insert(entity, kind.being.act_time as u64);

        return entity;
    }

    panic!(
        "Trying to add actor to position that does not exist! kind={}, pos={},{}",
        kind.id, point.x, point.y
    );
}
