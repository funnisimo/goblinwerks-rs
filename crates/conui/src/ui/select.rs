use super::*;
use conapp::Point;
use conapp::{Buffer, KeyEvent, MsgData, VirtualKeyCode};

static SELECT: Select = Select {};

/// Select is a widget that will show options and allow you to select them.
/// The selection can either be single (default) or multiple.
/// The options can either be a list (default, single or multiple), checkboxes (multiple), or radios (single).
/// - single: value of the item selected.  By default this is the text of the item.
/// - multiple: list of values for selected items.
pub struct Select {}

impl Select {
    /// Creates a new select and calls the provided initialization function with a `SelectBuilder` you can use to customize the Select.
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut SelectBuilder) -> (),
    {
        let mut select = SelectBuilder {
            el: Element::new(&SELECT),
        };
        parent.add_child(select.el.clone());

        init(&mut select);

        let child_size = select.el.children_size();
        let margin = select.el.margin();
        let size = (
            child_size.0 + margin[0] + margin[2],
            child_size.1 + margin[1] + margin[3],
        );
        select.el.set_size(size.0, size.1);

        if select.el.has_prop("multiple") == false {
            // Should have at most 1 selected
            let mut count = 0;
            for child in select.el.borrow().children.iter() {
                if count == 0 {
                    count = 1;
                } else {
                    child.remove_prop("checked");
                }
            }
        }
        // if count == 0 && select.el.child_count() > 0 {
        //     select.el.first_child().unwrap().add_prop("checked");
        // }
    }
}

impl Tag for Select {
    fn as_str(&self) -> &'static str {
        "select"
    }

    fn can_focus(&self, _el: &Element) -> bool {
        true
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
        } else if el.has_prop("multiple") {
            return Some(MsgData::List(items));
        } else {
            return items.drain(0..1).next();
        };
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        return select_handle_click(el, point);
    }

    fn handle_key(&self, root: &Element, el: &Element, key: &KeyEvent) -> Option<UiAction> {
        if let Some(action) = el.node.borrow().keys.get(key) {
            return action(root, el);
        }

        match select_handle_key(el, key) {
            None => {}
            Some(action) => return Some(action),
        }

        if let Some(parent) = el.node.borrow().parent_element() {
            return parent.handle_key(root, key);
        }
        None
    }
}

////////////////////////////////////////

pub struct SelectBuilder {
    el: Element,
}

impl SelectBuilder {
    pub fn class(&mut self, class: &str) -> &mut Self {
        self.el.add_class(class);
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.el.set_id(id);
        self
    }

    pub fn align(&mut self, align: Align) -> &mut Self {
        self.el.set_align(align);
        self
    }

    pub fn valign(&mut self, valign: Align) -> &mut Self {
        self.el.set_valign(valign);
        self
    }

    pub fn pos(&mut self, x: i32, y: i32) -> &mut Self {
        self.el.set_outer_pos(x, y);
        self
    }

    pub fn multiple(&mut self) -> &mut Self {
        self.el.add_prop("multiple");
        self
    }

    pub fn index(&mut self) -> &mut Self {
        self.el.add_prop("index");
        self
    }

    pub fn item(&mut self, text: &str) -> &mut Self {
        self.with_item(text, |item| {
            item.value(text.into());
        });
        self
    }

    pub fn with_item<F>(&mut self, text: &str, init: F) -> &mut Self
    where
        F: FnOnce(&mut SelectItemBuilder) -> (),
    {
        SelectItem::new(self, |item| {
            item.text(text);
            init(item);
            if item.node.value().is_none() {
                item.value(text.into());
            }
        });
        self
    }
}

impl ParentNode for SelectBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Padded for SelectBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Margined for SelectBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

///////////////////////////////////////////////////////

static SELECT_ITEM: SelectItem = SelectItem {};

pub struct SelectItem {}

impl SelectItem {
    fn new<F>(parent: &mut SelectBuilder, init: F) -> ()
    where
        F: FnOnce(&mut SelectItemBuilder) -> (),
    {
        let node = Element::new(&SELECT_ITEM);
        node.set_text("");

        parent.add_child(node.clone());

        let mut option = SelectItemBuilder { node: node.clone() };
        init(&mut option);

        if node.size().is_none() {
            text_set_size(&node, parent.el.inner_size());
        }

        if option.node.value().is_none() {
            let value = option
                .node
                .id()
                .to_owned()
                .unwrap_or_else(|| option.node.text().as_ref().unwrap().clone());
            option.node.set_value(Some(value.to_owned().into()));
        }
    }
}

impl Tag for SelectItem {
    fn as_str(&self) -> &'static str {
        "item"
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_text(el, buf);
    }
}

pub struct SelectItemBuilder {
    node: Element,
}

impl SelectItemBuilder {
    pub fn text(&self, text: &str) -> &Self {
        self.node.set_text(text);
        self
    }

    pub fn value(&self, value: MsgData) -> &Self {
        self.node.set_value(Some(value));
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    pub fn disabled(&self) -> &Self {
        self.node.add_prop("disabled");
        self
    }

    pub fn checked(&self) -> &Self {
        self.node.add_prop("checked");
        self
    }
}

pub(super) fn select_handle_click(el: &Element, point: Point) -> Option<UiAction> {
    if !el.contains(point) {
        return None;
    }

    if let Some(clicked) = el.find_child(&mut |ch| ch.contains(point)) {
        let is_multiple = el.has_prop("multiple");
        el.each_child(&mut |ch| {
            ch.remove_prop("hover");
            if !is_multiple {
                ch.remove_prop("checked");
            }
        });
        clicked.add_prop("hover");
        clicked.toggle_prop("checked");
        return Some(UiAction::Stop);
    }

    None
}

pub(super) fn select_handle_key(el: &Element, key: &KeyEvent) -> Option<UiAction> {
    match key.key_code {
        VirtualKeyCode::Up => {
            // move hover
            let hover_idx = match el
                .node
                .borrow()
                .children
                .iter()
                .position(|el| el.has_prop("hover"))
            {
                None => {
                    println!("nothing checked");
                    // hover last
                    el.child_count().saturating_sub(1)
                }
                Some(idx) => {
                    println!("prev - from={}", idx);
                    match idx {
                        0 => {
                            // hover last
                            el.child_count().saturating_sub(1)
                        }
                        x => {
                            // hover prev
                            x - 1
                        }
                    }
                }
            };
            el.each_child(&mut |ch| ch.remove_prop("hover"));
            el.get_child_by_index(hover_idx).unwrap().add_prop("hover");
        }
        VirtualKeyCode::Down => {
            // move hover
            let hover_idx = match el
                .node
                .borrow()
                .children
                .iter()
                .position(|el| el.has_prop("hover"))
            {
                None => {
                    println!("nothing checked");
                    // hover first
                    0
                }
                Some(idx) => {
                    println!("next - from={}", idx);
                    match idx < el.child_count().saturating_sub(1) {
                        true => idx + 1,
                        false => 0,
                    }
                }
            };
            el.each_child(&mut |ch| ch.remove_prop("hover"));
            el.get_child_by_index(hover_idx).unwrap().add_prop("hover");
        }
        VirtualKeyCode::Space | VirtualKeyCode::Return => {
            // update checked
            match el.find_child(&mut |ch| ch.has_prop("hover")) {
                None => {
                    println!("Nothing hovered - toggling first");
                    // select the first
                    if !el.has_prop("multiple") {
                        el.each_child(&mut |ch| ch.remove_prop("checked"));
                    }
                    el.get_child_by_index(0).unwrap().toggle_prop("checked");
                    return Some(UiAction::Stop);
                }
                Some(hovered) => {
                    if !el.has_prop("multiple") {
                        el.each_child(&mut |ch| ch.remove_prop("checked"));
                    }
                    hovered.toggle_prop("checked");
                    return Some(UiAction::Stop);
                }
            }
        }
        _ => {}
    }
    None
}
