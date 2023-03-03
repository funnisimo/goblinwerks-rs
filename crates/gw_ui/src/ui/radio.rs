use super::*;
use gw_app::Ecs;
use gw_app::{log, text::colored_line_len, Buffer, KeyEvent, MsgData, VirtualKeyCode};
use gw_util::point::Point;
use std::cmp::max;

static RADIO: Radio = Radio {};

pub struct Radio {}

impl Radio {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: Fn(&mut RadioBuilder) -> (),
    {
        let el = Element::new(&RADIO);
        el.set_attr("off_glyph", "-".into());
        el.set_attr("on_glyph", "*".into());
        el.set_attr("space", 1.into());
        let mut builder = RadioBuilder::new(el.clone());
        parent.add_child(el.clone());

        log("NEW RADIO");

        init(&mut builder);

        if let Some(spacing) = el.attr("spacing") {
            log("adjusting spacing");
            let space: u32 = spacing.try_into().unwrap();
            let b_el = el.borrow();
            let mut iter = b_el.children.iter();
            if let Some(prior) = iter.next() {
                let mut prior_pad_bottom = prior.pad()[3];
                for ch in iter {
                    let [_, pad_top, _, pad_bottom] = ch.pad();
                    if prior_pad_bottom + pad_top < space {
                        log("- adding pad_top");
                        ch.set_pad_top(space - prior_pad_bottom);
                    }
                    prior_pad_bottom = pad_bottom;
                }
            }
        }

        // finish list
        let children_size = el.children_size();
        log(format!("children - size = {:?}", children_size));
        match el.size().unwrap_or((0, 0)) {
            (0, 0) => {
                el.set_size(children_size.0, children_size.1);
            }
            (0, x) => {
                el.set_size(children_size.0, x);
                let mut height = 0;
                for ch in el.borrow().children.iter() {
                    if height >= x {
                        ch.add_prop("hidden");
                    } else {
                        let size = ch.size().unwrap();
                        if height + size.1 > x {
                            ch.set_size(size.0, x - height);
                            height = x;
                        } else {
                            height += size.1;
                        }
                    }
                }
            }
            (x, 0) => {
                for ch in el.borrow().children.iter() {
                    let size = ch.size().unwrap();
                    ch.set_size(x, size.1);
                }
                let child_size = el.children_size();
                el.set_size(x, child_size.1);
            }
            (w, h) => {
                let mut height = 0;
                for ch in el.borrow().children.iter() {
                    let size = ch.outer_size();
                    if height >= h {
                        ch.add_prop("hidden");
                    } else {
                        if height + size.1 > h {
                            ch.set_size(w, h - height);
                            height = h;
                        } else {
                            height += size.1;
                            ch.set_size(w, size.1);
                        }
                    }
                }
            }
        }
    }
}

impl Tag for Radio {
    fn as_str(&self) -> &'static str {
        "radio_group"
    }

    fn value(&self, el: &Element) -> Option<MsgData> {
        // if let Some(val) = &el.borrow().value {
        //     return Some(val.clone());
        // }

        let mut items: Vec<MsgData> = Vec::new();
        let use_index = el.has_prop("index");
        for (index, ch) in el.children().enumerate() {
            if ch.has_prop("checked") {
                if use_index {
                    items.push(MsgData::Index(index));
                } else {
                    items.push(ch.value().unwrap());
                }
            }
        }
        if items.is_empty() {
            return None;
        } else {
            return items.drain(0..1).next();
        };
    }
}

pub struct RadioBuilder {
    node: Element,
}

impl RadioBuilder {
    fn new(node: Element) -> Self {
        RadioBuilder { node }
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.node.set_id(id);
        self
    }

    pub fn item(&mut self, text: &str) -> &mut Self {
        let align = self.node.align();
        let on_glyph = self.node.attr("on_glyph").unwrap().to_string();
        let off_glyph = self.node.attr("off_glyph").unwrap().to_string();
        let nowrap = self.node.has_prop("nowrap");
        let space: i32 = self.node.attr("space").unwrap().try_into().unwrap();
        RadioItem::new(self, move |item| {
            if let Some(align) = align {
                item.align(align);
            }
            item.glyphs(&off_glyph, &on_glyph);
            item.space(space);
            if nowrap {
                item.nowrap();
            }
            item.text(text);
            item.value(text.into());
        });
        self
    }

    pub fn with_item<F>(&mut self, text: &str, init: F) -> &mut Self
    where
        F: Fn(&mut RadioItemBuilder) -> (),
    {
        let align = self.node.align();
        let on_glyph = self.node.attr("on_glyph").unwrap().to_string();
        let off_glyph = self.node.attr("off_glyph").unwrap().to_string();
        let nowrap = self.node.has_prop("nowrap");
        let space: i32 = self.node.attr("space").unwrap().try_into().unwrap();
        // let nowrap = self.node.has_prop("nowrap");
        RadioItem::new(self, move |item| {
            if let Some(align) = align {
                item.align(align);
            }
            item.glyphs(&off_glyph, &on_glyph);
            item.text(text);
            item.space(space);
            if nowrap {
                item.nowrap();
            }
            init(item);

            if item.node.value().is_none() {
                let text = item.label.text().as_ref().unwrap().into();
                item.value(text);
            }
        });
        self
    }

    pub fn align(&mut self, align: Align) -> &mut Self {
        self.node.set_align(align); // Need to give to items
        self
    }

    pub fn glyphs(&mut self, off_glyph: &str, on_glyph: &str) -> &mut Self {
        self.node.set_attr("off_glyph", off_glyph.into()); // Need to give to items
        self.node.set_attr("on_glyph", on_glyph.into()); // Need to give to items
        self
    }

    pub fn indent(&mut self, indent: u32) -> &mut Self {
        self.node.set_margin_left(indent);
        self
    }

    // SPACING - space between list items
    pub fn spacing(&mut self, lines: u32) -> &mut Self {
        self.node.set_attr("spacing", (lines as i32).into());
        self
    }

    // SPACE - space between glyph and label
    pub fn space(&mut self, chars: u32) -> &mut Self {
        self.node.set_attr("space", (chars as i32).into());
        self
    }

    pub fn index(&mut self) -> &mut Self {
        self.node.add_prop("index");
        self
    }

    pub fn class(&mut self, name: &str) -> &mut Self {
        self.node.add_class(name);
        self
    }

    // pub fn nowrap(&self) -> &Self {
    //     self.node.add_prop("nowrap"); // Need to give to items
    //     self
    // }
}

impl Padded for RadioBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Margined for RadioBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for RadioBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl ParentNode for RadioBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

//////////////////////////////////////////

static RADIO_ITEM: RadioItem = RadioItem {};

pub struct RadioItem {}

impl RadioItem {
    fn new<F>(parent: &RadioBuilder, init: F) -> ()
    where
        F: Fn(&mut RadioItemBuilder) -> (),
    {
        let label = Element::new(&LABEL);
        label.set_text(""); // default text
        let node = Element::new(&RADIO_ITEM);
        node.set_attr("off_glyph", "-".into());
        node.set_attr("on_glyph", "*".into());
        node.set_attr("space", 1.into());
        node.add_child(label.clone());
        parent.node.add_child(node.clone());

        let mut builder = RadioItemBuilder {
            node: node.clone(),
            label: label.clone(),
        };

        log("NEW RADIO ITEM");

        init(&mut builder);

        let off_text: String = node.attr("off_glyph").unwrap().to_string();
        let on_text: String = node.attr("off_glyph").unwrap().to_string();
        let space: i32 = node.attr("space").unwrap().try_into().unwrap();
        let prefix_size =
            max(colored_line_len(&off_text), colored_line_len(&on_text)) as u32 + space as u32;
        let inner_size = calc_common_size(
            node.inner_size(),
            node.to_inner_size(inner_size_hint(&parent.node)),
        );

        match node.has_prop("checked") {
            false => node.set_text(&off_text),
            true => node.set_text(&on_text),
        }

        log(format!(
            "Finish radio Item - using={:?}, inner_size={:?}, inner_size_hint={:?}",
            inner_size,
            node.inner_size(),
            inner_size_hint(&parent.node)
        ));
        text_set_size(&label, Some(inner_size));
        let child_size = node.children_size();
        node.set_size(child_size.0 + prefix_size, child_size.1);
    }
}

impl Tag for RadioItem {
    fn as_str(&self) -> &'static str {
        "radio"
    }

    fn can_focus(&self, _el: &Element) -> bool {
        true
    }

    fn to_inner_size(&self, el: &Element, size: (u32, u32)) -> (u32, u32) {
        let off_text: String = el.attr("off_glyph").unwrap().to_string();
        let on_text: String = el.attr("on_glyph").unwrap().to_string();
        let space: i32 = el.attr("space").unwrap().try_into().unwrap();
        let prefix_size =
            max(colored_line_len(&off_text), colored_line_len(&on_text)) as u32 + space as u32;
        let margin = el.margin();

        (
            size.0.saturating_sub(margin[0] + margin[2] + prefix_size),
            size.1.saturating_sub(margin[1] + margin[3]),
        )
    }

    fn layout_children(&self, el: &Element) -> () {
        let mut child_pos = el.pos().unwrap();
        // let size = el.size().unwrap();

        let margin = el.margin();
        child_pos.0 += margin[0] as i32;
        child_pos.1 += margin[1] as i32;

        let text = el.text();
        let space: i32 = el.attr("space").unwrap().try_into().unwrap();
        let prefix_width = text.as_ref().unwrap().len() as i32 + space;
        child_pos.0 += prefix_width;

        log(format!("layout list item children - {}", element_path(el)));
        for child in el.borrow().children.iter() {
            log(format!("- {:?} @ {:?}", element_path(child), child_pos));
            child.set_outer_pos(child_pos.0, child_pos.1); // calls layout_children
            let (_, child_height) = child.outer_size();
            child_pos.1 += child_height as i32;
            // let tag = child.node.borrow().tag;
            // tag.layout_children(&child);
        }
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        return radio_handle_click(root, el, point);
    }

    fn handle_key(&self, root: &Element, el: &Element, key: &KeyEvent) -> Option<UiAction> {
        if let Some(action) = el.node.borrow().keys.get(key) {
            return action(root, el);
        }

        match radio_handle_key(root, el, key) {
            None => {}
            Some(action) => return Some(action),
        }

        if let Some(parent) = el.node.borrow().parent_element() {
            return parent.handle_key(root, key);
        }
        None
    }

    fn handle_activate(&self, root: &Element, el: &Element) -> Option<UiAction> {
        let parent = el.parent().unwrap();
        let id = parent.id().as_ref().unwrap().clone();

        for child in parent.children() {
            child.remove_prop("checked");
            let off_glyph = child.attr("off_glyph").unwrap().to_string();
            child.set_text(&off_glyph);
        }

        el.add_prop("checked");
        let on_glyph = el.attr("on_glyph").unwrap().to_string();
        el.set_text(&on_glyph);

        Some(UiAction::Message(id, el.value()))
    }

    fn draw(&self, el: &Element, buf: &mut Buffer, ecs: &mut Ecs) {
        draw_checkbox(el, buf, ecs);
    }
}

pub(super) fn radio_handle_click(root: &Element, el: &Element, point: Point) -> Option<UiAction> {
    if !el.contains(point) {
        return None;
    }

    log(format!("click radio"));

    el.handle_activate(root)
}

pub(super) fn radio_handle_key(root: &Element, el: &Element, key: &KeyEvent) -> Option<UiAction> {
    match key.key_code {
        VirtualKeyCode::Space | VirtualKeyCode::Return => {
            // update checked
            return el.handle_activate(root);
        }
        _ => {}
    }

    None
}

pub struct RadioItemBuilder {
    node: Element,
    label: Element,
}

impl RadioItemBuilder {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.label.set_text(text);
        self
    }

    pub fn value(&self, value: MsgData) -> &Self {
        self.node.set_value(Some(value));
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self.label.add_class(class);
        self
    }

    pub fn width(&self, width: u32) -> &Self {
        let current = self.node.size().unwrap_or((0, 0));
        self.node.set_size(width, current.1);
        self
    }

    pub fn align(&self, align: Align) -> &Self {
        self.node.set_align(align);
        self
    }

    pub fn glyphs(&self, off_glyph: &str, on_glyph: &str) -> &Self {
        self.node.set_attr("off_glyph", off_glyph.into());
        self.node.set_attr("on_glyph", on_glyph.into());
        self
    }

    pub fn space(&self, chars: i32) -> &Self {
        self.node.set_attr("space", chars.into());
        self
    }

    pub fn checked(&self) -> &Self {
        self.label.add_prop("checked");
        self
    }

    pub fn nowrap(&self) -> &Self {
        self.label.add_prop("nowrap");
        self
    }
}

impl Padded for RadioItemBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

#[cfg(test)]
mod test {
    use crate::ui::test::extract_line;

    use super::*;

    #[test]
    fn basic_list() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            Radio::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.item("Test A").with_item("Test B", |item| {});
            });
        });

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "radio_group");
        assert_eq!(list.child_count(), 2);
    }

    #[test]
    fn list_margin() {
        let ui = page((80, 50), "DEFAULT", |body| {
            Radio::new(body, |list| {
                list.margin_left(2);
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.item("Test A").with_item("Test B", |item| {});
            });
        });

        ui.dump();

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "radio_group");
        assert_eq!(list.child_count(), 2);
        assert_eq!(list.margin(), [2, 0, 0, 0]);

        let mut buffer = Buffer::new(80, 50);
        let mut ecs = Ecs::new();
        ui.root().draw(&mut buffer, &mut ecs);

        assert_eq!(extract_line(&buffer, 0, 0, 12), "\0\0-\0Test A\0\0");
        assert_eq!(extract_line(&buffer, 0, 1, 12), "\0\0-\0Test B\0\0");
    }

    #[test]
    fn list_with_width() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            Radio::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.width(10);
                list.item("Item A")
                    .item("This is a longer item and it should wrap.")
                    .item("Item C");
            });
        });

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "radio_group");
        assert_eq!(list.child_count(), 3);
        assert_eq!(list.size().unwrap(), (10, 8));
    }
}
