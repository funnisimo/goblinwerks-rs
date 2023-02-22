use super::*;
use conapp::draw;
use conapp::Buffer;
use conapp::TextAlign;

static FRAME: Frame = Frame {};

pub struct Frame {}

impl Frame {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut FrameBuilder) -> (),
    {
        let mut frame = FrameBuilder {
            node: Element::new(&FRAME),
        };
        parent.add_child(frame.node.clone());

        init(&mut frame);

        // adjust spacing of child elements
        adjust_child_spacing(&frame.node);

        let mut size = frame.node.size().unwrap_or((0, 0));
        let child_size = frame.node.children_size(); // includes spacing
        let margin = frame.node.margin();

        if size.0 == 0 {
            let title_width = match *frame.node.text() {
                None => 0,
                Some(ref text) => text.len() as u32 + 4,
            };
            size.0 = max(child_size.0 + margin[0] + margin[2] + 2, title_width);
        }

        if size.1 == 0 {
            size.1 = 2 + child_size.1 + margin[1] + margin[3];
        }

        frame.node.set_size(size.0, size.1);
        println!(
            "- frame: size={:?}, outer_size={:?}, inner_size={:?}, child_size={:?}, path={}",
            frame.node.size().unwrap(),
            frame.node.outer_size(),
            frame.node.inner_size(),
            child_size,
            element_path(&frame.node)
        );
    }
}

impl Tag for Frame {
    fn as_str(&self) -> &'static str {
        "frame"
    }

    fn to_inner_size(&self, el: &Element, size: (u32, u32)) -> (u32, u32) {
        let node = el.borrow_mut();
        let margin = &node.margin;
        let size = node.size.unwrap_or((0, 0));

        (
            size.0.saturating_sub(2 + margin[0] + margin[2]),
            size.1.saturating_sub(2 + margin[1] + margin[3]),
        )
    }

    fn layout_children(&self, el: &Element) -> () {
        layout_frame(el);
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_frame(el, buf);
    }
}

//////////////////////////////////////////////

pub struct FrameBuilder {
    node: Element,
}

impl FrameBuilder {
    pub fn title(&self, title: &str) -> &Self {
        self.node.set_text(title);
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn align(&self, align: Align) -> &Self {
        self.node.set_align(align);
        self
    }

    pub fn valign(&self, valign: Align) -> &Self {
        self.node.set_valign(valign);
        self
    }

    pub fn spacing(&self, lines: u32) -> &Self {
        self.node.set_attr("spacing", (lines as i32).into());
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

    pub fn width(&self, width: u32) -> &Self {
        let current = self.node.size().unwrap_or((0, 0));
        self.node.set_size(width + 2, current.1); // add 2 for border
        self
    }
}

impl ParentNode for FrameBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Padded for FrameBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Margined for FrameBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

pub(super) fn layout_frame(el: &Element) {
    let mut pos = el.pos().unwrap();
    let mut size = el.size().unwrap();
    let margin = el.margin();

    let space: i32 = match el.attr("spacing") {
        None => 0,
        Some(spacing) => spacing.try_into().unwrap(),
    };

    // adjust for frame
    pos.0 += 1 + margin[0] as i32;
    pos.1 += 1 + margin[1] as i32;
    size.0 -= 2 + margin[0] + margin[2];
    size.1 -= 2 + margin[1] + margin[3];

    println!(
        "layout frame - inner_pos={:?}, inner_size={:?}, space={}, path={}",
        pos,
        size,
        space,
        element_path(el)
    );

    let all_size = el.children_size();
    let data = el.borrow();

    let mut y = pos.1
        + match data.valign.as_ref().unwrap_or(&Align::Min) {
            Align::Min => 0,
            Align::Center => (size.1 - all_size.1) / 2,
            Align::Max => size.1 - all_size.1,
        } as i32;

    println!(" - starting y = {}, all_size={:?}", y, all_size);

    for child in data.children.iter() {
        let child_size = child.outer_size();
        let child_anchor = match child.anchor() {
            None => data.align.unwrap_or(Align::Min),
            Some(x) => x,
        };
        let pos = (
            match child_anchor {
                Align::Min => 0,
                Align::Center => (size.0 - child_size.0) / 2,
                Align::Max => size.0 - child_size.0,
            } as i32
                + pos.0,
            y,
        );
        println!(
            " - child : size={:?}, child_size={:?}, pos={:?}, anchor={:?}, path={}",
            size,
            child_size,
            pos,
            child_anchor,
            element_path(child)
        );
        child.set_outer_pos(pos.0, pos.1);
        y += child_size.1 as i32;
        // y += space;
    }
}

pub(super) fn draw_frame(el: &Element, buf: &mut Buffer) {
    let style = el.style();
    if let Some(pos) = el.pos() {
        let size = el.size().unwrap();
        let mut frame = draw::Frame::new(buf)
            .border(style.border().unwrap_or(draw::BorderType::Single))
            .bg(style.border_bg())
            .fg(style.border_fg())
            .fill(Some(0), None, Some(style.bg()));
        if let Some(ref title) = *el.text() {
            frame = frame
                .title(&title)
                .title_align(TextAlign::Center)
                .title_fg(style.border_fg());
        }

        frame.draw(pos.0, pos.1, size.0, size.1);

        for child in el.borrow().children.iter() {
            child.draw(buf);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn frame_width() {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Test")
                    .pos(5, 25)
                    .width(28)
                    .id("FRAME");

                Text::new(frame, |text| {
                    text.id("TEXT").text("This is text.");
                });
            });
        });

        let body = ui.root();
        let frame = ui.find_by_id("FRAME").unwrap();
        let text = ui.find_by_id("TEXT").unwrap();

        assert_eq!(body.size().unwrap(), (80, 50));
        assert_eq!(frame.size().unwrap(), (30, 5)); // 28 is content width + 2 for border, 2 for border + 2 for margin + 1 for children
        assert_eq!(frame.inner_size().unwrap(), (26, 1));
        assert_eq!(text.size().unwrap(), (13, 1)); // 28 is width - 2 for margin, 1 line
    }

    #[test]
    fn frame_height() {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Test")
                    .pos(5, 25)
                    .id("FRAME")
                    .spacing(1);

                Text::new(frame, |text| {
                    text.id("A").text("This is text.");
                });

                Text::new(frame, |text| {
                    text.id("B").text("This is text.");
                });
            });
        });

        let body = ui.root();
        let frame = ui.find_by_id("FRAME").unwrap();

        assert_eq!(body.size().unwrap(), (80, 50));
        assert_eq!(frame.size().unwrap(), (17, 7)); // 13 is content width + 2 for border + 2 for margin, 2 for border + 2 for margin + 3 for children
        assert_eq!(frame.inner_size().unwrap(), (13, 3));
    }
}
