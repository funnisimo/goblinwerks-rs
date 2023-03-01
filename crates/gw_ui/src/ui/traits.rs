use super::*;
use crate::css::Style;
use gw_app::{log, KeyEvent};
use std::sync::Arc;

pub trait ParentNode {
    fn el(&self) -> &Element;

    fn add_child(&self, child: Element) {
        self.el().add_child(child);
    }
}

pub fn root_size(el: &Element) -> (u32, u32) {
    let mut current = el.clone();
    while let Some(parent) = current.parent() {
        current = parent;
    }
    current.size().unwrap()
}

pub fn inner_size_hint(el: &Element) -> Option<(u32, u32)> {
    if let Some(pos) = el.pos() {
        let base_size = root_size(el);
        let max_size = (
            base_size.0.saturating_sub(pos.0 as u32),
            base_size.1.saturating_sub(pos.1 as u32),
        );

        let res = match el.size() {
            None => el.to_inner_size(Some(max_size)),
            Some((0, x)) => el.to_inner_size(Some((max_size.0, (x as u32).min(max_size.1)))),
            Some((x, 0)) => el.to_inner_size(Some(((x as u32).min(max_size.0), max_size.1))),
            Some((x, y)) => el.to_inner_size(Some((
                (x as u32).min(max_size.0),
                (y as u32).min(max_size.1),
            ))),
        };

        log(format!(
            "- inner_size_hint({}) :: pos={:?}, root_size={:?} => max_size={:?} => size={:?}, result={:?}",
            element_path(el),
            pos,
            base_size,
            max_size,
            el.size(),
            res
        ));

        return res;
    }

    let res = match el.inner_size() {
        None => match el.parent() {
            None => return None,
            Some(parent) => el.to_inner_size(inner_size_hint(&parent)),
        },
        Some((0, _)) | Some((_, 0)) => match el.parent() {
            None => el.inner_size(),
            Some(parent) => Some(calc_common_size(
                el.inner_size(),
                el.to_inner_size(inner_size_hint(&parent)),
            )),
        },
        Some((w, h)) => Some((w, h)),
    };

    log(format!(
        "- inner_size_hint({}) :: NO_POS, inner_size={:?} => result={:?}",
        element_path(el),
        el.inner_size(),
        res
    ));
    res
}

pub trait Padded {
    fn el(&self) -> &Element;

    fn pad(&self, pad: u32) -> &Self {
        self.el().set_pad(pad);
        self
    }

    fn pad_top(&self, pad: u32) -> &Self {
        self.el().set_pad_top(pad);
        self
    }

    fn pad_bottom(&self, pad: u32) -> &Self {
        self.el().set_pad_bottom(pad);
        self
    }

    fn pad_left(&self, pad: u32) -> &Self {
        self.el().set_pad_left(pad);
        self
    }

    fn pad_right(&self, pad: u32) -> &Self {
        self.el().set_pad_right(pad);
        self
    }
}

pub trait Margined {
    fn el(&self) -> &Element;

    fn margin(&self, margin: u32) -> &Self {
        self.el().set_margin(margin);
        self
    }

    fn margin_top(&self, margin: u32) -> &Self {
        self.el().set_margin_top(margin);
        self
    }

    fn margin_bottom(&self, margin: u32) -> &Self {
        self.el().set_margin_bottom(margin);
        self
    }

    fn margin_left(&self, margin: u32) -> &Self {
        self.el().set_margin_left(margin);
        self
    }

    fn margin_right(&self, margin: u32) -> &Self {
        self.el().set_margin_right(margin);
        self
    }
}

pub trait Positioned {
    fn el(&self) -> &Element;

    // fn outer_pos(&self, x: i32, y: i32) -> &Self {
    //     self.el().set_outer_pos(x, y);
    //     self
    // }
    fn pos(&self, x: i32, y: i32) -> &Self {
        // self.outer_pos(x, y)
        self.el().set_pos(x, y);
        self
    }

    // fn get_outer_pos(&self) -> (i32, i32) {
    //     let pad = self.el().pad();
    //     let pos = self.el().pos().unwrap_or((0, 0));
    //     (pos.0 + pad[0] as i32, pos.1 + pad[1] as i32)
    // }
    // fn get_pos(&self) -> Option<(i32, i32)> {
    //     self.el().pos()
    // }
    // fn get_inner_pos(&self) -> (i32, i32) {
    //     let margin = self.el().margin();
    //     let pos = self.el().pos().unwrap_or((0, 0));
    //     (pos.0 + margin[0] as i32, pos.1 + margin[1] as i32)
    // }

    // fn outer_size(&self, width: u32, height: u32) -> &Self {
    //     let pad = self.el().pad();

    //     self.el()
    //         .set_size(width - pad[0] - pad[2], height - pad[1] - pad[3]);
    //     self
    // }
    fn size(&self, width: u32, height: u32) -> &Self {
        self.el().set_size(width, height);
        self
    }
    // fn inner_size(&self, width: u32, height: u32) -> &Self {
    //     let margin = self.el().margin();
    //     self.el().set_size(
    //         width + margin[0] + margin[2],
    //         height + margin[1] + margin[3],
    //     );
    //     self
    // }

    // fn outer_width(&self, width: u32) -> &Self {
    //     let pad = self.el().pad();
    //     self.width(width - pad[0] - pad[2]);
    //     self
    // }
    fn width(&self, width: u32) -> &Self {
        let current = self.el().size().unwrap_or((0, 0));
        self.el().set_size(width, current.1);
        self
    }
    // fn inner_width(&self, width: u32) -> &Self {
    //     let margin = self.el().margin();
    //     self.width(width + margin[0] + margin[2]);
    //     self
    // }

    fn height(&self, height: u32) -> &Self {
        let current = self.el().size().unwrap_or((0, 0));
        self.el().set_size(current.0, height);
        self
    }

    // fn get_outer_size(&self) -> (u32, u32) {
    //     let pad = self.el().pad();
    //     let size = self.el().size().unwrap_or((0, 0));
    //     (size.0 + pad[0], size.1 + pad[1])
    // }
    // fn get_size(&self) -> Option<(u32, u32)> {
    //     self.el().size()
    // }
    // fn get_inner_size(&self) -> (u32, u32) {
    //     let margin = self.el().margin();
    //     let size = self.el().size().unwrap_or((0, 0));
    //     (size.0 - margin[0], size.1 - margin[1])
    // }

    fn anchor(&self, anchor: Align) -> &Self {
        self.el().set_anchor(anchor);
        self
    }
    fn vanchor(&self, vanchor: Align) -> &Self {
        self.el().set_vanchor(vanchor);
        self
    }
}

pub trait Keyed {
    fn el(&self) -> &Element;

    fn clear_keys(&self) -> &Self {
        self.el().borrow_mut().keys.clear();
        self
    }

    fn bind_key<K: Into<KeyEvent>>(&self, key: K, action: Box<UiActionFn>) -> &Self {
        let el = self.el();
        el.borrow_mut().keys.insert(key.into(), action);
        self
    }

    fn activate_key<K: Into<KeyEvent>>(&self, key: K) -> &Self {
        let el = self.el();
        let id = match el.borrow().id {
            None => panic!("You must set ID before using activate_key."),
            Some(ref id) => id.clone(),
        };
        el.borrow_mut()
            .keys
            .insert(key.into(), UiAction::activate(id));
        self
    }
}

pub trait Styled {
    fn el(&self) -> &Element;

    fn fg(&self, fg: &str) -> &Self {
        let mut el = self.el().borrow_mut();
        match el.local_style {
            None => {
                let mut style = Style::new("$".into());
                style.set_fg_name(fg);
                el.local_style = Some(Arc::new(style));
            }
            Some(ref mut arc_style) => {
                let style = Arc::make_mut(arc_style);
                style.set_fg_name(fg);
            }
        }

        self
    }

    fn bg(&self, bg: &str) -> &Self {
        let mut el = self.el().borrow_mut();
        match el.local_style {
            None => {
                let mut style = Style::new("$".into());
                style.set_bg_name(bg);
                el.local_style = Some(Arc::new(style));

                log(format!("- Set Local BG - {:?}", el.local_style));
            }
            Some(ref mut arc_style) => {
                let style = Arc::make_mut(arc_style);
                style.set_bg_name(bg);
            }
        }

        self
    }
}
