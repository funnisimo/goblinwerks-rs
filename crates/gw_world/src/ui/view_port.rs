use crate::map::{CellFlags, Map};
use crate::memory::MapMemory;
use gw_app::color::{named, RGBA};
use gw_app::console::dump_buffer;
use gw_app::ecs::{systems::ResourceSet, Read, Write};
use gw_app::Ecs;
use gw_app::{log, Buffer};
use gw_ui::ui::*;
use gw_util::point::Point;

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

static VIEW_PORT: ViewPort = ViewPort {};

pub struct ViewPort {}

impl ViewPort {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut ViewPortBuilder) -> (),
    {
        let node = Element::new(&VIEW_PORT);
        parent.add_child(node.clone());
        node.set_click(true);

        let mut builder = ViewPortBuilder { node: node.clone() };
        init(&mut builder);

        if node.size().is_none() {
            // set as parent size
        }
    }
}

impl Tag for ViewPort {
    fn as_str(&self) -> &'static str {
        "viewport"
    }

    fn can_focus(&self, _el: &Element) -> bool {
        true
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        // match self {
        //     Tag::ViewPort => {
        if el.contains(point) {
            let ret = self.handle_activate(root, el);
            return ret;
        }

        for child in el.children() {
            if let Some(action) = child.handle_click(root, point) {
                return Some(action);
            }
        }
        //     }
        // }
        None
    }

    fn draw(&self, el: &Element, buf: &mut Buffer, ecs: &mut Ecs) {
        draw_map(el, buf, ecs);
    }
}

////////////////////////////////////////

pub struct ViewPortBuilder {
    node: Element,
}

impl ViewPortBuilder {
    pub fn text(&self, text: &str) -> &Self {
        self.node.set_text(text);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    // pub fn width(&self, width: u32) -> &Self {
    //     let height = self.node.size().unwrap_or((0, 1)).1;
    //     let current = self.node.size().unwrap_or((0, height));
    //     self.node.set_size(width, current.1);
    //     self
    // }

    // pub fn activate(&self, func: Box<UiActionFn>) -> &Self {
    //     self.node.set_activate(func);
    //     self
    // }

    // pub fn pos(&self, x: i32, y: i32) -> &Self {
    //     self.node.pos = Some((x, y));
    //     self
    // }

    // pub fn size(&self, width: u32, height: u32) -> &Self {
    //     self.node.set_size(width, height);
    //     self
    // }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    pub fn focus(&self) -> &Self {
        self.node.add_prop("focus");
        self
    }
}

impl Padded for ViewPortBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for ViewPortBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Keyed for ViewPortBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

// impl ParentNode for ViewPort {
//     fn add_child(&mut self, node: Element) {
//         panic!("ViewPort nodes cannot have children!");
//     }
// }

fn draw_map(el: &Element, buf: &mut Buffer, ecs: &mut Ecs) {
    let (mut map, mut memory) = <(Write<Map>, Write<MapMemory>)>::fetch_mut(&mut ecs.resources);

    let vis = AlwaysVisible::new();
    // let fov = global_world().get_fov(world.hero_entity()).unwrap().borrow();

    let pos = el.inner_pos().unwrap();
    let size = el.inner_size().unwrap();
    // TODO - let offset = viewport_data.offset;

    for dy in 0..size.1 as i32 {
        let y = dy + pos.1;
        for dx in 0..size.0 as i32 {
            let x = dx + pos.0;

            let idx = match map.to_idx(x, y) {
                None => continue,
                Some(idx) => idx,
            };

            let needs_draw = true; // map.needs_draw_idx(idx); // self.needs_draw
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
mod test {

    use super::*;

    #[test]
    fn parent_size() {
        let ui = page((80, 50), "DEFAULT", |body| {
            ViewPort::new(body, |button| {
                button.id("MAP");
            });
        });

        let viewport = ui.find_by_id("MAP").unwrap();

        assert_eq!(viewport.pos().unwrap(), (0, 0));
        assert_eq!(viewport.size().unwrap(), (80, 50));
    }

    #[test]
    fn set_size() {
        let ui = page((80, 50), "DEFAULT", |body| {
            ViewPort::new(body, |button| {
                button.id("MAP").size(40, 40).pos(10, 10);
            });
        });

        let viewport = ui.find_by_id("MAP").unwrap();

        assert_eq!(viewport.pos().unwrap(), (10, 10));
        assert_eq!(viewport.size().unwrap(), (40, 40));
    }
}
