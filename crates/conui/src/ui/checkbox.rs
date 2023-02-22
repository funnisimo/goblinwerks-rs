use super::*;
use conapp::Point;
use conapp::{console, text::colored_line_len, Buffer, KeyEvent, MsgData, VirtualKeyCode};

static CHECKBOX: Checkbox = Checkbox {};

pub struct Checkbox {}

impl Checkbox {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut CheckboxBuilder) -> (),
    {
        let label = Element::new(&LABEL);
        label.set_text(""); // default text
        let node = Element::new(&CHECKBOX);
        node.borrow_mut().click = true;
        node.set_attr("on_glyph", "X".into());
        node.set_attr("off_glyph", "-".into());
        node.set_attr("space", 1.into());

        node.add_child(label.clone());
        parent.add_child(node.clone());

        console(format!("Begin Checkbox, path={}", element_path(&node)));

        let mut checkbox = CheckboxBuilder {
            node: node.clone(),
            label: label.clone(),
        };
        init(&mut checkbox);

        console(format!("Finish Checkbox, path={}", element_path(&node)));

        let on_text: String = node.attr("on_glyph").unwrap().to_string();
        let off_text: String = node.attr("off_glyph").unwrap().to_string();
        let space: i32 = node.attr("space").unwrap().try_into().unwrap();

        let prefix_size =
            max(colored_line_len(&on_text), colored_line_len(&off_text)) as u32 + space as u32;
        let label_len = label.text().as_ref().unwrap().len() as u32;

        let inner_start = node.inner_size();
        let parent_hint = node.to_inner_size(inner_size_hint(parent.el()));
        let inner_size = calc_common_size(inner_start, parent_hint);
        console(format!(
            "checkbox - set label size using={:?} :: inner_size={:?}, parent={:?}",
            inner_size, inner_start, parent_hint
        ));

        text_set_size(&label, Some(inner_size));
        let label_size = label.size().unwrap();
        let node_size = match node.size() {
            None => (label_size.0 + prefix_size, label_size.1),
            Some((0, x)) => (label_size.0 + prefix_size, x),
            Some((x, 0)) => (x, label_size.1),
            Some((x, y)) => (x, y),
        };
        node.set_size(node_size.0, node_size.1);

        console(format!("        - label size => {:?}", label_size));
        console(format!(
            "        - node size  => {:?}",
            node.size().unwrap()
        ));

        match node.has_prop("checked") {
            false => {
                node.set_text(&off_text);
                node.set_value(None);
            }
            true => {
                node.set_text(&on_text);
                if let Some(count) = node.attr("count") {
                    node.set_value(Some(count));
                } else {
                    node.set_value(Some(true.into()));
                }
            }
        }

        console(format!("CHECKBOX - size={:?}", node.size().unwrap()));
    }
}

impl Tag for Checkbox {
    fn as_str(&self) -> &'static str {
        "checkbox"
    }

    fn can_focus(&self, _el: &Element) -> bool {
        true
    }

    fn to_inner_size(&self, el: &Element, size: (u32, u32)) -> (u32, u32) {
        let on_text: String = el.attr("on_glyph").unwrap().to_string();
        let off_text: String = el.attr("off_glyph").unwrap().to_string();
        let space: i32 = el.attr("space").unwrap().try_into().unwrap();

        let prefix_size =
            max(colored_line_len(&on_text), colored_line_len(&off_text)) as u32 + space as u32;
        let margin = el.margin();
        (
            size.0.saturating_sub(prefix_size + margin[0] + margin[1]),
            size.1.saturating_sub(margin[1] + margin[3]),
        )
    }

    fn layout_children(&self, el: &Element) -> () {
        layout_checkbox(el);
    }

    fn value(&self, el: &Element) -> Option<MsgData> {
        if el.has_prop("checked") {
            return el.node.borrow().value.clone();
        }
        None
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        if el.contains(point) {
            let ret = self.handle_activate(root, el);
            return ret;
        }

        for child in el.node.borrow().children.iter() {
            if let Some(action) = child.handle_click(root, point) {
                return Some(action);
            }
        }

        None
    }

    fn handle_key(&self, root: &Element, el: &Element, key: &KeyEvent) -> Option<UiAction> {
        if let Some(action) = el.node.borrow().keys.get(key) {
            return action(root, el);
        }

        match checkbox_handle_key(root, el, key) {
            None => {}
            Some(action) => return Some(action),
        }

        if let Some(parent) = el.node.borrow().parent_element() {
            return parent.handle_key(root, key);
        }
        None
    }

    fn handle_activate(&self, root: &Element, el: &Element) -> Option<UiAction> {
        // TODO - Move to Element?
        if let Some(func) = el.activate() {
            match func(root, el) {
                None => {}
                Some(x) => {
                    println!("handle_activate");
                    return Some(x);
                }
            }
        }

        el.toggle_prop("checked");

        if let Some(count) = el.attr("count") {
            match el.has_prop("checked") {
                true => el.set_value(Some(count)),
                false => el.set_value(None),
            }
        } else {
            match el.has_prop("checked") {
                true => el.set_value(Some(true.into())),
                false => el.set_value(None),
            }
        }

        checkbox_update(el)
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_checkbox(el, buf);
    }
}

pub struct CheckboxBuilder {
    pub(crate) node: Element,
    label: Element,
}

impl CheckboxBuilder {
    pub fn text(&self, text: &str) -> &Self {
        self.label.set_text(text);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn value(&self, val: MsgData) -> &Self {
        self.node.set_value(Some(val));
        self
    }

    pub fn width(&self, width: u32) -> &Self {
        let current = self.node.size().unwrap_or((0, 0));
        self.node.set_size(width, current.1);
        self
    }

    pub fn activate(&self, func: Box<UiActionFn>) -> &Self {
        self.node.set_activate(func);
        self
    }

    pub fn align(&self, align: Align) -> &Self {
        self.node.set_align(align);
        self
    }

    pub fn glyphs(&self, off_glyph: &str, on_glyph: &str) -> &Self {
        self.node.set_attr("on_glyph", on_glyph.into());
        self.node.set_attr("off_glyph", off_glyph.into());
        self
    }

    pub fn checked(&self) -> &Self {
        self.node.add_prop("checked");
        self
    }

    pub fn nowrap(&self) -> &Self {
        self.label.add_prop("nowrap");
        self
    }

    pub fn count(&self, glyph: &str, max: u16) -> &Self {
        self.node.set_attr("count", (max as i32).into());
        self.node.set_attr("count_glyph", glyph.into());
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self.label.add_class(class);
        self
    }

    pub fn focus(&self) -> &Self {
        self.node.add_prop("focus");
        self
    }
}

impl Padded for CheckboxBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for CheckboxBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Keyed for CheckboxBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

// impl ParentNode for Checkbox {
//     fn add_child(&mut self, node: Element) {
//         panic!("Checkbox nodes cannot have children!");
//     }
// }

fn get_count_text(text: &str, value: u32) -> String {
    let pat: Option<&str> = {
        let mut iter = text.char_indices();
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut res: Option<&str> = None;

        while let Some((i, ch)) = iter.next() {
            if ch == '#' {
                if start.is_none() {
                    start = Some(i);
                }
            } else if start.is_some() {
                end = Some(i);
                break;
            }
        }

        if let Some(start_idx) = start {
            res = match end {
                None => Some(&text[start_idx..]),
                Some(idx) => Some(&text[start_idx..idx]),
            }
        }

        res
    };

    if let Some(pat) = pat {
        if pat.len() > 0 {
            let mut num_str = format!("{}", value);
            if num_str.len() > pat.len() {
                return text.to_string();
            }

            while pat.len() > num_str.len() {
                num_str.insert(0, ' ');
            }
            return text.replace(pat, &num_str);
        }
    }
    text.to_string()
}

pub(super) fn checkbox_update(el: &Element) -> Option<UiAction> {
    match el.has_prop("checked") {
        true => {
            if let Some(count) = el.attr("count") {
                let max: i32 = count.try_into().unwrap();
                let current: i32 = el.value().unwrap().try_into().unwrap();

                let text = match current {
                    0 => {
                        el.remove_prop("checked");
                        el.attr("off_glyph").unwrap().to_string()
                    }
                    x if x == max => el.attr("on_glyph").unwrap().to_string(),
                    _ => {
                        let glyph = el.attr("count_glyph").unwrap().to_string();
                        get_count_text(&glyph, current as u32)
                    }
                };
                el.set_text(&text);
            } else {
                let text = el.attr("on_glyph").unwrap().to_string();
                el.set_text(&text);
            }
        }
        false => {
            let text = el.attr("off_glyph").unwrap().to_string();
            el.set_text(&text);
        }
    }

    let ret = Some(UiAction::Message(
        match el.id().as_ref() {
            None => "UI".to_string(),
            Some(id) => id.clone(),
        },
        match el.has_prop("checked") {
            false => None,
            true => el.value(),
        },
    ));
    ret
}

pub(super) fn checkbox_handle_key(
    root: &Element,
    el: &Element,
    key: &KeyEvent,
) -> Option<UiAction> {
    match key.key_code {
        VirtualKeyCode::Space | VirtualKeyCode::Return => {
            // update checked
            return el.handle_activate(root);
        }
        _ => {}
    }
    if let Some(count) = el.attr("count") {
        let max: i32 = count.try_into().unwrap();
        let mut current: i32 = el.value().unwrap_or(0.into()).try_into().unwrap();
        match key.key_code {
            VirtualKeyCode::Key3 if key.shift => {
                // #
                // Popup Number Prompt to get number
            }
            VirtualKeyCode::Delete => current = current / 10,
            _ => {}
        }

        if let Some(value) = match key.key_code {
            VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => Some(0),
            VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => Some(1),
            VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => Some(2),
            VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => Some(3),
            VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => Some(4),
            VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => Some(5),
            VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => Some(6),
            VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => Some(7),
            VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => Some(8),
            VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => Some(9),
            _ => None,
        } {
            current = current * 10 + value;
            if current > max {
                current = value;
                if current > max {
                    current = 0;
                }
            }
        }
        if current == 0 {
            el.set_value(None);
            el.remove_prop("checked");
        } else {
            el.set_value(Some(current.into()));
            el.add_prop("checked");
        }
        return checkbox_update(el);
    }
    None
}

pub(super) fn layout_checkbox(el: &Element) {
    let text = el.text();
    let space: i32 = el.attr("space").unwrap().try_into().unwrap();
    let prefix_width = text.as_ref().unwrap().len() as i32 + space;

    let label = el.first_child().unwrap();
    let pos = el.pos().unwrap();
    let size = el.size().unwrap();

    match el.align().unwrap_or(Align::Min) {
        Align::Max => {
            label.set_pos(pos.0, pos.1);
            label.set_size(size.0.saturating_sub(prefix_width as u32), size.1);
        }
        _ => {
            label.set_pos(pos.0 + prefix_width, pos.1);
            label.set_size(size.0.saturating_sub(prefix_width as u32), size.1);
        }
    }
}

pub(super) fn draw_checkbox(el: &Element, buf: &mut Buffer) {
    let pos = el.pos().unwrap();
    let size = el.size().unwrap();
    let style = el.style();

    let check_pos = match el.align().unwrap_or(Align::Min) {
        Align::Max => (pos.0 + size.0 as i32 - 1, pos.1),
        _ => pos,
    };

    {
        let text = el.text();
        conapp::draw::colored(buf)
            .fg(style.accent_fg())
            .bg(style.bg())
            .print(check_pos.0, check_pos.1, text.as_ref().unwrap());
    }

    // draw the label
    for child in el.children() {
        child.draw(buf);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ui;
    use conapp::Point;

    #[test]
    fn simple_check() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox.id("HELLO").text("Hello World");
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.value(), None); // Not checked
        assert!(checkbox.pos().is_some());
        assert_eq!(checkbox.size().unwrap(), (13, 1));
        assert!(checkbox.contains(Point::new(0, 0)));
        assert!(!checkbox.contains(Point::new(9, 8)));

        let label = checkbox.first_child().unwrap();
        assert_eq!(label.text().as_ref().unwrap(), "Hello World");
        assert_eq!(label.size().unwrap(), (11, 1));
    }

    #[test]
    fn sized_check() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox.id("HELLO").text("Hello World").size(20, 2);
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.text().as_ref().unwrap(), "-"); // Not checked
        assert!(checkbox.pos().is_some());
        assert_eq!(checkbox.size().unwrap(), (20, 2));
        assert!(checkbox.contains(Point::new(0, 0)));
        assert!(!checkbox.contains(Point::new(9, 8)));

        let label = checkbox.first_child().unwrap();
        assert_eq!(label.text().as_ref().unwrap(), "Hello World");
        assert_eq!(label.size().unwrap(), (18, 2));
    }

    #[test]
    fn wrapped_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox
                    .id("HELLO")
                    .text("This is a longer text that will wrap to multiple lines.")
                    .width(15);
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.text().as_ref().unwrap(), "-"); // Not checked
        assert!(checkbox.pos().is_some());
        assert_eq!(checkbox.size().unwrap(), (15, 5));

        let label = checkbox.first_child().unwrap();
        assert_eq!(
            label.text().as_ref().unwrap(),
            "This is a longer text that will wrap to multiple lines."
        );
        assert_eq!(label.size().unwrap(), (13, 5));
    }

    #[test]
    fn check_num() {
        let mut ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox
                    .id("HELLO")
                    .text("Number Check.")
                    .count("#", 10)
                    .width(15);
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.text().as_ref().unwrap(), "-"); // Not checked
        assert_eq!(checkbox.value(), None);

        // On
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(10.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "X");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "-");

        // 3
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key3.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(3.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "3");

        // 4
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key4.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(4.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "4");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "-");
    }

    #[test]
    fn check_num2() {
        let mut ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox
                    .id("HELLO")
                    .text("Number Check.")
                    .glyphs("--", "XX")
                    .count("##", 20)
                    .width(15);
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.text().as_ref().unwrap(), "--"); // Not checked
        assert_eq!(checkbox.value(), None);

        // On
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(20.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "XX");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "--");

        // 3
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key3.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(3.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), " 3");

        // 1
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key1.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(1.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), " 1");

        // 2
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key2.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(12.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "12");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "--");
    }

    #[test]
    fn check_num3() {
        let mut ui = ui::page((80, 50), "DEFAULT", |body| {
            Checkbox::new(body, |checkbox| {
                checkbox
                    .id("HELLO")
                    .text("Number Check.")
                    .count("#", 20)
                    .width(15);
            });
        });

        let checkbox = ui.find_by_id("HELLO").unwrap();
        assert_eq!(checkbox.text().as_ref().unwrap(), "-"); // Not checked
        assert_eq!(checkbox.value(), None);

        // On
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(20.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "X");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "-");

        // 3
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key3.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(3.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "3");

        // 1
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key1.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(1.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "1");

        // 2
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Key2.into()),
            Some(UiAction::Message("HELLO".to_owned(), Some(12.into())))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "#");

        // Off
        assert_eq!(
            ui.handle_key(&VirtualKeyCode::Space.into()),
            Some(UiAction::Message("HELLO".to_owned(), None))
        );
        assert_eq!(checkbox.text().as_ref().unwrap(), "-");
    }

    #[test]
    fn frame_width() {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Wrapping")
                    .pos(5, 25)
                    .width(28)
                    .id("FRAME");

                Checkbox::new(frame, |chk| {
                    chk.id("TRUNC").nowrap().text("This should be a single line of text that is not wrapped, but is instead truncated.");
                });
            });
        });

        let body = ui.root();
        let frame = ui.find_by_id("FRAME").unwrap();
        let check = ui.find_by_id("TRUNC").unwrap();
        let label = check.last_child().unwrap();

        assert_eq!(body.size().unwrap(), (80, 50));
        assert_eq!(frame.size().unwrap(), (30, 5)); // 28 is width, + 2 for border
        assert_eq!(frame.inner_size().unwrap(), (26, 1)); // 28 is width, + 2 for border, 1 for content
        assert_eq!(check.size().unwrap(), (26, 1)); // 28 is width, - 2 for margin
        assert_eq!(label.size().unwrap(), (24, 1)); // 26 for checkbox width, -2 for glyph
    }
}
