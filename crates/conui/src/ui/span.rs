use super::*;

static SPAN: Span = Span {};

pub struct Span {}

impl Span {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&SpanBuilder) -> (),
    {
        let el = Element::new(&SPAN);
        el.set_attr("spacing", 1.into());
        let mut span = SpanBuilder { el: el.clone() };

        parent.add_child(el.clone());
        init(&mut span);

        if el.size().is_none() {
            let child_size = el.children_size();

            let margin = el.margin();
            let size = (
                child_size.0 + margin[0] + margin[2],
                child_size.1 + margin[1] + margin[3],
            );

            span.el.set_size(size.0, size.1);
            println!(
                "Sizing span... child_size={:?}, size={:?}",
                child_size, size
            );
        }
    }
}

impl Tag for Span {
    fn as_str(&self) -> &'static str {
        "span"
    }

    fn layout_children(&self, el: &Element) -> () {
        layout_span(el);
    }

    // layout is horizontal
    fn children_size(&self, el: &Element) -> (u32, u32) {
        let space: u32 = match el.attr("spacing") {
            None => 0,
            Some(data) => (data.try_into().unwrap_or(0_i32)) as u32,
        };

        let mut size = el.borrow().children.iter().fold((0, 0), |out, n| {
            let child_size = n.outer_size();
            (out.0 + child_size.0 + space, max(child_size.1, out.1))
        });
        size.0 = size.0.saturating_sub(space); // don't put space after last item
        size
    }
}

pub struct SpanBuilder {
    el: Element,
}

impl SpanBuilder {
    pub fn id(&self, id: &str) -> &Self {
        self.el.set_id(id);
        self
    }

    pub fn spacing(&self, val: u16) -> &Self {
        self.el.set_attr("spacing", (val as i32).into());
        self
    }

    // pub fn pos(&mut self, x: i32, y: i32) -> &mut Self {
    //     self.el.set_pos(x, y);
    //     self
    // }

    // pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
    //     self.el.set_size(width, height);
    //     self
    // }
}

impl ParentNode for SpanBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Padded for SpanBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Positioned for SpanBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

pub(super) fn layout_span(el: &Element) {
    let pos = el.inner_pos().unwrap();
    let size = el.outer_size();

    println!("layout span - inner_pos={:?}, outer_size={:?}", pos, size);
    // adjust for frame

    let spacing: i32 = el.attr("spacing").unwrap().try_into().unwrap();
    let child_size = el.children_size(); // includes spacing
    let data = el.borrow();

    let mut x = pos.0
        + match data.align.as_ref().unwrap_or(&Align::Min) {
            Align::Min => 0,
            Align::Center => (size.0 - child_size.0) / 2,
            Align::Max => size.0 - child_size.0,
        } as i32;

    println!(" - starting x = {}", x);
    for child in data.children.iter() {
        let child_size = child.outer_size();
        let pos = (
            x,
            match data.valign.as_ref().unwrap_or(&Align::Min) {
                Align::Min => 0,
                Align::Center => (size.1 - child_size.1) / 2,
                Align::Max => size.1 - child_size.1,
            } as i32
                + pos.1,
        );
        println!(" - child pos = {},{}", pos.0, pos.1);
        child.set_outer_pos(pos.0, pos.1);
        x += child_size.0 as i32 + spacing;
    }
}

// pub(super) fn render_span(el: &Element, ctx: &mut dyn AppContext) {
//     let style = el.style();
//     if let Some(pos) = el.pos() {
//         let size = el.size().unwrap();
//         let mut frame = draw::Span::new(style.border().unwrap_or(draw::BorderType::Single));
//         frame
//             .bg(style.bg())
//             .fg(style.fg())
//             .fill(style.fill())
//             .pos(pos.0, pos.1)
//             .size(size.0, size.1);
//         if let Some(title) = el.text() {
//             frame.title(title);
//         }

//         frame.draw(ctx.draw_target(UI_CONSOLE));

//         for child in el.borrow().children.iter() {
//             child.render(ctx);
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn layout_span() {
        let ui = dialog((80, 50), "DEFAULT", |dlg| {
            Span::new(dlg, |span| {
                Button::new(span, |btn| {
                    btn.text("Button").width(8).pad_left(1).pad_right(1);
                });

                Button::new(span, |btn| {
                    btn.text("Button").width(8).pad_left(1).pad_right(1);
                });
            });
        });

        assert_eq!(ui.root().size().unwrap(), (20, 1));
    }

    #[test]
    fn pad_span() {
        let ui = dialog((80, 50), "DEFAULT", |dlg| {
            Span::new(dlg, |span| {
                span.pad_top(1).pad_bottom(1);

                Button::new(span, |btn| {
                    btn.text("Button").width(8).pad_left(1).pad_right(1);
                });

                Button::new(span, |btn| {
                    btn.text("Button").width(8).pad_left(1).pad_right(1);
                });
            });
        });

        let span = ui.root().first_child().unwrap();
        assert_eq!(span.pad(), [0, 1, 0, 1]);
        assert_eq!(span.size().unwrap(), (20, 1));
        assert_eq!(span.outer_size(), (20, 3));
        assert_eq!(ui.root().size().unwrap(), (20, 3));
    }
}
