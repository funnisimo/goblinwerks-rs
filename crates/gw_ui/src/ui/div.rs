use super::*;

static DIV: Div = Div {};

/// CheckGroup is a widget that will show options and allow you to select them.
/// You configure the checkboxes individually.
/// The value of this widget is:
/// None - if none of the child checkboxes are checked
/// Map<Id,bool> - a map of child checkbox id to child checkbox value for the checked checkboxes.
/// Map<Id,Count> - a map of child checkbox id to count if
pub struct Div {}

impl Div {
    /// Creates a new select and calls the provided initialization function with a `DivBuilder` you can use to customize the CheckGroup.
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: FnOnce(&mut DivBuilder) -> (),
    {
        let mut select = DivBuilder {
            el: Element::new(&DIV),
        };
        parent.add_child(select.el.clone());

        init(&mut select);

        let child_size = select.el.children_size();
        let margin = select.el.margin();
        let size = (
            child_size.0 + margin[0] + margin[2],
            child_size.1 + margin[1] + margin[3],
        );
        select.el.set_size(size.0, size.1);
    }
}

impl Tag for Div {
    fn as_str(&self) -> &'static str {
        "div"
    }
}

////////////////////////////////////////

pub struct DivBuilder {
    el: Element,
}

impl DivBuilder {
    pub fn class(&mut self, class: &str) -> &mut Self {
        self.el.add_class(class);
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.el.set_id(id);
        self
    }

    pub fn align(&mut self, align: Align) -> &mut Self {
        self.el.set_align(align);
        self
    }

    pub fn valign(&mut self, valign: Align) -> &mut Self {
        self.el.set_valign(valign);
        self
    }
}

impl ParentNode for DivBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Positioned for DivBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Padded for DivBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}

impl Margined for DivBuilder {
    fn el(&self) -> &Element {
        &self.el
    }
}
