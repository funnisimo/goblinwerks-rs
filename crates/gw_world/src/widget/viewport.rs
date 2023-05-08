use std::ops::DerefMut;

use crate::camera::Camera;
use crate::fov::FOV;
use crate::level::NeedsDraw;
use crate::map::Cell;
// use crate::map::Wrap;
use crate::map::{CellFlags, Map};
use crate::memory::MapMemory;
use crate::position::Position;
use crate::sprite::Sprite;
use gw_app::color::named::BLACK;
use gw_app::color::{named, RGBA};
use gw_app::messages::Messages;
use gw_app::Panel;
use gw_app::{log, AppEvent, ScreenResult};
use gw_ecs::{Ecs, Join, ReadComp, ReadRes, SystemData, TryReadRes, TryWriteRes, World, WriteRes};
use gw_util::point::Point;
use gw_util::rect::Rect;
use gw_util::value::Value;

pub enum VisType {
    NONE,
    MAPPED,
    REVEALED,
    VISIBLE,
}

#[allow(unused_variables)]
pub trait VisSource {
    fn get_vis_type(&self, idx: usize) -> VisType {
        VisType::VISIBLE
    }
}

pub struct FovVisibility<'a> {
    fov: ReadRes<'a, FOV>,
}

impl<'a> FovVisibility<'a> {
    pub fn new(fov: ReadRes<'a, FOV>) -> Self {
        FovVisibility { fov }
    }
}

impl<'a> VisSource for FovVisibility<'a> {
    fn get_vis_type(&self, idx: usize) -> VisType {
        if self.fov.is_visible_idx(idx) {
            return VisType::VISIBLE;
        }
        if self.fov.is_revealed_idx(idx) {
            return VisType::REVEALED;
        }
        if self.fov.is_mapped_idx(idx) {
            return VisType::MAPPED;
        }
        VisType::NONE
    }
}

pub struct AlwaysVisible {}

impl AlwaysVisible {
    pub fn new() -> Self {
        AlwaysVisible {}
    }
}

impl VisSource for AlwaysVisible {}

///////////////////////////////////////////////////
///////////////////////////////////////////////////

pub struct Viewport {
    pub con: Panel,
    id: String,
    last_mouse: Point,
    needs_draw: bool,
    // lock: Lock,
}

impl Viewport {
    pub fn builder(id: &str) -> ViewPortBuilder {
        ViewPortBuilder::new(id)
    }

    fn new(builder: ViewPortBuilder) -> Self {
        let extents = builder.extents;
        let con = Panel::new(builder.size.0, builder.size.1, &builder.font)
            .with_extents(extents.0, extents.1, extents.2, extents.3);
        Viewport {
            con,
            id: builder.id,
            last_mouse: Point::new(-1, -1),
            needs_draw: true,
            // lock: builder.lock,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.con.size()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.con.resize(width, height);
        self.needs_draw = true;
    }

    fn get_map_cell(&self, world: &World, screen_pct: (f32, f32)) -> Option<Point> {
        let view_point = match self.con.mouse_point(screen_pct) {
            None => return None,
            Some(pt) => pt,
        };

        let calc_cell = move |map: &Map, camera: &Camera| {
            let map_size = map.size();
            let base_offset = camera.offset();
            let offset: Point = map.lock.lock(base_offset, *camera.size(), map_size).into();

            let map_point: Point = view_point + offset;
            match map.try_wrap_xy(map_point.x, map_point.y) {
                None => None,
                Some((x, y)) => Some(Point::new(x, y)),
            }
        };

        let (map, camera) = <(ReadRes<Map>, ReadRes<Camera>)>::fetch(world);
        calc_cell(&*map, &*camera)
    }

    pub fn input(&mut self, world: &mut World, event: &AppEvent) -> Option<ScreenResult> {
        match event {
            AppEvent::MousePos(screen_pct) => match self.get_map_cell(world, *screen_pct) {
                None => {}
                Some(cell) => {
                    if cell != self.last_mouse {
                        let mut msgs = world.write_global::<Messages>();
                        msgs.push(
                            &format!("{}_MOVE", self.id),
                            Some(Value::Point(cell.x, cell.y)),
                        );
                        self.last_mouse = cell;
                    }
                }
            },
            AppEvent::MouseDown(mouse) => match self.get_map_cell(world, mouse.pos) {
                None => {}
                Some(cell) => {
                    let mut msgs = world.write_global::<Messages>();
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

    pub fn update(&mut self, _world: &mut World) -> Option<ScreenResult> {
        None
    }

    pub fn draw_level(&mut self, world: &mut World) {
        {
            if !world.has_resource::<Camera>() {
                let map_size = world.read_resource::<Map>().size();
                let camera = Camera::new(map_size.0, map_size.1);
                world.insert_resource(camera);
            }

            let (mut map, camera, needs_draw, memory, fov) = <(
                WriteRes<Map>,
                ReadRes<Camera>,
                ReadRes<NeedsDraw>,
                TryWriteRes<MapMemory>,
                TryReadRes<FOV>,
            )>::fetch(world);

            let offset = {
                if self.con.size() != *camera.size() {
                    self.resize(camera.size().0, camera.size().1);
                }
                let base_offset = camera.offset();
                map.lock
                    .lock(base_offset, *camera.size(), map.size())
                    .into()
            };

            let viewport_needs_draw =
                { needs_draw.needs_draw() || camera.needs_draw() || self.needs_draw };

            let has_mem = memory.is_some();
            let has_fov = fov.is_some();
            match (has_mem, has_fov) {
                (true, true) => {
                    let vis = FovVisibility::new(fov.unwrap());
                    draw_map(
                        self,
                        map.deref_mut(),
                        memory,
                        &vis,
                        offset,
                        viewport_needs_draw,
                    );
                }
                (true, false) => {
                    let vis = AlwaysVisible::new();
                    draw_map(
                        self,
                        map.deref_mut(),
                        memory,
                        &vis,
                        offset,
                        viewport_needs_draw,
                    );
                }
                (false, true) => {
                    let vis = FovVisibility::new(fov.unwrap());
                    draw_map(
                        self,
                        map.deref_mut(),
                        None,
                        &vis,
                        offset,
                        viewport_needs_draw,
                    );
                }
                (false, false) => {
                    let vis = AlwaysVisible::new();
                    draw_map(
                        self,
                        map.deref_mut(),
                        None,
                        &vis,
                        offset,
                        viewport_needs_draw,
                    );
                }
            };
        }

        draw_actors(self, world);
        clear_needs_draw(self, world);
    }

    pub fn draw_map(
        &mut self,
        map: &mut Map,
        memory: Option<WriteRes<MapMemory>>,
        vis: &dyn VisSource,
        offset: (i32, i32),
        force_draw: bool,
    ) {
        let needs_draw = force_draw || self.needs_draw;
        draw_map(self, map, memory, vis, offset, needs_draw);
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
    // wrap: Wrap,
    // lock: Lock,
}

impl ViewPortBuilder {
    fn new(id: &str) -> Self {
        ViewPortBuilder {
            size: (0, 0),
            extents: (0.0, 0.0, 1.0, 1.0),
            id: id.to_string(),
            font: "DEFAULT".to_string(),
            // wrap: Wrap::None,
            // lock: Lock::None,
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

    // pub fn wrap(mut self, wrap: Wrap) -> Self {
    //     self.wrap = wrap;
    //     self
    // }

    // pub fn lock(mut self, lock: Lock) -> Self {
    //     self.lock = lock;
    //     self
    // }

    pub fn build(self) -> Viewport {
        Viewport::new(self)
    }
}

fn draw_map(
    viewport: &mut Viewport,
    map: &mut Map,
    mut memory: Option<WriteRes<MapMemory>>,
    vis: &dyn VisSource,
    offset: (i32, i32),
    force_draw: bool,
) {
    // let vis = AlwaysVisible::new();
    // let fov = global_world().get_fov(world.hero_entity()).unwrap().borrow();

    // let map_size = map.get_size();
    let view_size = viewport.con.size();
    // TODO - let offset = viewport.offset;

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let left = offset.0; // camera.pos.x - size.0 as i32 / 2;
    let top = offset.1; // camera.pos.y - size.1 as i32 / 2;
    let black = BLACK.into();

    // log(format!("map region = {:?}", map.region()));

    // let draw_bounds = Rect::with_size(
    //     0.max(view_size.0.saturating_sub(map_size.0) as i32 / 2),
    //     0.max(view_size.1.saturating_sub(map_size.1) as i32 / 2),
    //     view_size.0.min(map_size.0),
    //     view_size.1.min(map_size.1),
    // );

    for y0 in 0..view_size.1 as i32 {
        for x0 in 0..view_size.0 as i32 {
            // if !draw_bounds.contains(x0, y0) {
            //     continue;
            // }

            let idx = match map.get_wrapped_index(x0 + left, y0 + top) {
                None => {
                    // TODO - Fancy?
                    buf.draw(x0, y0, 0, black, black);
                    continue;
                }
                Some(idx) => idx,
            };

            // log(format!(
            //     "index = {} : {},{} => {:?}",
            //     idx,
            //     x0 + left,
            //     y0 + top,
            //     map.to_point(idx)
            // ));

            let needs_draw = force_draw || map.needs_draw(idx);
            let needs_snapshot = memory.is_none() || map.needs_snapshot(idx);
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
                    let (x, y) = map.to_xy(idx);
                    let (glyph, mut fg, mut bg) = match needs_snapshot {
                        true => {
                            // println!(": tile changed - {},{}", x, y);
                            let cell = map.get_cell(idx).unwrap();
                            let tile_sprite = cell.sprite();
                            if let Some(memory) = memory.as_mut() {
                                memory.set_sprite(
                                    x,
                                    y,
                                    tile_sprite.fg,
                                    tile_sprite.bg,
                                    tile_sprite.glyph,
                                );
                                map.clear_needs_snapshot(idx);
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
                            if memory.is_none() {
                                buf.draw(x0, y0, 0, black, black);
                                continue;
                            }
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

                    if map.has_flag(idx, CellFlags::IS_CURSOR) {
                        // bg = mix_colors(&bg, &RGBA::rgba(0, 255, 255, 128))
                        bg = RGBA::binary_inverse(&fg);
                    } else if map.has_flag(idx, CellFlags::IS_HIGHLIGHTED) {
                        bg = RGBA::alpha_mix(&bg, &RGBA::rgba(255, 255, 0, 128))
                    }

                    // if map.blocked[idx] {
                    //     bg = named::YELLOW.into();
                    // }

                    buf.draw(x0, y0, glyph, fg, bg);
                    // map.clear_needs_draw_idx(idx);
                    map.set_flag(idx, CellFlags::DRAWN_THIS_FRAME);
                } else {
                    let mut bg = named::BLACK.into();
                    if map.has_flag(idx, CellFlags::IS_CURSOR) {
                        bg = RGBA::rgba(0, 128, 128, 255);
                    } else if map.has_flag(idx, CellFlags::IS_HIGHLIGHTED) {
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

fn draw_actors(viewport: &mut Viewport, world: &mut World) {
    let (map, camera, position, sprite) = <(
        ReadRes<Map>,
        ReadRes<Camera>,
        ReadComp<Position>,
        ReadComp<Sprite>,
    )>::fetch(world);

    // TODO - USE REGION

    let map_size = map.size();
    let region = map.region();
    let view_size = viewport.con.size();
    // let center = camera.center();
    // let half_size = (view_size.0 / 2, view_size.1 / 2);

    let buf = viewport.con.buffer_mut();
    // DO NOT CLEAR BUFFER!!!

    let base_left = map.lock.lock_x(
        camera.center().x - view_size.0 as i32 / 2,
        view_size.0,
        map_size.0,
    );
    let base_top = map.lock.lock_y(
        camera.center().y - view_size.1 as i32 / 2,
        view_size.1,
        map_size.1,
    );

    let (left, top) = match map.try_wrap_xy(base_left, base_top) {
        None => (base_left, base_top),
        Some((x, y)) => (x, y),
    };
    let bounds = Rect::with_size(left, top, view_size.0, view_size.1);

    for (pos, sprite) in (&position, &sprite).join() {
        if !region.contains(pos.x, pos.y) {
            log("ACTOR NOT IN REGION");
            continue;
        }

        let mut vx = pos.x;
        while vx < left {
            vx += map_size.0 as i32;
        }
        let mut vy = pos.y;
        while vy < top {
            vy += map_size.1 as i32;
        }

        // log(format!(
        //     "Draw Actor - pos={:?}, vx,vy={},{}, bounds={:?}",
        //     pos, vx, vy, bounds
        // ));

        if bounds.contains(vx, vy) {
            let bufx = vx - left;
            let bufy = vy - top;

            if let Some(idx) = map.get_wrapped_index(pos.x, pos.y) {
                if map.has_flag(idx, CellFlags::DRAWN_THIS_FRAME) {
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
    }

    // dump_buffer(buf);

    // self.needs_redraw = false;
}

fn clear_needs_draw(viewport: &mut Viewport, world: &mut World) {
    let (mut map, mut camera, mut needs_draw) =
        <(WriteRes<Map>, WriteRes<Camera>, WriteRes<NeedsDraw>)>::fetch(world);

    let size = viewport.con.size();

    let left = camera.center().x - size.0 as i32 / 2;
    let top = camera.center().y - size.1 as i32 / 2;

    for y0 in 0..size.1 as i32 {
        let y = y0 + top;
        for x0 in 0..size.0 as i32 {
            let x = x0 + left;
            let idx = match map.get_wrapped_index(x, y) {
                None => {
                    continue;
                }
                Some(idx) => idx,
            };

            if map.has_flag(idx, CellFlags::DRAWN_THIS_FRAME) {
                map.clear_flag(idx, CellFlags::DRAWN_THIS_FRAME | CellFlags::NEEDS_DRAW);
            }
        }
    }

    viewport.needs_draw = false;
    needs_draw.clear();
    camera.clear_needs_draw();
}

#[cfg(test)]
mod test {}
