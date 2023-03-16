use gw_app::{ecs::Entity, log};
use gw_util::point::Point;

use crate::{level::Level, position::Position};

pub struct Camera {
    center: Point,
    follows: Option<Entity>,
    size: (u32, u32),
    needs_draw: bool,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Camera {
            center: Point::new(width as i32 / 2, height as i32 / 2),
            follows: None,
            size: (width, height),
            needs_draw: true,
        }
    }

    pub fn with_center(mut self, x: i32, y: i32) -> Self {
        self.center.x = x;
        self.center.y = y;
        self
    }

    pub fn offset(&self) -> (i32, i32) {
        (
            self.center.x - self.size.0 as i32 / 2,
            self.center.y - self.size.1 as i32 / 2,
        )
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = (width, height);
        self.set_needs_draw();
    }

    pub fn set_center(&mut self, x: i32, y: i32) {
        self.center.set(x, y);
        self.set_needs_draw();
    }

    pub fn move_center(&mut self, dx: i32, dy: i32) {
        self.center.x = self.center.x + dx;
        self.center.y = self.center.y + dy;
        self.set_needs_draw();
    }

    pub fn set_center_point(&mut self, pt: &Point) {
        self.set_center(pt.x, pt.y);
    }

    pub fn needs_draw(&self) -> bool {
        self.needs_draw
    }

    pub fn clear_needs_draw(&mut self) {
        self.needs_draw = false;
    }

    pub fn set_needs_draw(&mut self) {
        self.needs_draw = true;
    }

    pub fn set_follows(&mut self, entity: Entity) {
        self.follows = Some(entity);
        self.needs_draw = true;
    }

    pub fn clear_follows(&mut self) {
        self.follows = None;
        self.needs_draw = true;
    }
}

pub fn update_camera_follows(level: &mut Level) {
    if let Some(mut camera) = level.resources.get_mut::<Camera>() {
        if let Some(ref entity) = camera.follows {
            if let Some(entry) = level.world.entry(*entity) {
                if let Ok(pos) = entry.get_component::<Position>() {
                    camera.set_center(pos.x, pos.y);
                }
            } else {
                camera.follows = None;
                log("Cancelling camera follows - entity not found.");
            }
        }
    }
}
