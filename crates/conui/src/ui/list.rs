use super::*;
use conapp::{log, text::colored_line_len, Buffer, MsgData};

static LIST: List = List {};

pub struct List {}

impl List {
    pub fn new<F>(parent: &dyn ParentNode, init: F) -> ()
    where
        F: Fn(&mut ListBuilder) -> (),
    {
        let el = Element::new(&LIST);
        el.set_attr("off_glyph", "-".into());
        el.set_attr("space", 1.into());
        let mut builder = ListBuilder::new(el.clone());
        parent.add_child(el.clone());

        log("NEW LIST");

        init(&mut builder);

        adjust_child_spacing(&el);

        for ch in el.children() {
            log(format!(
                " - {} - {:?} ? {:?}",
                element_path(&ch),
                ch.size().unwrap(),
                ch.outer_size()
            ));
        }

        // finish list
        let children_size = el.children_size();
        log(format!("children - size = {:?}", children_size));

        for ch in el.children() {
            log(format!(
                " - {} - {:?} ? {:?}",
                element_path(&ch),
                ch.size().unwrap(),
                ch.outer_size()
            ));
        }

        match el.size().unwrap_or((0, 0)) {
            (0, 0) => {
                el.set_size(children_size.0, children_size.1);
            }
            (0, x) => {
                el.set_size(children_size.0, x);
                let mut height = 0;
                for ch in el.borrow().children.iter() {
                    if height >= x {
                        ch.add_prop("hidden");
                    } else {
                        let size = ch.size().unwrap();
                        if height + size.1 > x {
                            ch.set_size(size.0, x - height);
                            height = x;
                        } else {
                            height += size.1;
                        }
                    }
                }
            }
            (x, 0) => {
                for ch in el.borrow().children.iter() {
                    let size = ch.size().unwrap();
                    ch.set_size(x, size.1);
                }
                let child_size = el.children_size();
                el.set_size(x, child_size.1);
            }
            (w, h) => {
                let mut height = 0;
                for ch in el.borrow().children.iter() {
                    let size = ch.outer_size();
                    if height >= h {
                        ch.add_prop("hidden");
                    } else {
                        if height + size.1 > h {
                            ch.set_size(w, h - height);
                            height = h;
                        } else {
                            height += size.1;
                            ch.set_size(w, size.1);
                        }
                    }
                }
            }
        }
    }
}

impl Tag for List {
    fn as_str(&self) -> &'static str {
        "list"
    }
}

pub struct ListBuilder {
    node: Element,
}

impl ListBuilder {
    fn new(node: Element) -> Self {
        ListBuilder { node }
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn item(&mut self, text: &str) -> &mut Self {
        let align = self.node.align();
        let glyph = self.node.attr("off_glyph").unwrap().to_string();
        let nowrap = self.node.has_prop("nowrap");
        let space: i32 = self.node.attr("space").unwrap().try_into().unwrap();
        ListItem::new(self, move |item| {
            if let Some(align) = align {
                item.align(align);
            }
            item.glyph(&glyph);
            item.space(space);
            if nowrap {
                item.nowrap();
            }
            item.text(text);
        });
        self
    }

    pub fn with_item<F>(&mut self, text: &str, init: F) -> &mut Self
    where
        F: Fn(&mut ListItemBuilder) -> (),
    {
        let align = self.node.align();
        let glyph = self.node.attr("off_glyph");
        let space: i32 = self.node.attr("space").unwrap().try_into().unwrap();
        // let nowrap = self.node.has_prop("nowrap");
        ListItem::new(self, move |item| {
            if let Some(align) = align {
                item.align(align);
            }
            if let Some(glyph) = glyph.as_ref() {
                item.glyph(&glyph.to_string());
            }
            item.text(text);
            item.space(space);
            // if nowrap {
            //     item.nowrap();
            // }
            init(item);
        });
        self
    }

    pub fn align(&self, align: Align) -> &Self {
        self.node.set_align(align); // Need to give to items
        self
    }

    pub fn glyph(&self, glyph: &str) -> &Self {
        self.node.set_attr("off_glyph", glyph.into()); // Need to give to items
        self
    }

    pub fn indent(&self, indent: u32) -> &Self {
        self.node.set_margin_left(indent);
        self
    }

    // SPACING - space between list items
    pub fn spacing(&self, lines: u32) -> &Self {
        self.node.set_attr("spacing", (lines as i32).into());
        self
    }

    // SPACE - space between glyph and label
    pub fn space(&self, chars: u32) -> &Self {
        self.node.set_attr("space", (chars as i32).into());
        self
    }

    // pub fn nowrap(&self) -> &Self {
    //     self.node.add_prop("nowrap"); // Need to give to items
    //     self
    // }
}

impl Padded for ListBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Margined for ListBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl Positioned for ListBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl ParentNode for ListBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

//////////////////////////////////////////

static LIST_ITEM: ListItem = ListItem {};

pub struct ListItem {}

impl ListItem {
    fn new<F>(parent: &ListBuilder, init: F) -> ()
    where
        F: Fn(&mut ListItemBuilder) -> (),
    {
        let label = Element::new(&LABEL);
        label.set_text(""); // default text
        let node = Element::new(&LIST_ITEM);
        node.set_attr("off_glyph", "-".into());
        node.set_attr("space", 1.into());
        node.add_child(label.clone());
        parent.node.add_child(node.clone());

        let mut checkbox = ListItemBuilder {
            node: node.clone(),
            label: label.clone(),
        };

        log("NEW ITEM");

        init(&mut checkbox);

        let off_text: String = node.attr("off_glyph").unwrap().to_string();
        let space: i32 = node.attr("space").unwrap().try_into().unwrap();
        let prefix_size = colored_line_len(&off_text) as u32 + space as u32;
        let inner_size = calc_common_size(
            node.inner_size(),
            node.to_inner_size(inner_size_hint(&parent.node)),
        );

        log(format!(
            "Finish list Item({}) - inner_size={:?}, node.inner_size={:?}, inner_size_hint={:?}",
            element_path(&node),
            inner_size,
            node.inner_size(),
            inner_size_hint(&parent.node)
        ));
        text_set_size(&label, Some(inner_size));
        let child_size = node.children_size();
        node.set_size(child_size.0 + prefix_size, child_size.1);

        log(format!(
            " - list Item({}) - actual size={:?}",
            element_path(&node),
            node.size().unwrap()
        ));

        node.set_text(&off_text);
    }
}

impl Tag for ListItem {
    fn as_str(&self) -> &'static str {
        "item"
    }

    fn to_inner_size(&self, el: &Element, size: (u32, u32)) -> (u32, u32) {
        let off_text: String = el.attr("off_glyph").unwrap().to_string();
        let space: i32 = el.attr("space").unwrap().try_into().unwrap();
        let prefix_size = colored_line_len(&off_text) as u32 + space as u32;
        let margin = el.margin();

        (
            size.0.saturating_sub(margin[0] + margin[2] + prefix_size),
            size.1.saturating_sub(margin[1] + margin[3]),
        )
    }

    fn layout_children(&self, el: &Element) -> () {
        let mut child_pos = el.pos().unwrap();
        // let size = el.size().unwrap();

        let margin = el.margin();
        child_pos.0 += margin[0] as i32;
        child_pos.1 += margin[1] as i32;

        let text = el.text();
        let space: i32 = el.attr("space").unwrap().try_into().unwrap();
        let prefix_width = text.as_ref().unwrap().len() as i32 + space;
        child_pos.0 += prefix_width;

        log(format!("layout list item children - {}", element_path(el)));
        for child in el.borrow().children.iter() {
            log(format!("- {:?} @ {:?}", element_path(child), child_pos));
            child.set_outer_pos(child_pos.0, child_pos.1); // calls layout_children
            let (_, child_height) = child.outer_size();
            child_pos.1 += child_height as i32;
            // let tag = child.node.borrow().tag;
            // tag.layout_children(&child);
        }
    }

    fn value(&self, el: &Element) -> Option<MsgData> {
        None
    }

    fn draw(&self, el: &Element, buf: &mut Buffer) {
        draw_checkbox(el, buf);
    }
}

pub struct ListItemBuilder {
    node: Element,
    label: Element,
}

impl ListItemBuilder {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.label.set_text(text);
        self
    }

    pub fn id(&self, id: &str) -> &Self {
        self.node.set_id(id);
        self
    }

    pub fn class(&self, class: &str) -> &Self {
        self.node.add_class(class);
        self.label.add_class(class);
        self
    }

    pub fn width(&self, width: u32) -> &Self {
        let current = self.node.size().unwrap_or((0, 0));
        self.node.set_size(width, current.1);
        self
    }

    pub fn align(&self, align: Align) -> &Self {
        self.node.set_align(align);
        self
    }

    pub fn glyph(&self, glyph: &str) -> &Self {
        self.node.set_attr("off_glyph", glyph.into());
        self
    }

    pub fn space(&self, chars: i32) -> &Self {
        self.node.set_attr("space", chars.into());
        self
    }

    pub fn nowrap(&self) -> &Self {
        self.label.add_prop("nowrap");
        self
    }

    pub fn sublist<F>(&mut self, init: F) -> &mut Self
    where
        F: Fn(&mut ListBuilder) -> (),
    {
        List::new(self, init);
        self
    }
}

impl Padded for ListItemBuilder {
    fn el(&self) -> &Element {
        &self.node
    }
}

impl ParentNode for ListItemBuilder {
    fn el(&self) -> &Element {
        &self.node
    }

    // fn add_child(&mut self, node: Element) {
    //     let margin_left = self.node.margin()[0];
    //     let glyph_len = self.node.attr("off_glyph").unwrap().to_string().len() as u32;
    //     node.margin_left(margin_left + glyph_len);
    //     self.node.add_child(node);
    // }
}

#[cfg(test)]
mod test {
    use crate::ui::test::extract_line;

    use super::*;

    #[test]
    fn basic_list() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.item("Test A").with_item("Test B", |item| {});
            });
        });

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "list");
        assert_eq!(list.child_count(), 2);
    }

    #[test]
    fn list_margin() {
        let ui = page((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                list.margin_left(2);
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.item("Test A").with_item("Test B", |item| {});
            });
        });

        ui.dump();

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "list");
        assert_eq!(list.child_count(), 2);
        assert_eq!(list.margin(), [2, 0, 0, 0]);

        let mut buffer = Buffer::new(80, 50);
        ui.root().draw(&mut buffer);

        assert_eq!(extract_line(&buffer, 0, 0, 12), "\0\0-\0Test A\0\0");
        assert_eq!(extract_line(&buffer, 0, 1, 12), "\0\0-\0Test B\0\0");
    }

    #[test]
    fn basic_sublist() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.item("Test A").with_item("Test B", |item| {
                    List::new(item, |sublist| {
                        sublist.item("Apple").item("Banana").item("Carrot");
                    });
                });
            });
        });

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "list");
        assert_eq!(list.child_count(), 2); // 2 items

        let item_b = list.last_child().unwrap();
        assert_eq!(item_b.child_count(), 2); // label + list

        let sublist = item_b.last_child().unwrap();
        assert_eq!(sublist.child_count(), 3); // 3 items
    }

    #[test]
    fn sublist_spacing() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.id("MAIN");
                list.spacing(1);
                list.item("Test A")
                    .with_item("Test B", |item| {
                        List::new(item, |sublist| {
                            sublist.id("SUBLIST");
                            sublist.item("Apple").item("Banana").item("Carrot");
                        });
                    })
                    .item("Test C");
            });
        });

        let mut buffer = Buffer::new(80, 50);
        ui.draw(&mut buffer);

        assert_eq!(extract_line(&buffer, 0, 0, 12), "-\0Test A\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 1, 12), "\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 2, 12), "-\0Test B\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 3, 12), "\0\0-\0Apple\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 4, 12), "\0\0-\0Banana\0\0");
        assert_eq!(extract_line(&buffer, 0, 5, 12), "\0\0-\0Carrot\0\0");
        assert_eq!(extract_line(&buffer, 0, 6, 12), "\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 7, 12), "-\0Test C\0\0\0\0");

        let list = ui.find_by_id("MAIN").unwrap();

        let item_a = list.first_child().unwrap();
        assert_eq!(item_a.pad(), [0, 0, 0, 0]);
        assert_eq!(item_a.pos().unwrap(), (0, 0));
        assert_eq!(item_a.outer_size(), (8, 1));
        assert_eq!(item_a.size().unwrap(), (8, 1));

        let item_b = list.children().skip(1).next().unwrap();
        let sublist = item_b.last_child().unwrap();
        assert_eq!(sublist.child_count(), 3); // 3 items
        assert_eq!(sublist.outer_size(), (8, 3)); // 3 items
        assert_eq!(item_b.size().unwrap(), (10, 4));
        assert_eq!(item_b.outer_size(), (10, 5)); // padded on top
        assert_eq!(item_b.size().unwrap(), (10, 4));
        assert_eq!(item_b.pad(), [0, 1, 0, 0]);
        assert_eq!(item_b.pos().unwrap(), (0, 2));

        let item_c = list.last_child().unwrap();
        assert_eq!(item_c.child_count(), 1); // label + list
        assert_eq!(item_c.pad(), [0, 1, 0, 0]);
        assert_eq!(item_c.pos().unwrap(), (0, 7));
        assert_eq!(item_c.outer_size(), (8, 2)); // padded on top
        assert_eq!(item_c.size().unwrap(), (8, 1)); // padded on top

        assert_eq!(list.tag(), "list");
        assert_eq!(list.child_count(), 3); // 3 items
        assert_eq!(list.size().unwrap(), (10, 8));
    }

    #[test]
    fn list_with_width() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.width(10);
                list.item("Item A")
                    .item("This is a longer item and it should wrap.")
                    .item("Item C");
            });
        });

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "list");
        assert_eq!(list.child_count(), 3);
        assert_eq!(list.size().unwrap(), (10, 8));
    }

    #[test]
    fn with_width_and_sublist() {
        let ui = dialog((80, 50), "DEFAULT", |body| {
            List::new(body, |list| {
                // list.unordered();   // default
                // list.numbered("#) ");
                // list.alpha("a - ");
                // list.alpha("{A} ");

                list.width(14).id("MAIN");
                list.item("Item A")
                    .with_item("Item B", |item| {
                        List::new(item, |sub| {
                            sub.id("SUBLIST");
                            sub.item("This is a longer item and it will be wrapped.");
                        });
                    })
                    .item("Item C");
            });
        });

        let sublist = ui.find_by_id("SUBLIST").unwrap();
        assert_eq!(sublist.child_count(), 1);
        assert_eq!(sublist.size().unwrap(), (12, 5));

        let list = ui.root().first_child().unwrap();
        assert_eq!(list.tag(), "list");
        assert_eq!(list.id().as_ref().unwrap(), "MAIN");
        assert_eq!(list.child_count(), 3);
        assert_eq!(list.size().unwrap(), (14, 8));
    }
}
