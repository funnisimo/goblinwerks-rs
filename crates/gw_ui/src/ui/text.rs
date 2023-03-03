use super::*;
use gw_app::draw;
use gw_app::log;
use gw_app::Buffer;
use gw_app::Ecs;
use gw_app::Value;
use gw_util::text::{parse_colored_lines, wrap_colored};
use std::cmp::min;

pub(crate) static TEXT: Text = Text {};

pub struct Text {}

impl Text {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut TextBuilder) -> (),
    {
        let node = Element::new(&TEXT);
        node.set_text("");
        parent.add_child(node.clone());

        let mut txt = TextBuilder { node: node.clone() };
        init(&mut txt);

        text_set_size(&node, inner_size_hint(parent.el()));
    }
}

impl Tag for Text {
    fn as_str(&self) -> &'static str {
        "text"
    }

    fn value(&self, el: &Element) -> Option<Value> {
        match el.text().as_ref() {
            None => return Some(Value::Text("".to_string())),
            Some(txt) => return Some(Value::Text(txt.to_owned())),
        }
    }

    fn draw(&self, el: &Element, buf: &mut Buffer, _app: &mut Ecs) {
        draw_text(el, buf);
    }
}

pub struct TextBuilder {
    node: Element,
}

impl TextBuilder {
    pub fn text(&self, text: &str) -> &Self {
        self.node.set_text(text);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    pub fn nowrap(&self) -> &Self {
        self.node.add_prop("nowrap");
        self
    }

    // pub fn width(&self, width: u32) -> &Self {
    //     let current = self.node.size().unwrap_or((0, 1));
    //     self.node.set_size(width, current.1);
    //     self
    // }

    // pub fn size(&self, width: u32, height: u32) -> &Self {
    //     self.node.set_size(width, height);
    //     self
    // }
}

impl Padded for TextBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for TextBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Styled for TextBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

// impl ParentNode for Text {
//     fn add_child(&mut self, node: Element) {
//         panic!("Text nodes cannot have children!");
//     }
// }

pub fn draw_text(el: &Element, buf: &mut Buffer) {
    let node = el.borrow();
    if el.has_prop("hidden") {
        return;
    }
    let style = node.styles.as_ref().unwrap();
    if let Some(pos) = node.pos.as_ref() {
        let mut draw = draw::colored(buf);
        draw = draw.fg(style.fg()).bg(style.bg());

        let size = node.size.unwrap();
        draw = draw.width(size.0 as i32);
        draw = draw.height(size.1 as i32);

        match node.props.contains("nowrap") {
            true => {
                draw.print(pos.0, pos.1, el.text().as_ref().unwrap());
            }
            false => {
                draw.wrap(pos.0, pos.1, el.text().as_ref().unwrap());
            }
        };
    }
}

pub(super) fn calc_common_size(a: Option<(u32, u32)>, b: Option<(u32, u32)>) -> (u32, u32) {
    let ca = a.unwrap_or((0, 0));
    let cb = b.unwrap_or((0, 0));

    let ra = match (ca.0, cb.0) {
        (0, 0) => 0,
        (0, x) => x,
        (x, 0) => x,
        (x, y) => min(x, y),
    };

    let rb = match (ca.1, cb.1) {
        (0, 0) => 0,
        (0, x) => x,
        (x, 0) => x,
        (x, y) => min(x, y),
    };

    (ra, rb)
}

pub fn text_set_size(node: &Element, max_size: Option<(u32, u32)>) {
    let mut size = node.size().unwrap_or((0, 0));
    let max_size = max_size.unwrap_or((0, 0));

    if node.has_prop("nowrap") {
        if size.1 == 0 {
            size.1 = 1;
        }
    }

    log(format!(
        "text set size, text='{}', size={:?}, max_size={:?}",
        node.text().as_ref().unwrap(),
        size,
        max_size
    ));

    if size.0 == 0 {
        // wrap on line breaks
        let (width, mut height) = {
            let node_text = node.text();
            let txt = node_text.as_ref().unwrap();
            let lines = parse_colored_lines(txt);
            (
                lines.iter().fold(0, |out, line| max(out, line.char_len())) as u32,
                lines.len().max(1) as u32,
            )
        };

        if max_size.1 > 0 {
            height = height.min(max_size.1);
        }
        if size.1 > 0 {
            height = size.1;
        }

        if max_size.0 == 0 || width <= max_size.0 {
            node.set_size(width, height);
            log(format!(" - result size 1 ={:?}", node.size()));
            return;
        } else {
            size.0 = max_size.0;
        }
    }

    let (width, mut height) = {
        let node_text = node.text();
        let txt = node_text.as_ref().unwrap();
        let lines = wrap_colored(size.0 as usize, txt);
        (
            lines.iter().fold(0, |out, line| max(out, line.char_len())) as u32,
            lines.len().max(1) as u32,
        )
    };

    if max_size.1 > 0 {
        height = height.min(max_size.1);
    }
    if size.1 > 0 {
        height = size.1;
    }
    match node.size().unwrap_or((0, 0)) {
        (0, 0) => node.set_size(width, height),
        (0, x) => node.set_size(width, x),
        (x, 0) => node.set_size(x, height),
        _ => {}
    }

    log(format!(" - result size={:?}", node.size()));
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ui::{self, test::extract_line};
    use gw_app::RGBA;

    #[test]
    fn simple_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO").text("Hello World");
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(text.text().as_ref().unwrap(), "Hello World");

        assert_eq!(text.size().unwrap(), (11, 1));
    }

    #[test]
    fn draw_width() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO").text("Hello").width(10).bg("#F00".into());
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(text.text().as_ref().unwrap(), "Hello");
        assert_eq!(text.size().unwrap(), (10, 1));
        assert_eq!(text.style().bg(), RGBA::rgb(255, 0, 0));

        let mut buf = Buffer::new(20, 10);
        let mut ecs = Ecs::new();
        ui.draw(&mut buf, &mut ecs);

        assert_eq!(extract_line(&buf, 0, 0, 10), "Hello\0\0\0\0\0");
        assert_eq!(buf.get_back(0, 0).unwrap(), &RGBA::rgba(255, 0, 0, 255)); // first cell has bg color
        assert_eq!(buf.get_back(9, 0).unwrap(), &RGBA::rgba(255, 0, 0, 255)); // last cell has bg color
    }

    #[test]
    fn set_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO").text("").width(15);
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(text.text().as_ref().unwrap(), "");
        assert_eq!(text.size().unwrap(), (15, 1));

        text.set_text("Hello World");
        assert_eq!(text.text().as_ref().unwrap(), "Hello World");
        assert_eq!(text.size().unwrap(), (15, 1));
    }

    #[test]
    fn trunc_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO")
                    .text("This is a longer text that will be truncated.")
                    .size(15, 1);
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(
            text.text().as_ref().unwrap(),
            "This is a longer text that will be truncated."
        );
        assert_eq!(text.size().unwrap(), (15, 1));

        let mut buf = Buffer::new(80, 50);
        let mut ecs = Ecs::new();
        ui.root().draw(&mut buf, &mut ecs);
        assert_eq!(extract_line(&buf, 0, 0, 16), "This is a lon-\0\0"); // hyphen tries to go in middle of word (even if only 1 line tall)
    }

    #[test]
    fn sized_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO").text("Hello World").size(20, 2);
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(text.text().as_ref().unwrap(), "Hello World");
        assert_eq!(text.size().unwrap(), (20, 2));
    }

    #[test]
    fn wrapped_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO")
                    .text("This is a longer text that will wrap to multiple lines.")
                    .width(13);
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(
            text.text().as_ref().unwrap(),
            "This is a longer text that will wrap to multiple lines."
        );
        assert_eq!(text.size().unwrap(), (13, 5));

        let mut buf = Buffer::new(80, 50);
        let mut ecs = Ecs::new();
        ui.root().draw(&mut buf, &mut ecs);

        assert_eq!(extract_line(&buf, 0, 0, 15), "This is a\0\0\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 1, 15), "longer text\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 2, 15), "that will\0\0\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 3, 15), "wrap to mult-\0\0");
        assert_eq!(extract_line(&buf, 0, 4, 15), "iple lines.\0\0\0\0");
    }

    #[test]
    fn sized_wrapped_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO")
                    .text("This is a longer text that will wrap to multiple lines.")
                    .size(13, 3);
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(
            text.text().as_ref().unwrap(),
            "This is a longer text that will wrap to multiple lines."
        );
        assert_eq!(text.size().unwrap(), (13, 3));

        let mut buf = Buffer::new(80, 50);
        let mut ecs = Ecs::new();
        ui.root().draw(&mut buf, &mut ecs);

        assert_eq!(extract_line(&buf, 0, 0, 15), "This is a\0\0\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 1, 15), "longer text\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 2, 15), "that will\0\0\0\0\0\0");
        assert_eq!(extract_line(&buf, 0, 3, 5), "\0\0\0\0\0");
    }

    #[test]
    fn nowrap_text() {
        let ui = ui::page((80, 50), "DEFAULT", |body| {
            Text::new(body, |txt| {
                txt.id("HELLO")
                    .text("This is a longer text that will wrap to multiple lines.")
                    .width(13)
                    .nowrap();
            });
        });

        let text = ui.find_by_id("HELLO").unwrap();
        assert_eq!(
            text.text().as_ref().unwrap(),
            "This is a longer text that will wrap to multiple lines."
        );
        assert_eq!(text.size().unwrap(), (13, 1));

        let mut buf = Buffer::new(80, 50);
        let mut ecs = Ecs::new();
        ui.root().draw(&mut buf, &mut ecs);

        assert_eq!(extract_line(&buf, 0, 0, 15), "This is a lon\0\0");
        assert_eq!(extract_line(&buf, 0, 1, 5), "\0\0\0\0\0");
    }
}
