use super::*;
use conapp::{Buffer, VirtualKeyCode};

static BODY: Body = Body {};

pub struct Body {}

impl Body {
    pub(super) fn new<F>(page_size: (u32, u32), init: F) -> Element
    where
        F: FnOnce(&mut BodyBuilder) -> (),
    {
        let body_el = Element::new(&BODY);
        body_el.bind_key(VirtualKeyCode::Tab, UiAction::focus_next());
        body_el.bind_key((VirtualKeyCode::Tab, true), UiAction::focus_prev());
        body_el.set_size(page_size.0, page_size.1);
        // body_el.set_outer_pos(0, 0);
        body_el.set_id("body");

        let mut body = BodyBuilder {
            node: body_el.clone(),
        };
        init(&mut body);

        body_el.layout_children();
        body_el
    }
}

impl Tag for Body {
    fn as_str(&self) -> &'static str {
        "body"
    }

    fn layout_children(&self, el: &Element) -> () {
        body_layout_children(el);
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_body(el, buf);
    }
}

///////////////////////////////////

pub struct BodyBuilder {
    node: Element,
}

impl BodyBuilder {
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.node.set_align(align);
        self
    }

    pub fn valign(&mut self, valign: Align) -> &mut Self {
        self.node.set_valign(valign);
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self
    }
}

impl ParentNode for BodyBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Margined for BodyBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Keyed for BodyBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

pub(super) fn body_layout_children(body: &Element) {
    let data = body.borrow();
    let all_size = data.children.iter().fold((0, 0), |out, el| {
        let size = el.outer_size();
        match el.pos() {
            None => (max(out.0, size.0), size.1 + out.1),
            Some(pos) => out,
        }
    });
    let size = data.size.unwrap_or(all_size.clone());
    let body_pos = data.pos.clone().unwrap_or((0, 0));

    println!(
        "{} layout children - pos={:?}, align={:?}, valign={:?}, size={},{}, all_size={},{}",
        body.tag(),
        body_pos,
        data.align,
        data.valign,
        size.0,
        size.1,
        all_size.0,
        all_size.1
    );

    let mut y = body_pos.1
        + match data.valign.as_ref().unwrap_or(&Align::Min) {
            Align::Min => 0,
            Align::Center => size.1.saturating_sub(all_size.1) / 2,
            Align::Max => size.1.saturating_sub(all_size.1),
        } as i32;

    println!(" - starting y = {}", y);
    for child in data.children.iter() {
        if child.pos().is_none() {
            let child_size = child.outer_size();
            let pos = (
                match data.align.as_ref().unwrap_or(&Align::Min) {
                    Align::Min => 0,
                    Align::Center => (size.0 - child_size.0) / 2,
                    Align::Max => size.0 - child_size.0,
                } as i32
                    + body_pos.0,
                y,
            );
            println!(" - child :  pos={:?}, full_size={:?}", pos, child_size);
            child.set_outer_pos(pos.0, pos.1); // calls layout_children
            y += child_size.1 as i32;
        } else {
            child.layout_children();
        }
    }
}

pub fn draw_body(el: &Element, buf: &mut Buffer) {
    let bg = el.style().bg();
    buf.fill(None, None, Some(bg));

    for child in el.borrow().children.iter() {
        child.draw(buf);
    }
}
