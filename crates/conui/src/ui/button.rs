use super::*;
use conapp::Point;
use conapp::{Buffer, MsgData};

static BUTTON: Button = Button {};

pub struct Button {}

impl Button {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut ButtonBuilder) -> (),
    {
        let mut txt = ButtonBuilder {
            node: Element::new(&BUTTON),
        };
        txt.node.borrow_mut().click = true;
        parent.add_child(txt.node.clone());
        init(&mut txt);

        if txt.node.borrow().text.is_none() {
            txt.node.set_text("");
        }
        if txt.node.size().is_none() {
            let len = txt.node.text().as_ref().unwrap().len() as u32;
            txt.node.set_size(len, 1);
        }
    }
}

impl Tag for Button {
    fn as_str(&self) -> &'static str {
        "button"
    }

    fn can_focus(&self, _el: &Element) -> bool {
        true
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        // match self {
        //     Tag::Button => {
        if el.contains(point) {
            let ret = self.handle_activate(root, el);
            return ret;
        }
        //     }
        //     Tag::Select => {
        //         return select_handle_click(el, point);
        //     }
        //     Tag::Checkbox => {
        //         if el.contains(point) {
        //             let ret = self.handle_activate(root, el);
        //             return ret;
        //         }
        //     }
        //     Tag::Custom(x) => return x.handle_click(root, el, point),
        //     _ => {
        for child in el.node.borrow().children.iter() {
            if let Some(action) = child.handle_click(root, point) {
                return Some(action);
            }
        }
        //     }
        // }
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

        // match self {
        Some(UiAction::Message(
            match el.id().as_ref() {
                None => "UI".to_string(),
                Some(id) => id.clone(),
            },
            None,
        ))
        //     Tag::Checkbox => {
        //         el.toggle_prop("checked");

        //         match el.has_prop("checked") {
        //             true => {
        //                 let text = el.attr("on_glyph").unwrap().to_string();
        //                 el.set_text(&text);
        //             }
        //             false => {
        //                 let text = el.attr("off_glyph").unwrap().to_string();
        //                 el.set_text(&text);
        //             }
        //         }

        //         let ret = Some(UiAction::Message(
        //             match el.id().as_ref() {
        //                 None => "UI".to_string(),
        //                 Some(id) => id.clone(),
        //             },
        //             match el.has_prop("checked") {
        //                 false => None,
        //                 true => el.value(),
        //             },
        //         ));
        //         ret
        //     }
        //     Tag::Custom(x) => return x.handle_activate(root, el),
        //     _ => None,
        // }
        // None
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_text(el, buf);
    }
}

////////////////////////////////////////

pub struct ButtonBuilder {
    node: Element,
}

impl ButtonBuilder {
    pub fn text(&self, text: &str) -> &Self {
        self.node.set_text(text);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn value(&self, val: Option<MsgData>) -> &Self {
        self.node.set_value(val);
        self
    }

    pub fn width(&self, width: u32) -> &Self {
        let height = self.node.size().unwrap_or((0, 1)).1;
        let current = self.node.size().unwrap_or((0, height));
        self.node.set_size(width, current.1);
        self
    }

    pub fn activate(&self, func: Box<UiActionFn>) -> &Self {
        self.node.set_activate(func);
        self
    }

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

impl Padded for ButtonBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for ButtonBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Keyed for ButtonBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

// impl ParentNode for Button {
//     fn add_child(&mut self, node: Element) {
//         panic!("Button nodes cannot have children!");
//     }
// }

#[cfg(test)]
mod test {

    use super::*;
    use crate::ui;
    use conapp::Point;

    #[test]
    fn simple_button() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Button::new(body, |button| {
                button.id("HELLO").text("Hello World");
            });
        });

        let button = ui.find_by_id("HELLO").unwrap();
        assert_eq!(button.text().as_ref().unwrap(), "Hello World");

        assert!(button.pos().is_some());
        assert_eq!(button.size().unwrap(), (11, 1));
        assert!(button.contains(Point::new(0, 0)));
        assert!(!button.contains(Point::new(9, 8)));
    }
}
