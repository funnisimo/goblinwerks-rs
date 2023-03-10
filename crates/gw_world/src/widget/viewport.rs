use crate::level::{Level, Levels};
use crate::map::Cell;
use crate::map::{CellFlags, Map};
use crate::memory::MapMemory;
use crate::position::Position;
use crate::sprite::Sprite;
use gw_app::color::named::BLACK;
use gw_app::color::{named, RGBA};
use gw_app::ecs::query::IntoQuery;
use gw_app::ecs::Entity;
use gw_app::ecs::{systems::ResourceSet, Read, Write};
use gw_app::messages::Messages;
use gw_app::{log, AppEvent, ScreenResult};
use gw_app::{Ecs, Panel};
use gw_util::point::Point;
use gw_util::rect::Rect;
use gw_util::value::Value;

enum VisType {
    NONE,
    MAPPED,
    REVEALED,
    VISIBLE,
}

#[allow(unused_variables)]
trait VisSource {
    fn get_vis_type(&self, idx: usize) -> VisType {
        VisType::VISIBLE
    }
}

// struct FovVisibility<'a> {
//     fov: Ref<'a, FOV>,
// }

// impl<'a> FovVisibility<'a> {
//     pub fn new(fov: &'a RefCell<FOV>) -> Self {
//         FovVisibility { fov: fov.borrow() }
//     }
// }

// impl<'a> VisSource for FovVisibility<'a> {
//     fn get_vis_type(&self, idx: usize) -> VisType {
//         if self.fov.is_visible_idx(idx) {
//             return VisType::VISIBLE;
//         }
//         if self.fov.is_revealed_idx(idx) {
//             return VisType::REVEALED;
//         }
//         if self.fov.is_mapped_idx(idx) {
//             return VisType::MAPPED;
//         }
//         VisType::NONE
//     }
// }

struct AlwaysVisible {}
impl AlwaysVisible {
    fn new() -> Self {
        AlwaysVisible {}
    }
}
impl VisSource for AlwaysVisible {}

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

    pub fn offset_for(&self, size: (u32, u32)) -> (i32, i32) {
        (
            self.center.x - size.0 as i32 / 2,
            self.center.y - size.1 as i32 / 2,
        )
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
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

pub struct Viewport {
    pub con: Panel,
    id: String,
    last_mouse: Point,
    needs_draw: bool,
}

impl Viewport {
    pub fn builder(id: &str) -> ViewPortBuilder {
        ViewPortBuilder::new(id)
    }

    fn new(builder: ViewPortBuilder) -> Self {
        let con = Panel::new(builder.size.0, builder.size.1, &builder.font);
        Viewport {
            con,
            id: builder.id,
            last_mouse: Point::new(-1, -1),
            needs_draw: true,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.con.size()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.con.resize(width, height);
        self.needs_draw = true;
    }

    fn get_map_cell(&self, ecs: &Ecs, screen_pct: (f32, f32)) -> Option<Point> {
        let view_point = self.con.mouse_point(screen_pct).unwrap();

        let (map_size, offset) = match ecs.resources.get::<Levels>() {
            Some(levels) => {
                let level = levels.current();
                let map_size = level.resources.get::<Map>().unwrap().get_size();
                let camera = level.resources.get::<Camera>().unwrap();
                let offset: Point = camera.offset_for(camera.size()).into();
                (map_size, offset)
            }
            None => match ecs.resources.get::<Level>() {
                Some(level) => {
                    let map_size = level.resources.get::<Map>().unwrap().get_size();
                    let camera = level.resources.get::<Camera>().unwrap();
                    let offset: Point = camera.offset_for(camera.size()).into();
                    (map_size, offset)
                }
                None => {
                    let map_size = match ecs.resources.get::<Map>() {
                        Some(map) => map.get_size(),
                        None => return None,
                    };
                    let camera = ecs.resources.get::<Camera>().unwrap();
                    let offset: Point = camera.offset_for(camera.size()).into();
                    (map_size, offset)
                }
            },
        };

        let map_point: Point = view_point + offset;

        if map_point.x < 0
            || map_point.y < 0
            || map_point.x >= map_size.0 as i32
            || map_point.y >= map_size.1 as i32
        {
            return None;
        }
        Some(map_point)
    }

    pub fn input(&mut self, ecs: &mut Ecs, event: &AppEvent) -> Option<ScreenResult> {
        match event {
            AppEvent::MousePos(screen_pct) => match self.get_map_cell(ecs, *screen_pct) {
                None => {}
                Some(cell) => {
                    if cell != self.last_mouse {
                        let mut msgs = ecs.resources.get_mut::<Messages>().unwrap();
                        msgs.push(
                            &format!("{}_MOVE", self.id),
                            Some(Value::Point(cell.x, cell.y)),
                        );
                        self.last_mouse = cell;
                    }
                }
            },
            AppEvent::MouseDown(mouse) => match self.get_map_cell(ecs, mouse.pos) {
                None => {}
                Some(cell) => {
                    let mut msgs = ecs.resources.get_mut::<Messages>().unwrap();
                    msgs.push(
                        &format!("{}_CLICK", self.id),
                        Some(Value::Point(cell.x, cell.y)),
                    );
                }
            },
            _ => {}
        }
        None
    }

    pub fn update(&mut self, _ecs: &mut Ecs) -> Option<ScreenResult> {
        None
    }

    pub fn draw_level(&mut self, level: &mut Level) {
        if !level.resources.contains::<Camera>() {
            let map_size = level.resources.get::<Map>().unwrap().get_size();
            let camera = Camera::new(map_size.0, map_size.1);
            level.resources.insert(camera);
        }

        let offset = {
            let camera = level.resources.get::<Camera>().unwrap();
            if self.con.size() != camera.size {
                self.resize(camera.size.0, camera.size.1);
            }
            camera.offset_for(self.con.size())
        };

        let viewport_needs_draw = {
            let camera = level.resources.get::<Camera>().unwrap();
            level.needs_draw() || camera.needs_draw() || self.needs_draw
        };

        if level.resources.contains::<MapMemory>() {
            let (mut map, mut memory) =
                <(Write<Map>, Write<MapMemory>)>::fetch_mut(&mut level.resources);

            draw_map(
                self,
                &mut map,
                Some(&mut memory),
                offset,
                viewport_needs_draw,
            );
        } else {
            let mut map = level.resources.get_mut::<Map>().unwrap();
            draw_map(self, &mut map, None, offset, viewport_needs_draw);
        }

        draw_actors(self, level);
        clear_needs_draw(self, level);
    }

    pub fn draw_map(
        &mut self,
        map: &mut Map,
        memory: Option<&mut MapMemory>,
        offset: (i32, i32),
        force_draw: bool,
    ) {
        let needs_draw = force_draw || self.needs_draw;
        draw_map(self, map, memory, offset, needs_draw);
    }

    pub fn render(&mut self, ecs: &mut Ecs) {
        self.con.render(ecs);
    }
}

////////////////////////////////////////

pub struct ViewPortBuilder {
    size: (u32, u32),
    extents: (f32, f32, f32, f32),
    id: String,
    font: String,
}

impl ViewPortBuilder {
    fn new(id: &str) -> Self {
        ViewPortBuilder {
            size: (0, 0),
            extents: (0.0, 0.0, 1.0, 1.0),
            id: id.to_string(),
            font: "DEFAULT".to_string(),
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn extents(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.extents = (left, top, right, bottom);
        self
    }

    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_string();
        self
    }

    pub fn build(self) -> Viewport {
        Viewport::new(self)
    }
}

fn draw_map(
    viewport: &mut Viewport,
    map: &mut Map,
    mut memory: Option<&mut MapMemory>,
    offset: (i32, i32),
    force_draw: bool,
) {
    let vis = AlwaysVisible::new();
    // let fov = global_world().get_fov(world.hero_entity()).unwrap().borrow();

    let size = viewport.con.size();
    // TODO - let offset = viewport.offset;

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let left = offset.0; // camera.pos.x - size.0 as i32 / 2;
    let top = offset.1; // camera.pos.y - size.1 as i32 / 2;
    let black = BLACK.into();

    for y0 in 0..size.1 as i32 {
        let y = y0 + top;
        for x0 in 0..size.0 as i32 {
            let x = x0 + left;
            let idx = match map.to_idx(x, y) {
                None => {
                    // TODO - Fancy?
                    buf.draw(x0, y0, 0, black, black);
                    continue;
                }
                Some(idx) => idx,
            };

            let needs_draw = force_draw || map.needs_draw_idx(idx);
            let needs_snapshot = memory.is_none() || map.needs_snapshot_idx(idx);
            let (visible, revealed, mapped) = match vis.get_vis_type(idx) {
                VisType::MAPPED => (false, false, true),
                VisType::REVEALED => (false, true, false),
                VisType::VISIBLE => (true, true, false),
                VisType::NONE => (false, false, false),
            };

            // Render a tile depending upon the tile type
            if needs_draw {
                // println!("draw : {}", idx);

                if revealed || mapped {
                    let (glyph, mut fg, mut bg) = match needs_snapshot {
                        true => {
                            // println!(": tile changed - {},{}", x, y);
                            let cell = map.get_cell_at_idx(idx).unwrap();
                            let tile_sprite = cell.sprite();
                            if let Some(memory) = memory.as_mut() {
                                memory.set_sprite(
                                    x,
                                    y,
                                    tile_sprite.fg,
                                    tile_sprite.bg,
                                    tile_sprite.glyph,
                                );
                                map.clear_needs_snapshot_idx(idx);
                            }
                            (tile_sprite.glyph, tile_sprite.fg, tile_sprite.bg)
                        }
                        false => match memory.as_mut().unwrap().get_sprite(x, y) {
                            Some(buf) => (buf.glyph, buf.fg.clone(), buf.bg.clone()),
                            None => (0, RGBA::new(), RGBA::new()),
                        },
                    };

                    if mapped {
                        bg = RGBA::alpha_mix(&bg, &RGBA::rgba(128, 0, 128, 128));
                        fg = RGBA::alpha_mix(&fg, &RGBA::rgba(0, 128, 0, 128));
                    } else {
                        // for item_id in map.items_at_xy(x as i32, y as i32) {
                        //     if let Some(sprite) = global_world().get_sprite(item_id) {
                        //         let sprite = sprite.borrow();
                        //         if sprite.glyph != 0 && sprite.fg.a() > 0 {
                        //             glyph = sprite.glyph;
                        //             fg = mix_colors(&fg, &sprite.fg);
                        //         }
                        //         bg = mix_colors(&bg, &sprite.bg);
                        //     }
                        // }

                        if !visible {
                            fg = RGBA::darken(&fg, 0.35); // Need to slightly dim as well
                        } else {
                            // for actor_id in map.actors_at_xy(x as i32, y as i32) {
                            //     if let Some(sprite) = global_world().get_sprite(actor_id) {
                            //         let sprite = sprite.borrow();
                            //         if sprite.glyph != 0 && sprite.fg.a() > 0 {
                            //             glyph = sprite.glyph;
                            //             fg = mix_colors(&fg, &sprite.fg);
                            //         }
                            //         bg = mix_colors(&bg, &sprite.bg);
                            //     }
                            // }

                            // TODO - ViewPortConfig in resources...
                            // if let Some(mask) = self.mask.as_ref() {
                            //     if mask.in_fov(x, y) {
                            //         bg = RGBA::alpha_mix(&bg, &RGBA::rgba(0, 128, 0, 128));
                            //     }
                            // }
                        }
                    }

                    if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_CURSOR) {
                        // bg = mix_colors(&bg, &RGBA::rgba(0, 255, 255, 128))
                        bg = RGBA::binary_inverse(&fg);
                    } else if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_HIGHLIGHTED) {
                        bg = RGBA::alpha_mix(&bg, &RGBA::rgba(255, 255, 0, 128))
                    }

                    // if map.blocked[idx] {
                    //     bg = named::YELLOW.into();
                    // }

                    buf.draw(x0, y0, glyph, fg, bg);
                    // map.clear_needs_draw_idx(idx);
                    map.set_flag_xy(x, y, CellFlags::DRAWN_THIS_FRAME);
                } else {
                    let mut bg = named::BLACK.into();
                    if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_CURSOR) {
                        bg = RGBA::rgba(0, 128, 128, 255);
                    } else if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_HIGHLIGHTED) {
                        bg = RGBA::alpha_mix(&bg, &RGBA::rgba(255, 255, 0, 128))
                    }
                    buf.print_opt(x0, y0, Some(' '), Some(named::BLACK.into()), Some(bg));
                }
            }
        }
    }

    // dump_buffer(buf);

    // self.needs_redraw = false;
}

fn draw_actors(viewport: &mut Viewport, ecs: &mut Level) {
    let (map, camera) = <(Read<Map>, Read<Camera>)>::fetch(&ecs.resources);

    let size = viewport.con.size();
    // TODO - let offset = viewport.offset;

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let left = camera.center.x - size.0 as i32 / 2;
    let top = camera.center.y - size.1 as i32 / 2;
    let bounds = Rect::with_size(left, top, size.0 as i32, size.1 as i32);

    let mut query = <(&Position, &Sprite)>::query();

    for (pos, sprite) in query.iter(&ecs.world) {
        if bounds.contains(pos.x, pos.y) {
            if !map.has_xy(pos.x, pos.y) {
                // NOTE - This is an error somewhere, but instead of panicing we just ignore it.
                continue;
            }
            if map.has_flag_xy(pos.x, pos.y, CellFlags::DRAWN_THIS_FRAME) {
                let bufx = pos.x - left;
                let bufy = pos.y - top;

                let fg = buf.get_fore(bufx, bufy).unwrap();
                let bg = buf.get_back(bufx, bufy).unwrap();

                buf.draw(
                    bufx,
                    bufy,
                    sprite.glyph,
                    RGBA::alpha_mix(fg, &sprite.fg),
                    RGBA::alpha_mix(bg, &sprite.bg),
                );
            }
        }
    }

    // dump_buffer(buf);

    // self.needs_redraw = false;
}

fn clear_needs_draw(viewport: &mut Viewport, level: &mut Level) {
    level.clear_needs_draw();

    let (mut map, mut camera) = <(Write<Map>, Write<Camera>)>::fetch_mut(&mut level.resources);

    let size = viewport.con.size();

    let left = camera.center.x - size.0 as i32 / 2;
    let top = camera.center.y - size.1 as i32 / 2;

    for y0 in 0..size.1 as i32 {
        let y = y0 + top;
        for x0 in 0..size.0 as i32 {
            let x = x0 + left;
            let _idx = match map.to_idx(x, y) {
                None => {
                    continue;
                }
                Some(idx) => idx,
            };

            if map.has_flag_xy(x, y, CellFlags::DRAWN_THIS_FRAME) {
                map.clear_flag_xy(x, y, CellFlags::DRAWN_THIS_FRAME | CellFlags::NEEDS_DRAW);
            }
        }
    }

    viewport.needs_draw = false;
    camera.clear_needs_draw();
}

#[cfg(test)]
mod test {}
