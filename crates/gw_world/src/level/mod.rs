mod level;
use gw_app::{
    ecs::{AtomicRef, AtomicRefMut},
    Ecs,
};
pub use level::*;
mod levels;
pub use levels::*;

pub fn with_current_level<F, T>(ecs: &Ecs, func: F) -> T
where
    F: FnOnce(&Level) -> T,
{
    match ecs.resources.get::<Levels>() {
        Some(levels) => func(levels.current()),
        None => match ecs.resources.get::<Level>() {
            Some(level) => func(&*level),
            None => panic!("No Level or Levels in Ecs"),
        },
    }
}

pub fn with_current_level_mut<F, T>(ecs: &mut Ecs, func: F) -> T
where
    F: FnOnce(&mut Level) -> T,
{
    match ecs.resources.get_mut::<Levels>() {
        Some(mut levels) => func(levels.current_mut()),
        None => match ecs.resources.get_mut::<Level>() {
            Some(mut level) => func(&mut *level),
            None => panic!("No Level or Levels in Ecs"),
        },
    }
}

pub fn get_current_level(ecs: &Ecs) -> AtomicRef<Level> {
    match ecs.resources.get::<Levels>() {
        Some(levels) => AtomicRef::map(levels, |levels| levels.current()),
        None => match ecs.resources.get::<Level>() {
            Some(level) => level,
            None => panic!("No Levels or Level in Ecs!"),
        },
    }
}

pub fn get_current_level_mut(ecs: &Ecs) -> AtomicRefMut<Level> {
    match ecs.resources.get_mut::<Levels>() {
        Some(levels) => AtomicRefMut::map(levels, |levels| levels.current_mut()),
        None => match ecs.resources.get_mut::<Level>() {
            Some(level) => level,
            None => panic!("No Levels or Level in Ecs!"),
        },
    }
}
