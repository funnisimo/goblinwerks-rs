use crate::entity::Entity;
use crate::light::{Light, LIGHT_COEF};
use crate::player::Player;
use crate::{Entities, PLAYER_FOV_RADIUS};
use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};
use gw_app::img::Images;
use gw_app::{color::RGBA, draw, Buffer, Ecs, Image};
use std::sync::Arc;

const START_COLOR: RGBA = RGBA::rgba(255, 0, 0, 255);
const LIGHT_COLOR: RGBA = RGBA::rgba(255, 255, 0, 255);
const LIGHT_RADIUS: f32 = 15.0;
const PLAYER_LIGHT_RADIUS: f32 = 8.0;
const PLAYER_LIGHT_COLOR: RGBA = RGBA::rgba(150, 150, 150, 255);
const WALL_COLOR: RGBA = RGBA::rgba(255, 255, 255, 255);
const GOBLIN_COLOR: RGBA = RGBA::rgba(0, 255, 0, 255);
const VISITED_BLEND_COLOR: RGBA = RGBA::rgba(10, 10, 40, 255);
const VISITED_BLEND_COEF: f32 = 0.8;

pub const CHAR_SUBP_NW: i32 = 226;
pub const CHAR_SUBP_NE: i32 = 227;
pub const CHAR_SUBP_N: i32 = 228;
pub const CHAR_SUBP_SE: i32 = 229;
pub const CHAR_SUBP_DIAG: i32 = 230;
pub const CHAR_SUBP_E: i32 = 231;
pub const CHAR_SUBP_SW: i32 = 232;

fn my_to_glyph(flag: u8) -> i32 {
    match flag {
        0 => 0,
        1 => CHAR_SUBP_NE,
        2 => CHAR_SUBP_SW,
        3 => -CHAR_SUBP_DIAG,
        4 => CHAR_SUBP_SE,
        5 => CHAR_SUBP_E,
        6 => -CHAR_SUBP_N,
        7 => -CHAR_SUBP_NW,
        _ => 0,
    }
}

pub struct Level {
    /// a picture containing color coded walls, player start pos and entities. subcell resolution (2x2 pixel for each console cell)
    level_img: Arc<Image>,
    /// the level's ground texture. subcell resolution
    ground: Arc<Image>,
    /// whether the level_img has been loaded
    // loaded: bool,
    /// computed light in the level. subcell resolution
    lightmap: Image,
    /// the level size in console cells
    size: (i32, i32),
    /// the player start position in console cells
    start: (i32, i32),
    /// where the player can walk (one bool for each console cell)
    walls: Vec<bool>,
    /// what part of the level have been visited (subcell resolution)
    visited_2x: Vec<bool>,
    /// utility to compute field of view
    fov: FovRestrictive,
    /// what part of the level are in the player's field of view (subcell resolution)
    map: MapData,
    /// the final background image (subcell resolution)
    render_output: Image,
    /// dynamic lights in the level
    lights: Vec<Light>,
    /// some dim light following the player to keep him from being in total darkness
    player_light: Light,
    // path: String,
}

impl Level {
    pub fn new(level_img: Arc<Image>, ground: Arc<Image>) -> Self {
        Self {
            level_img,
            ground,
            // loaded: false,
            lightmap: Image::empty(1, 1),
            render_output: Image::empty(1, 1),
            size: (0, 0),
            start: (0, 0),
            walls: Vec::new(),
            visited_2x: Vec::new(),
            fov: FovRestrictive::new(),
            map: MapData::new(1, 1),
            lights: Vec::new(),
            player_light: Light::new((0, 0), PLAYER_LIGHT_RADIUS, PLAYER_LIGHT_COLOR),
        }
    }

    pub fn start_pos(&self) -> (i32, i32) {
        self.start
    }
    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        self.walls[self.offset(pos)]
    }
    pub fn light_at(&self, (x, y): (i32, i32)) -> RGBA {
        self.lightmap
            // .borrow()
            .pixel(x as u32 * 2, y as u32 * 2)
            .unwrap()
    }
    pub fn update(&mut self) {
        for light in self.lights.iter_mut() {
            light.update();
        }
    }
    pub fn render(&mut self, buffer: &mut Buffer, player_pos: (i32, i32)) {
        self.compute_lightmap(player_pos);

        for y in 0..self.size.1 as usize * 2 {
            for x in 0..self.size.0 as usize * 2 {
                let off = self.offset_2x((x as i32, y as i32));
                if self.map.is_in_fov(x, y)
                    && (self.map.is_transparent(x, y) || !self.visited_2x[off])
                {
                    self.visited_2x[off] = true;
                    let ground_col = self.ground.pixel(x as u32, y as u32).unwrap();
                    let light_col = self.lightmap.pixel(x as u32, y as u32).unwrap();
                    let mut r =
                        f32::from(ground_col.0) * f32::from(light_col.0) * LIGHT_COEF / 255.0;
                    let mut g =
                        f32::from(ground_col.1) * f32::from(light_col.1) * LIGHT_COEF / 255.0;
                    let mut b =
                        f32::from(ground_col.2) * f32::from(light_col.2) * LIGHT_COEF / 255.0;
                    r = r.min(255.0);
                    g = g.min(255.0);
                    b = b.min(255.0);
                    self.render_output.put_pixel(
                        x as u32,
                        y as u32,
                        (r as u8, g as u8, b as u8, 255).into(),
                    );
                } else if self.visited_2x[off] {
                    let col = self.ground.pixel(x as u32, y as u32).unwrap();
                    let dark_col = RGBA::blend(&col, &VISITED_BLEND_COLOR, VISITED_BLEND_COEF);
                    self.render_output
                        // .borrow_mut()
                        .put_pixel(x as u32, y as u32, dark_col);
                } else {
                    self.render_output
                        .put_pixel(x as u32, y as u32, (0, 0, 0, 255).into());
                }
            }
        }

        draw::subcell(buffer).to_glyph(&my_to_glyph).blit(
            &self.render_output,
            0,
            0,
            0,
            0,
            None,
            None,
        );
    }
    pub fn is_in_fov(&self, pos: (i32, i32)) -> bool {
        self.map.is_in_fov(pos.0 as usize * 2, pos.1 as usize * 2)
    }
    pub fn compute_fov(&mut self, (x, y): (i32, i32), radius: usize) {
        self.map.clear_fov();
        self.fov
            .compute_fov(&mut self.map, x as usize * 2, y as usize * 2, radius, true);
    }
    // fn add_light(&mut self, pos: (i32, i32)) {
    //     self.lights.push(Light::new(pos, LIGHT_RADIUS, LIGHT_COLOR));
    // }
    fn compute_lightmap(&mut self, (px, py): (i32, i32)) {
        // TODO check if filling with black pixels is faster
        self.lightmap = Image::empty(self.size.0 as u32 * 2, self.size.1 as u32 * 2);
        let mut fov = FovRestrictive::new();
        *self.player_light.pos_mut() = ((px * 2) as f32, (py * 2) as f32);
        self.player_light
            .render(&mut self.map, &mut fov, &mut self.lightmap, false);
        for light in self.lights.iter() {
            light.render(&mut self.map, &mut fov, &mut self.lightmap, true);
        }
    }
    fn compute_walls_2x_and_start_pos(&mut self) -> Vec<Entity> {
        let mut entities = Vec::new();
        let level_img = &self.level_img;
        let image_size = level_img.size();
        self.size = (image_size.0 as i32 / 2, image_size.1 as i32 / 2);
        self.walls = vec![false; (self.size.0 * self.size.1) as usize];
        self.map = MapData::new(image_size.0 as usize, image_size.1 as usize);
        for y in 0..image_size.1 {
            for x in 0..image_size.0 {
                let p = level_img.pixel(x, y).unwrap();
                self.map
                    .set_transparent(x as usize, y as usize, p != WALL_COLOR);
                self.visited_2x.push(false);
                let pos_1x = (x as i32 / 2, y as i32 / 2);
                match p {
                    START_COLOR => self.start = pos_1x,
                    LIGHT_COLOR => {
                        // self.add_light((x as i32, y as i32));
                        self.lights.push(Light::new(
                            (x as i32, y as i32),
                            LIGHT_RADIUS,
                            LIGHT_COLOR,
                        ));

                        entities.push(Entity::new_light(pos_1x));
                    }
                    GOBLIN_COLOR => {
                        let off = self.offset(pos_1x);
                        self.walls[off] = true;
                        entities.push(Entity::new_goblin(pos_1x));
                    }
                    _ => (),
                }
            }
        }
        entities
    }
    fn compute_walls(&mut self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let mut count = 0;
                let x2 = x as usize * 2;
                let y2 = y as usize * 2;
                if self.map.is_transparent(x2, y2) {
                    count += 1;
                }
                if self.map.is_transparent(x2 + 1, y2) {
                    count += 1;
                }
                if self.map.is_transparent(x2, y2 + 1) {
                    count += 1;
                }
                if self.map.is_transparent(x2 + 1, y2 + 1) {
                    count += 1;
                }
                if count < 2 {
                    let off = self.offset((x, y));
                    self.walls[off] = true;
                }
            }
        }
    }
    fn offset(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.size.0 as i32).max(0) as usize
    }
    fn offset_2x(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.size.0 as i32 * 2).max(0) as usize
    }
}

pub fn load_level(ecs: &mut Ecs, img_path: &str) -> bool {
    let level_path = img_path.to_owned() + ".png";
    let ground_path = img_path.to_owned() + "_color.png";

    let (level_img, ground) = {
        let images = ecs.resources.get::<Images>().unwrap();
        (images.get(&level_path), images.get(&ground_path))
    };

    if level_img.is_none() || ground.is_none() {
        return false;
    }

    let mut level = Level::new(level_img.unwrap(), ground.unwrap());

    let entities = level.compute_walls_2x_and_start_pos();
    level.compute_walls();
    level.lightmap = Image::empty(level.size.0 as u32 * 2, level.size.1 as u32 * 2);
    level.render_output = Image::empty(level.size.0 as u32 * 2, level.size.1 as u32 * 2);

    let mut player = Player::new(super::PLAYER_SPEED);
    player.move_to(level.start_pos());
    level.compute_fov(player.pos(), PLAYER_FOV_RADIUS);

    ecs.resources.insert(level);
    ecs.resources.insert(Entities(entities));
    ecs.resources.insert(player);

    true
}
