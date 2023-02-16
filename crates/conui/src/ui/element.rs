use super::*;
use conapp::Point;
use conapp::{Buffer, KeyEvent, MsgData};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};
use std::sync::Arc;

// pub type Callback = dyn FnMut(&mut UI) -> ();

pub struct ElementData {
    pub(super) id: Option<String>,    // Option?
    pub(super) tag: &'static dyn Tag, // Option?
    pub(super) parent: Option<Weak<RefCell<ElementData>>>,
    pub(super) pos: Option<(i32, i32)>,
    pub(super) size: Option<(u32, u32)>,
    pub(super) align: Option<Align>,
    pub(super) valign: Option<Align>,
    pub(super) anchor: Option<Align>,
    pub(super) vanchor: Option<Align>,
    pub(super) local_style: Option<Arc<Style>>,
    pub(super) classes: HashSet<String>,
    pub(super) props: HashSet<String>,
    pub(super) attrs: HashMap<String, MsgData>,
    pub(super) pad: [u32; 4],
    pub(super) margin: [u32; 4],
    pub(super) text: Option<String>,
    pub(super) styles: Option<Rc<ComputedStyle>>,
    pub(super) click: bool,
    pub(super) value: Option<MsgData>,
    pub(super) keys: HashMap<KeyEvent, Box<UiActionFn>>,
    pub(super) activate: Option<Box<UiActionFn>>,

    pub(crate) children: Vec<Element>,
}

impl ElementData {
    pub fn new(tag: &'static dyn Tag) -> Self {
        ElementData {
            id: None,
            tag,
            parent: None,
            pos: None,
            size: None,
            local_style: None,
            classes: HashSet::new(),
            props: HashSet::new(),
            attrs: HashMap::new(),
            text: None,
            align: None,
            valign: None,
            anchor: None,
            vanchor: None,
            pad: [0, 0, 0, 0],
            margin: [0, 0, 0, 0],
            // click: None,
            styles: None,
            click: false,
            value: None,
            keys: HashMap::new(),
            activate: None,
            children: Vec::new(),
        }
    }

    pub fn parent_element(&self) -> Option<Element> {
        match self.parent.as_ref() {
            None => None,
            Some(weak) => match weak.upgrade() {
                None => None,
                Some(rc) => Some(Element::from(rc.clone())),
            },
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Element");
        d.field("tag", &self.tag.as_str());
        if self.id.is_some() {
            d.field("id", &self.id.as_ref().unwrap());
        }
        if self.parent.is_some() {
            let parent = self.parent_element().unwrap();
            if parent.has_any_id() {
                d.field("parent", &format!("#{}", parent.id().as_ref().unwrap()));
            } else {
                d.field("parent", &parent.tag());
            }
        }
        if self.pos.is_some() {
            d.field("pos", self.pos.as_ref().unwrap());
        }
        if self.size.is_some() {
            d.field("size", self.size.as_ref().unwrap());
        }
        if !self.classes.is_empty() {
            d.field("class", &self.classes);
        }
        if !self.props.is_empty() {
            d.field("props", &self.props);
        }

        if !self.attrs.is_empty() {
            d.field("attrs", &self.attrs);
        }

        if self.align.is_some() {
            d.field("align", self.align.as_ref().unwrap());
        }
        if self.valign.is_some() {
            d.field("valign", self.valign.as_ref().unwrap());
        }

        if self.anchor.is_some() {
            d.field("anchor", self.anchor.as_ref().unwrap());
        }
        if self.vanchor.is_some() {
            d.field("vanchor", self.vanchor.as_ref().unwrap());
        }

        if self.pad.iter().any(|v| *v > 0) {
            d.field("pad", &self.pad);
        }
        if self.margin.iter().any(|v| *v > 0) {
            d.field("margin", &self.margin);
        }
        if self.click {
            d.field("click", &true);
        }

        if let Some(ref text) = self.text {
            d.field("text", text);
        }

        if !self.keys.is_empty() {
            d.field("keys", &self.keys.keys());
        }

        if !self.children.is_empty() {
            let kids: Vec<String> = self
                .children
                .iter()
                .map(|e| match *e.id() {
                    None => e.tag().to_string(),
                    Some(ref id) => format!("#{}", id),
                })
                .collect();
            d.field("children", &kids);
        }

        if let Some(ref styles) = self.styles {
            d.field("styles", &styles.len());
            // d.field("all_styles", &styles);
        }

        if let Some(ref value) = self.value {
            d.field("value", &value);
            // d.field("all_styles", &styles);
        }

        d.finish()
    }
}

#[derive(Clone)]
pub struct Element {
    pub(super) node: Rc<RefCell<ElementData>>,
}

impl Element {
    // pub fn from(node: Rc<RefCell<ElementData>>) -> Self {
    //     Element { node }
    // }

    pub fn new(tag: &'static dyn Tag) -> Self {
        Element {
            node: Rc::new(RefCell::new(ElementData::new(tag))),
        }
    }

    pub fn is_root(&self) -> bool {
        self.node.borrow().parent.is_none()
    }

    pub fn borrow(&self) -> Ref<ElementData> {
        self.node.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<ElementData> {
        self.node.borrow_mut()
    }

    pub fn is(&self, other: &Element) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }

    pub fn can_focus(&self) -> bool {
        let tag = self.node.borrow().tag.clone();
        tag.can_focus(self)
    }

    pub fn id(&self) -> Ref<Option<String>> {
        let b = self.node.borrow();
        Ref::map(b, |d| &d.id)
    }
    pub fn has_any_id(&self) -> bool {
        self.node.borrow().id.is_some()
    }
    pub fn has_id(&self, id: &str) -> bool {
        match self.node.borrow().id.as_ref() {
            None => false,
            Some(val) => val == id,
        }
    }
    pub(super) fn set_id(&self, id: &str) {
        self.node.borrow_mut().id = Some(id.to_string());
    }

    pub fn text(&self) -> Ref<Option<String>> {
        let b = self.node.borrow();
        Ref::map(b, |d| &d.text)
    }
    pub fn has_any_text(&self) -> bool {
        self.borrow().text.is_some()
    }
    pub fn set_text(&self, text: &str) {
        self.node.borrow_mut().text = Some(text.to_string());
    }

    pub fn value(&self) -> Option<MsgData> {
        let tag = self.node.borrow().tag;
        tag.value(self)
    }
    pub fn set_value(&self, val: Option<MsgData>) {
        self.node.borrow_mut().value = val;
    }

    pub fn tag(&self) -> &'static str {
        self.node.borrow().tag.as_str()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.node.borrow().tag.as_str() == tag
    }

    pub fn size(&self) -> Option<(u32, u32)> {
        self.node.borrow().size.clone()
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.node.borrow_mut().size = Some((width, height));
    }

    pub fn outer_size(&self) -> (u32, u32) {
        let node = self.node.borrow();
        let size = node.size.unwrap_or((0, 0));
        let pad = &node.pad;

        (size.0 + pad[0] + pad[2], size.1 + pad[1] + pad[3])
    }

    pub fn inner_size(&self) -> Option<(u32, u32)> {
        let tag = self.borrow().tag;
        match self.size() {
            None => None,
            Some(size) => Some(tag.to_inner_size(self, size)),
        }
    }

    pub fn to_inner_size(&self, size: Option<(u32, u32)>) -> Option<(u32, u32)> {
        let tag = self.borrow().tag;
        match size {
            None => None,
            Some(size) => Some(tag.to_inner_size(self, size)),
        }
    }

    pub fn children_size(&self) -> (u32, u32) {
        let tag = self.borrow().tag;
        tag.children_size(self)
    }

    pub fn pos(&self) -> Option<(i32, i32)> {
        self.node.borrow().pos.clone()
    }

    pub fn set_pos(&self, x: i32, y: i32) -> &Self {
        self.node.borrow_mut().pos = Some((x, y));
        self
    }

    pub fn inner_pos(&self) -> Option<(i32, i32)> {
        let node = self.node.borrow();
        let margin = node.margin;
        match self.node.borrow().pos {
            None => None,
            Some((x, y)) => Some((x + margin[0] as i32, y + margin[1] as i32)),
        }
    }

    pub(super) fn set_outer_pos(&self, x: i32, y: i32) {
        let (x1, y1) = {
            let node = self.node.borrow();
            let pad = &node.pad;
            (x + pad[0] as i32, y + pad[1] as i32)
        };
        self.node.borrow_mut().pos = Some((x1, y1));
        let tag = self.node.borrow().tag;
        tag.layout_children(self);
    }

    pub fn align(&self) -> Option<Align> {
        self.node.borrow().align
    }

    pub(super) fn set_align(&self, val: Align) {
        self.node.borrow_mut().align = Some(val);
    }

    pub fn valign(&self) -> Option<Align> {
        self.node.borrow().valign
    }

    pub(super) fn set_valign(&self, val: Align) {
        self.node.borrow_mut().valign = Some(val);
    }

    pub fn anchor(&self) -> Option<Align> {
        self.node.borrow().anchor
    }

    pub(super) fn set_anchor(&self, val: Align) {
        self.node.borrow_mut().anchor = Some(val);
    }

    pub fn vanchor(&self) -> Option<Align> {
        self.node.borrow().vanchor
    }

    pub(super) fn set_vanchor(&self, val: Align) {
        self.node.borrow_mut().vanchor = Some(val);
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.node.borrow().classes.contains(class)
    }

    pub fn add_class(&self, class: &str) {
        if class.len() > 0 {
            self.node.borrow_mut().classes.insert(class.to_string());
        }
    }

    pub fn has_prop(&self, prop: &str) -> bool {
        self.node.borrow().props.contains(prop)
    }

    pub fn add_prop(&self, prop: &str) {
        if prop.len() > 0 {
            self.node.borrow_mut().props.insert(prop.to_string());
        }
    }

    pub fn remove_prop(&self, prop: &str) {
        if prop.len() > 0 {
            self.node.borrow_mut().props.remove(prop);
        }
    }

    pub fn toggle_prop(&self, prop: &str) {
        if prop.len() == 0 {
            return;
        }
        let mut node = self.node.borrow_mut();
        if node.props.contains(prop) {
            node.props.remove(prop);
        } else {
            node.props.insert(prop.to_string());
        }
    }

    pub fn attr(&self, attr: &str) -> Option<MsgData> {
        match self.node.borrow().attrs.get(attr) {
            None => None,
            Some(d) => Some(d.clone()),
        }
    }

    pub fn set_attr(&self, attr: &str, val: MsgData) {
        if attr.len() > 0 {
            self.node.borrow_mut().attrs.insert(attr.to_owned(), val);
        }
    }

    pub fn has_attr(&self, attr: &str) -> bool {
        if attr.len() == 0 {
            return false;
        }
        self.node.borrow().attrs.contains_key(attr)
    }

    pub fn pad(&self) -> [u32; 4] {
        self.node.borrow().pad
    }
    pub fn set_pad(&self, pad: u32) {
        self.node.borrow_mut().pad = [pad, pad, pad, pad];
    }

    pub fn set_pad_left(&self, pad: u32) {
        self.node.borrow_mut().pad[0] = pad;
    }
    pub fn set_pad_top(&self, pad: u32) {
        self.node.borrow_mut().pad[1] = pad;
    }
    pub fn set_pad_right(&self, pad: u32) {
        self.node.borrow_mut().pad[2] = pad;
    }
    pub fn set_pad_bottom(&self, pad: u32) {
        self.node.borrow_mut().pad[3] = pad;
    }

    pub fn margin(&self) -> [u32; 4] {
        self.node.borrow().margin
    }
    pub fn set_margin(&self, margin: u32) {
        self.node.borrow_mut().margin = [margin, margin, margin, margin];
    }

    pub fn set_margin_left(&self, margin: u32) {
        let mut el = self.node.borrow_mut();
        let current = &mut el.margin;
        let diff = margin - current[0];
        current[0] = margin;
        if let Some(size) = &mut el.size {
            size.0 += diff;
        }
    }
    pub fn set_margin_top(&self, margin: u32) {
        let mut el = self.node.borrow_mut();
        let current = &mut el.margin;
        let diff = margin - current[1];
        current[0] = margin;
        if let Some(size) = &mut el.size {
            size.1 += diff;
        }
    }
    pub fn set_margin_right(&self, margin: u32) {
        let mut el = self.node.borrow_mut();
        let current = &mut el.margin;
        let diff = margin - current[2];
        current[0] = margin;
        if let Some(size) = &mut el.size {
            size.0 += diff;
        }
    }
    pub fn set_margin_bottom(&self, margin: u32) {
        let mut el = self.node.borrow_mut();
        let current = &mut el.margin;
        let diff = margin - current[3];
        current[0] = margin;
        if let Some(size) = &mut el.size {
            size.1 += diff;
        }
    }

    pub fn parent(&self) -> Option<Element> {
        self.node.borrow().parent_element()
    }

    pub fn children(&self) -> impl Iterator<Item = Element> {
        self.node.borrow().children.clone().into_iter()
    }

    pub fn contains(&self, point: Point) -> bool {
        match self.pos() {
            None => return false,
            Some(pos) => {
                if pos.0 > point.x || pos.1 > point.y {
                    return false;
                }
                match self.size() {
                    None => return false,
                    Some(size) => {
                        let right = pos.0 + size.0 as i32;
                        let bottom = pos.1 + size.1 as i32;
                        if right <= point.x || bottom <= point.y {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn every_element<F>(&self, func: &mut F)
    where
        F: FnMut(&Element) -> (),
    {
        func(self);
        for child in self.borrow().children.iter() {
            child.every_element(func);
        }
    }

    pub fn each_child<F>(&self, func: &mut F)
    where
        F: FnMut(&Element) -> (),
    {
        for child in self.borrow().children.iter() {
            func(child);
        }
    }

    pub fn find_child<F>(&self, func: &mut F) -> Option<Element>
    where
        F: FnMut(&Element) -> bool,
    {
        for child in self.borrow().children.iter() {
            if func(child) {
                return Some(child.clone());
            }
        }
        None
    }

    pub(super) fn update_hover(&self, point: Point) -> Option<Element> {
        let mut res: Option<Element> = None;
        for child in self.borrow().children.iter() {
            if let Some(el) = child.update_hover(point) {
                res = Some(el);
            }
        }
        if self.contains(point) {
            self.add_prop("hover");
            if res.is_none() && self.can_focus() {
                res = Some(self.clone());
            }
        } else {
            self.remove_prop("hover");
        }
        res
    }

    pub(super) fn activate(&self) -> Option<Ref<Box<UiActionFn>>> {
        let b = self.node.borrow();
        match b.activate {
            None => None,
            Some(_) => Some(Ref::map(b, |d| d.activate.as_ref().unwrap())),
        }
    }

    pub(super) fn set_activate(&self, func: Box<UiActionFn>) {
        self.node.borrow_mut().activate = Some(func);
    }

    pub(super) fn handle_click(&self, root: &Element, point: Point) -> Option<UiAction> {
        let tag = self.node.borrow().tag.clone();
        tag.handle_click(root, self, point)
    }

    pub(super) fn handle_activate(&self, root: &Element) -> Option<UiAction> {
        let tag = self.node.borrow().tag.clone();
        tag.handle_activate(root, self)
    }

    pub(super) fn bind_key<K: Into<KeyEvent>>(&self, key: K, action: Box<UiActionFn>) {
        self.node.borrow_mut().keys.insert(key.into(), action);
    }

    pub(super) fn handle_key(&self, root: &Element, key: &KeyEvent) -> Option<UiAction> {
        let tag = self.node.borrow().tag.clone();
        tag.handle_key(root, self, key)
    }

    pub(super) fn setup_style(&self, styles: &StyleSheet) {
        let computed = styles.get_computed_style(self);
        self.borrow_mut().styles = Some(Rc::new(computed));

        for child in self.borrow().children.iter() {
            child.setup_style(styles);
        }
    }

    pub fn style(&self) -> Rc<ComputedStyle> {
        self.borrow().styles.as_ref().unwrap().clone()
    }

    pub(super) fn layout_children(&self) {
        let tag = self.node.borrow().tag;
        tag.layout_children(self);
    }

    pub fn add_child(&self, child: Element) {
        child.node.borrow_mut().parent = Some(Rc::downgrade(&self.node));
        self.node.borrow_mut().children.push(child);
    }

    pub fn child_count(&self) -> usize {
        self.node.borrow().children.len()
    }

    pub fn first_child(&self) -> Option<Element> {
        match self.node.borrow().children.iter().next() {
            None => None,
            Some(el) => Some(el.clone()),
        }
    }

    pub fn last_child(&self) -> Option<Element> {
        match self.node.borrow().children.iter().last() {
            None => None,
            Some(el) => Some(el.clone()),
        }
    }

    pub fn child_position(&self, child: &Element) -> Option<usize> {
        self.node.borrow().children.iter().position(|e| e.is(child))
    }

    pub fn leaf_matching(&self, cmp: impl Fn(&Element) -> bool + Copy) -> Option<Element> {
        if self.borrow().children.is_empty() {
            if cmp(self) {
                return Some(self.clone());
            }
        } else {
            for child in self.node.borrow().children.iter() {
                if let Some(node) = child.leaf_matching(cmp) {
                    return Some(node);
                }
            }
        }
        None
    }

    pub fn leaf_at(&self, point: Point) -> Option<Element> {
        self.leaf_matching(|e| e.contains(point))
    }

    pub fn get_child_by_index(&self, index: usize) -> Option<Element> {
        match self.node.borrow().children.get(index) {
            None => None,
            Some(el) => Some(el.clone()),
        }
    }

    pub fn get_child_by_id(&self, id: &str) -> Option<Element> {
        for child in self.node.borrow().children.iter() {
            if let Some(child_id) = child.id().as_ref() {
                if id == child_id {
                    return Some(child.clone());
                }
            }
        }
        None
    }

    pub fn find_by_id(&self, id: &str) -> Option<Element> {
        for child in self.node.borrow().children.iter() {
            if let Some(child_id) = child.id().as_ref() {
                if id == child_id {
                    return Some(child.clone());
                }
            }
            if let Some(node) = child.find_by_id(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn get_child_by_tag(&self, tag: &str) -> Option<Element> {
        for child in self.node.borrow().children.iter() {
            if tag == child.tag() {
                return Some(child.clone());
            }
        }
        None
    }

    pub fn find_by_tag(&self, tag: &str) -> Option<Element> {
        for child in self.node.borrow().children.iter() {
            if tag == child.tag() {
                return Some(child.clone());
            }
            if let Some(node) = child.find_by_tag(tag) {
                return Some(node);
            }
        }
        None
    }

    pub fn draw(&self, buf: &mut Buffer) {
        let tag = self.node.borrow().tag;
        tag.draw(self, buf);
    }

    pub fn dump(&self) {
        println!("{:?}", self);

        for child in self.borrow().children.iter() {
            child.dump();
        }
    }
}

impl From<Rc<RefCell<ElementData>>> for Element {
    fn from(node: Rc<RefCell<ElementData>>) -> Self {
        Element { node }
    }
}

impl From<&Rc<RefCell<ElementData>>> for Element {
    fn from(node: &Rc<RefCell<ElementData>>) -> Self {
        Element { node: node.clone() }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node.borrow().fmt(f)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node.borrow();
        let tag = node.tag.as_str();
        match node.id {
            None => write!(f, "<{}>", tag),
            Some(ref id) => write!(f, "<{}#{}>", tag, id),
        }
    }
}

pub(super) fn layout_children(el: &Element) {
    let mut parent_pos = el.pos().unwrap();
    // let size = el.size().unwrap();

    let margin = el.margin();
    parent_pos.0 += margin[0] as i32;
    parent_pos.1 += margin[1] as i32;

    for child in el.borrow().children.iter() {
        child.set_outer_pos(parent_pos.0, parent_pos.1);
        let (_, child_height) = child.outer_size();
        parent_pos.1 += child_height as i32;
        let tag = child.node.borrow().tag;
        tag.layout_children(&child);
    }
}

pub fn element_path(el: &Element) -> String {
    if let Some(ref id) = *el.id() {
        return format!("#{}", id);
    }

    if let Some(parent) = el.parent() {
        let index = parent
            .children()
            .filter(|ch| ch.tag() == el.tag())
            .position(|ch| ch.is(el))
            .unwrap();
        return format!("{}.{}[{}]", element_path(&parent), el.tag(), index);
    } else {
        return el.tag().to_string();
    }
}

pub fn dump_element(el: &Element) {
    _dump_element(el, 0);
}

fn _dump_element(el: &Element, indent: usize) {
    let spaces = " ".repeat(indent);
    println!("{}{:?}", spaces, el);

    for child in el.children() {
        _dump_element(&child, indent + 2);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use conapp::Point;

    #[test]
    fn contains() {
        let node = Element::new(&TEXT);

        assert!(node.pos().is_none());
        assert!(node.size().is_none());
        assert_eq!(node.contains(Point::new(6, 6)), false);

        node.set_outer_pos(5, 5);
        assert_eq!(node.contains(Point::new(6, 6)), false);

        node.set_size(10, 1);
        assert_eq!(node.contains(Point::new(5, 4)), false);
        assert_eq!(node.contains(Point::new(4, 5)), false);
        assert_eq!(node.contains(Point::new(5, 5)), true);
        assert_eq!(node.contains(Point::new(14, 5)), true);
        assert_eq!(node.contains(Point::new(15, 5)), false);
        assert_eq!(node.contains(Point::new(5, 6)), false);
        assert_eq!(node.contains(Point::new(6, 6)), false);

        node.set_outer_pos(8, 3);
        assert_eq!(node.contains(Point::new(6, 5)), false);
    }
}
