use crate::map::{CellFlags, Map};
use crate::memory::MapMemory;
use crate::position::Position;
use crate::sprite::Sprite;
use gw_app::color::named::BLACK;
use gw_app::color::{named, RGBA};
use gw_app::ecs::query::IntoQuery;
use gw_app::ecs::{systems::ResourceSet, Read, Write};
use gw_app::messages::Messages;
use gw_app::{log, AppEvent, Buffer, ScreenResult};
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
    pub pos: Point,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Point::new(0, 0),
        }
    }
}

pub struct Viewport {
    pub con: Panel,
    id: String,
    last_mouse: Point,
    last_camera_pos: Point,
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
            last_camera_pos: Point::new(-1, -1),
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

    pub fn input(&mut self, ecs: &mut Ecs, event: &AppEvent) -> Option<ScreenResult> {
        match event {
            AppEvent::MousePos(screen_pct) => match self.con.mouse_point(*screen_pct) {
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
            AppEvent::MouseDown(mouse) => match self.con.mouse_point(mouse.pos) {
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

    pub fn render(&mut self, ecs: &mut Ecs) {
        if !ecs.resources.contains::<Camera>() {
            let map_size = ecs.resources.get::<Map>().unwrap().get_size();
            let mut camera = Camera::new();
            camera.pos = Point::new(map_size.0 as i32 / 2, map_size.1 as i32 / 2);
            ecs.resources.insert(camera);
        }

        let viewport_needs_draw = {
            let camera = ecs.resources.get::<Camera>().unwrap();
            let viewport_needs_draw = self.needs_draw || self.last_camera_pos != camera.pos; // viewport.needs_draw || map.needs_draw();
            self.last_camera_pos = camera.pos;
            self.needs_draw = false;
            viewport_needs_draw
        };

        // Do we need to draw?
        draw_map(self, ecs, viewport_needs_draw);
        draw_actors(self, ecs);
        clear_needs_draw(self, ecs);

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

fn draw_map(viewport: &mut Viewport, ecs: &mut Ecs, needs_draw: bool) {
    let (mut map, mut memory, camera) =
        <(Write<Map>, Write<MapMemory>, Read<Camera>)>::fetch_mut(&mut ecs.resources);

    let vis = AlwaysVisible::new();
    // let fov = global_world().get_fov(world.hero_entity()).unwrap().borrow();

    let size = viewport.con.size();
    // TODO - let offset = viewport.offset;

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let left = camera.pos.x - size.0 as i32 / 2;
    let top = camera.pos.y - size.1 as i32 / 2;
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

            let needs_draw = needs_draw || map.needs_draw_idx(idx);
            let needs_snapshot = map.needs_snapshot_idx(idx);
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
                    if needs_snapshot {
                        // println!(": tile changed - {},{}", x, y);
                        match map.get_tile_at_idx(idx) {
                            None => {
                                buf.print_opt(
                                    x0,
                                    y0,
                                    Some('!'),
                                    Some(named::RED.into()),
                                    Some(named::BLACK.into()),
                                );
                                continue;
                            }
                            Some(tile) => {
                                memory.set_sprite(x, y, tile.fg, tile.bg, tile.glyph);
                                map.clear_needs_snapshot_idx(idx);
                            }
                        }
                    }

                    let (glyph, mut fg, mut bg) = match memory.get_sprite(x, y) {
                        Some(buf) => (buf.glyph, buf.fg.clone(), buf.bg.clone()),
                        None => (0, RGBA::new(), RGBA::new()),
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

fn draw_actors(viewport: &mut Viewport, ecs: &mut Ecs) {
    let (map, camera) = <(Read<Map>, Read<Camera>)>::fetch(&ecs.resources);

    let size = viewport.con.size();
    // TODO - let offset = viewport.offset;

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let left = camera.pos.x - size.0 as i32 / 2;
    let top = camera.pos.y - size.1 as i32 / 2;
    let bounds = Rect::with_size(left, top, size.0 as i32, size.1 as i32);

    let mut query = <(&Position, &Sprite)>::query();

    for (pos, sprite) in query.iter(&ecs.world) {
        if bounds.contains(pos.x, pos.y) {
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

fn clear_needs_draw(viewport: &mut Viewport, ecs: &mut Ecs) {
    let (mut map, camera) = <(Write<Map>, Read<Camera>)>::fetch_mut(&mut ecs.resources);

    let size = viewport.con.size();

    let left = camera.pos.x - size.0 as i32 / 2;
    let top = camera.pos.y - size.1 as i32 / 2;

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
}

#[cfg(test)]
mod test {}
