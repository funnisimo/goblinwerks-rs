use super::*;
use gw_app::{log, Ecs};
use gw_app::{Buffer, KeyEvent, Value};
use gw_util::point::Point;

pub trait Tag {
    fn as_str(&self) -> &'static str;

    fn layout_children(&self, el: &Element) -> () {
        layout_children(el)
    }

    fn children_size(&self, el: &Element) -> (u32, u32) {
        // let space: u32 = match el.attr("spacing") {
        //     None => 0,
        //     Some(data) => data.try_into().unwrap_or(0),
        // };

        log(format!("children_size: {}", element_path(el)));
        // let space = 0;

        let size = el.borrow().children.iter().fold((0, 0), |out, n| {
            let size = n.outer_size();
            log(format!("-- {:?}", size));
            (max(out.0, size.0), size.1 + out.1 /* + space */)
        });
        // size.1 = size.1.saturating_sub(space); // don't put space after last item

        log(format!("- {:?}", size));
        size
    }

    fn to_inner_size(&self, el: &Element, size: (u32, u32)) -> (u32, u32) {
        let margin = el.margin();
        (
            size.0.saturating_sub(margin[0] + margin[2]),
            size.1.saturating_sub(margin[1] + margin[3]),
        )
    }

    fn value(&self, el: &Element) -> Option<Value> {
        if let Some(val) = &el.borrow().value {
            return Some(val.clone());
        }

        None
    }

    fn can_focus(&self, el: &Element) -> bool {
        false
    }

    fn handle_click(&self, root: &Element, el: &Element, point: Point) -> Option<UiAction> {
        for child in el.node.borrow().children.iter() {
            if let Some(action) = child.handle_click(root, point) {
                return Some(action);
            }
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

        None
    }

    fn handle_key(&self, root: &Element, el: &Element, key: &KeyEvent) -> Option<UiAction> {
        if let Some(action) = el.node.borrow().keys.get(key) {
            return action(root, el);
        }

        if let Some(parent) = el.node.borrow().parent_element() {
            return parent.handle_key(root, key);
        }
        None
    }

    fn draw(&self, el: &Element, buf: &mut Buffer, ecs: &mut Ecs) {
        for child in el.borrow().children.iter() {
            child.draw(buf, ecs);
        }
    }
}
