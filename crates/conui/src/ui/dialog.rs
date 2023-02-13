use super::*;
use conapp::{Buffer, VirtualKeyCode};

static DIALOG: Dialog = Dialog {};

pub struct Dialog {}

impl Dialog {
    pub(super) fn new<F>(page_size: (u32, u32), init: F) -> Element
    where
        F: FnOnce(&mut DialogBuilder) -> (),
    {
        let node = Element::new(&DIALOG);
        node.bind_key(VirtualKeyCode::Tab, UiAction::focus_next());
        node.bind_key((VirtualKeyCode::Tab, true), UiAction::focus_prev());
        node.set_align(Align::CENTER); // default in center
        node.set_valign(Align::MIDDLE); // default in middle
        node.set_size(page_size.0, page_size.1);

        let mut dlg = DialogBuilder { node: node.clone() };
        init(&mut dlg);

        if node.size().unwrap() == page_size {
            let size = node.children_size();
            node.set_size(size.0, size.1);
        }
        println!(
            "- dialog size={:?}, full_size={:?}",
            node.size().unwrap(),
            node.outer_size()
        );

        node.layout_children();
        node
    }
}

impl Tag for Dialog {
    fn as_str(&self) -> &'static str {
        "dialog"
    }

    fn layout_children(&self, el: &Element) -> () {
        body_layout_children(el);
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_body(el, buf);
    }
}

///////////////////////////////////

pub struct DialogBuilder {
    node: Element,
}

impl DialogBuilder {
    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    pub fn center(&self) -> &Self {
        self.node.set_align(Align::Center);
        self
    }

    pub fn middle(&self) -> &Self {
        self.node.set_valign(Align::Center);
        self
    }

    pub fn pos(&self, x: i32, y: i32) -> &Self {
        self.node.set_pos(x, y);
        self
    }

    pub fn size(&self, width: u32, height: u32) -> &Self {
        self.node.set_size(width, height);
        self
    }

    pub fn anchor(&self, anchor: Align) -> &Self {
        self.node.set_anchor(anchor);
        self
    }

    pub fn vanchor(&self, vanchor: Align) -> &Self {
        self.node.set_vanchor(vanchor);
        self
    }
}

impl Positioned for DialogBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl ParentNode for DialogBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Keyed for DialogBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ui::dialog;

    #[test]
    fn simple_dialog() {
        let ui = dialog((80, 50), "DEFAULT", |dlg| {
            dlg.id("DIALOG");

            Frame::new(dlg, |frame| {
                frame.id("FRAME").title("Taco Tuesday");

                Text::new(frame, |txt| {
                    txt.text("Welcome to the UI.").id("TEXT").width(30);
                });

                Text::new(frame, |txt| {
                    txt.text("Isn't this great?").id("TEXT2");
                });
            });
        });

        let dlg = ui.root();
        assert_eq!(dlg.id().as_ref().unwrap(), "DIALOG");

        let frame = dlg.get_child_by_tag("frame").unwrap();
        assert_eq!(frame.text().as_ref().unwrap(), "Taco Tuesday");
        assert_eq!(frame.borrow().children.len(), 2);
    }

    #[test]
    fn positioned_dialog() {
        let ui = dialog((80, 50), "DEFAULT", |dlg| {
            dlg.id("DIALOG").pos(15, 2);

            Frame::new(dlg, |frame| {
                frame.title("Taco Tuesday").size(20, 20);

                Text::new(frame, |txt| {
                    txt.text("Welcome to the UI.").id("TEXT").width(30);
                });

                Text::new(frame, |txt| {
                    txt.text("Isn't this great?").id("TEXT2");
                });
            });
        });

        assert_eq!(
            *ui.console.extents(),
            (15.0 / 80.0, 2.0 / 50.0, 35.0 / 80.0, 22.0 / 50.0)
        );

        let dlg = ui.root();
        assert_eq!(dlg.id().as_ref().unwrap(), "DIALOG");
        assert_eq!(dlg.pos().unwrap(), (0, 0));
        assert_eq!(dlg.size().unwrap(), (20, 20));

        let frame = dlg.get_child_by_tag("frame").unwrap();
        assert_eq!(frame.parent().unwrap(), dlg);
        assert_eq!(frame.text().as_ref().unwrap(), "Taco Tuesday");
        assert_eq!(frame.borrow().children.len(), 2);
        assert_eq!(frame.pos().unwrap(), (15, 2));
        assert_eq!(frame.size().unwrap(), (20, 20));
    }
}
