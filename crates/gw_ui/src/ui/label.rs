use super::*;
use gw_app::{Buffer, Ecs};

pub(super) static LABEL: Label = Label {};

pub struct Label {}

impl Tag for Label {
    fn as_str(&self) -> &'static str {
        "label"
    }

    fn draw(&self, el: &Element, buf: &mut Buffer, _app: &mut Ecs) {
        draw_text(el, buf);
    }
}

pub struct LabelBuilder {
    node: Element,
}

impl LabelBuilder {
    // pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    // where
    //     F: FnOnce(&mut Label) -> (),
    // {
    //     let mut txt = Label {
    //         node: Element::new(Tag::Label),
    //     };
    //     init(&mut txt);

    //     if !txt.node.has_any_text() {
    //         txt.node.set_text("");
    //     }
    //     if txt.node.size().is_none() {
    //         txt.node.set_size(txt.node.text().as_ref().unwrap().len() as u32, 1);
    //     }

    //     parent.add_child(txt.node);
    // }

    pub fn text(&self, text: &str) -> &Self {
        self.node.set_text(text);
        self
    }

    // pub fn id(&self, id: &str) -> &Self {
    //     self.node.set_id(id);
    //     self
    // }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    // fn width(&self, width: u32) -> &Self {
    //     let current = self.el().size().unwrap_or((0, 1));
    //     self.el().set_size(width, current.1);
    //     self
    // }

    // pub fn size(&self, width: u32, height: u32) -> &Self {
    //     self.node.set_size(width, height);
    //     self
    // }
}

// impl Padded for Label {
//     fn el(&self) -> &Element {
//         &self.node
//     }
// }
