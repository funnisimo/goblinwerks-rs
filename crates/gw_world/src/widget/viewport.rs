use crate::map::{CellFlags, Map};
use crate::memory::MapMemory;
use gw_app::color::{named, RGBA};
use gw_app::ecs::{systems::ResourceSet, Read, Write};
use gw_app::messages::Messages;
use gw_app::{log, AppEvent, Buffer, ScreenResult};
use gw_app::{Ecs, Panel};
use gw_util::point::Point;
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

pub struct Viewport {
    con: Panel,
    id: String,
    last_mouse: Point,
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
        }
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
        // Do we need to draw?
        draw_map(self, ecs);
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

fn draw_map(viewport: &mut Viewport, ecs: &mut Ecs) {
    let (mut map, mut memory) = <(Write<Map>, Write<MapMemory>)>::fetch_mut(&mut ecs.resources);

    let vis = AlwaysVisible::new();
    // let fov = global_world().get_fov(world.hero_entity()).unwrap().borrow();

    let size = viewport.con.size();
    // TODO - let offset = viewport.offset;
    let viewport_needs_draw = false; // viewport.needs_draw || map.needs_draw();

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    for y in 0..size.1 as i32 {
        for x in 0..size.0 as i32 {
            let idx = match map.to_idx(x, y) {
                None => continue,
                Some(idx) => idx,
            };

            let needs_draw = map.needs_draw_idx(idx) || viewport_needs_draw;
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
                                    x,
                                    y,
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

                    buf.draw(x, y, glyph, fg, bg);
                    map.clear_needs_draw_idx(idx);
                } else {
                    let mut bg = named::BLACK.into();
                    if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_CURSOR) {
                        bg = RGBA::rgba(0, 128, 128, 255);
                    } else if map.has_flag_xy(x as i32, y as i32, CellFlags::IS_HIGHLIGHTED) {
                        bg = RGBA::alpha_mix(&bg, &RGBA::rgba(255, 255, 0, 128))
                    }
                    buf.print_opt(x, y, Some(' '), Some(named::BLACK.into()), Some(bg));
                }
            }
        }
    }

    // dump_buffer(buf);

    // self.needs_redraw = false;
}

#[cfg(test)]
mod test {}