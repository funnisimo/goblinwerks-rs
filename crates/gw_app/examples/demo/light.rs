use crate::noise::simplex;
use doryen_fov::{FovAlgorithm, MapData};
use gw_app::{color::RGBA, Image};
// use std::cell::RefCell;
// use std::rc::Rc;

pub const LIGHT_COEF: f32 = 1.5;

const TIME_SCALE: f32 = 0.05;
const LIGHT_INTENSITY: f32 = 1.5;
const LIGHT_FLICKER_MOVE: f32 = 2.0;
const LIGHT_FLICKER_INTENSITY: f32 = 0.4;
const LIGHT_FLICKER_RADIUS: f32 = 0.2;

pub struct Light {
    pos: (f32, f32),
    radius: f32,
    intensity: f32,
    color: RGBA,
    t: f32,
}

impl Light {
    pub fn new((x, y): (i32, i32), radius: f32, color: RGBA) -> Self {
        Self {
            pos: (x as f32, y as f32),
            radius,
            color,
            intensity: LIGHT_INTENSITY,
            // "random" t initial value so that all lights don't flicker in sync
            t: (x * y) as f32,
        }
    }
    pub fn is_penumbra(color: RGBA, level: usize) -> bool {
        (color.0 as usize + color.1 as usize + color.2 as usize) < level
    }
    pub fn pos_mut(&mut self) -> &mut (f32, f32) {
        &mut self.pos
    }
    pub fn update(&mut self) {
        self.t += TIME_SCALE;
    }
    pub fn render(
        &self,
        level_map: &mut MapData,
        fov: &mut dyn FovAlgorithm,
        lightmap: &mut Image,
        flicker: bool,
    ) {
        let (px, py, intensity, radius) = if flicker {
            // alter light position, radius and intensity over time
            (
                self.pos.0 + (LIGHT_FLICKER_MOVE * (simplex(self.t) - 0.5)),
                self.pos.1 + (LIGHT_FLICKER_MOVE * (simplex(self.t + 2.0) - 0.5)),
                self.intensity + LIGHT_FLICKER_INTENSITY * (simplex(self.t + 4.0) - 0.5),
                self.radius * (1.0 + LIGHT_FLICKER_RADIUS * (simplex(self.t + 6.0) - 0.5)),
            )
        } else {
            (self.pos.0, self.pos.1, self.intensity, self.radius)
        };
        let minx = ((px - radius).floor() as i32).max(0) as u32;
        let maxx = ((px + radius).ceil() as i32).min(lightmap.width() as i32 - 1) as u32;
        let miny = ((py - radius).floor() as i32).max(0) as u32;
        let maxy = ((py + radius).ceil() as i32).min(lightmap.height() as i32 - 1) as u32;
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut map = MapData::new(width as usize, height as usize);
        for y in miny..=maxy {
            for x in minx..=maxx {
                map.set_transparent(
                    (x - minx) as usize,
                    (y - miny) as usize,
                    level_map.is_transparent(x as usize, y as usize),
                );
            }
        }
        fov.compute_fov(
            &mut map,
            px as usize - minx as usize,
            py as usize - miny as usize,
            radius as usize,
            true,
        );
        let light_color = RGBA::scale(&self.color, intensity);
        let radius2 = radius * radius;
        let radius_coef = 1.0 / (1.0 + radius2 / 20.0);
        for y in miny..=maxy {
            for x in minx..=maxx {
                if map.is_in_fov((x - minx) as usize, (y - miny) as usize) {
                    let dx = x as f32 - px;
                    let dy = y as f32 - py;
                    // good looking lights.
                    let squared_dist = dx * dx + dy * dy;
                    let intensity_coef = 1.0 / (1.0 + squared_dist / 20.0);
                    let intensity_coef = intensity_coef - radius_coef;
                    let intensity_coef = intensity_coef / (1.0 - radius_coef);
                    if intensity_coef > 0.0 {
                        let light = RGBA::darken(&light_color, 1.0 - intensity_coef);
                        let cur_light = lightmap.pixel(x, y).unwrap();
                        lightmap
                            // .borrow_mut()
                            .put_pixel(x, y, RGBA::mix(&light, &cur_light));
                    }
                }
            }
        }
    }
}
