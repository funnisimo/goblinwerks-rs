use super::*;
use conapp::color::{BLACK, RGBA, WHITE};
use conapp::draw::BorderType;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct Style {
    selector: Selector,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    border_fg: Option<RGBA>,
    border_bg: Option<RGBA>,
    border: Option<BorderType>,
    accent_fg: Option<RGBA>,
}

impl Style {
    pub fn new(selector: Selector) -> Self {
        Style {
            selector,
            fg: None,
            bg: None,
            border_fg: None,
            border_bg: None,
            border: None,
            accent_fg: None,
        }
    }

    pub fn local() -> Self {
        Style::new("$".into())
    }

    pub fn score(&self) -> u32 {
        self.selector.score()
    }

    pub fn fg(&self) -> Option<RGBA> {
        self.fg
    }

    pub fn bg(&self) -> Option<RGBA> {
        self.bg
    }

    pub fn border_fg(&self) -> Option<RGBA> {
        self.border_fg
    }

    pub fn border_bg(&self) -> Option<RGBA> {
        self.border_bg
    }

    pub fn border(&self) -> Option<BorderType> {
        self.border
    }

    pub fn accent_fg(&self) -> Option<RGBA> {
        self.accent_fg
    }

    pub fn is_base_match(&self, el: &Element) -> bool {
        self.selector.is_base_match(el)
    }

    pub fn matches(&self, el: &Element) -> bool {
        self.selector.matches(el)
    }

    pub fn set_fg(&mut self, fg: RGBA) {
        self.fg = Some(fg);
    }

    // pub fn set_fg_name(&mut self, fg: &str) {
    //     self.fg = get_color(fg);
    // }

    pub fn set_bg(&mut self, bg: RGBA) {
        self.bg = Some(bg);
    }

    // pub fn set_bg_name(&mut self, bg: &str) {
    //     self.bg = get_color(bg);
    // }

    pub fn set_border_fg(&mut self, fg: RGBA) {
        self.border_fg = Some(fg);
    }

    // pub fn set_border_fg_name(&mut self, fg: &str) {
    //     self.border_fg = get_color(fg);
    // }

    pub fn set_border_bg(&mut self, bg: RGBA) {
        self.border_bg = Some(bg);
    }

    // pub fn set_border_bg_name(&mut self, bg: &str) {
    //     self.border_bg = get_color(bg);
    // }

    pub fn set_border(&mut self, border: Option<BorderType>) {
        self.border = border;
    }

    pub fn set_accent_fg(&mut self, fg: RGBA) {
        self.accent_fg = Some(fg);
    }

    // pub fn set_accent_fg_name(&mut self, fg: &str) {
    //     self.accent_fg = get_color(fg);
    // }

    // pub fn set(&mut self, key: &str, value: &str) {
    //     if key == "fg" || key == "color" {
    //         self.set_fg_name(value);
    //     } else if key == "bg" || key == "background-color" {
    //         self.set_bg_name(value);
    //     } else if key == "border-color" {
    //         self.set_border_fg_name(value);
    //     } else if key == "border-block-color" {
    //         self.set_border_bg_name(value);
    //     } else if key == "border" {
    //         match value.parse::<u32>() {
    //             Err(_) => {}
    //             Ok(v) => match v {
    //                 0 => self.border = None,
    //                 1 => self.border = Some(BorderType::Single),
    //                 2 => self.border = Some(BorderType::Double),
    //                 _ => self.border = Some(BorderType::Color),
    //             },
    //         }
    //     } else if key == "accent-color" {
    //         self.set_accent_fg_name(value);
    //     }
    // }
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Style");

        s.field("selector", &self.selector);

        if let Some(ref fg) = self.fg {
            s.field("fg", fg);
        }

        if let Some(ref bg) = self.bg {
            s.field("bg", bg);
        }

        if let Some(ref border_fg) = self.border_fg {
            s.field("border_fg", border_fg);
        }

        if let Some(ref border_bg) = self.border_bg {
            s.field("border_bg", border_bg);
        }

        if let Some(ref border) = self.border {
            s.field("border", border);
        }

        if let Some(ref accent_fg) = self.accent_fg {
            s.field("accent_fg", accent_fg);
        }

        s.finish()
    }
}

pub struct ComputedStyle {
    styles: Vec<Arc<Style>>,
    el: Element,
}

impl ComputedStyle {
    pub fn new(mut styles: Vec<Arc<Style>>, el: &Element) -> Self {
        if let Some(ref local_style) = el.borrow().local_style {
            styles.push(local_style.clone());
        }
        // sort descending - so first not None value is the one to use
        styles.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());

        ComputedStyle {
            styles,
            el: el.clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.styles.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<Style>> {
        self.styles.iter()
    }

    pub fn fg(&self) -> RGBA {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.fg)
        {
            Some(rgba) => rgba,
            None => match self.el.parent() {
                None => WHITE,
                Some(ref parent) => parent.style().fg(),
            },
        }
    }

    pub fn bg(&self) -> RGBA {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.bg)
        {
            Some(rgba) => rgba,
            None => match self.el.parent() {
                None => BLACK,
                Some(ref parent) => {
                    // println!("Using parent bg");
                    parent.style().bg()
                }
            },
        }
    }

    pub fn border_fg(&self) -> RGBA {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.border_fg)
        {
            Some(rgba) => rgba,
            None => match self.el.parent() {
                None => WHITE,
                Some(ref parent) => parent.style().border_fg(),
            },
        }
    }

    pub fn border_bg(&self) -> RGBA {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.border_bg)
        {
            Some(rgba) => rgba,
            None => match self.el.parent() {
                None => BLACK,
                Some(ref parent) => parent.style().border_bg(),
            },
        }
    }

    pub fn border(&self) -> Option<BorderType> {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.border.clone())
        {
            Some(border) => Some(border),
            None => match self.el.parent() {
                None => None,
                Some(ref parent) => parent.style().border(),
            },
        }
    }

    pub fn accent_fg(&self) -> RGBA {
        match self
            .styles
            .iter()
            .filter(|s| s.matches(&self.el)) // filter to match changes in class+props
            .find_map(|s| s.accent_fg)
        {
            Some(rgba) => rgba,
            None => match self.el.parent() {
                None => WHITE,
                Some(ref parent) => parent.style().accent_fg(),
            },
        }
    }
}

impl Debug for ComputedStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = f.debug_list();
        for style in self.styles.iter() {
            l.entry(&style);
        }
        l.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::prelude::colors;
    // use crate::prelude::init_colors;
    use conapp::color::{BLACK, RGBA, WHITE};

    #[test]
    fn computed_default() {
        // init_colors();

        let a = Arc::new(Style::new("*".into())); // low priority
        let b = Arc::new(Style::new("text".into())); // med
        let c = Arc::new(Style::new("#id".into())); // hi
                                                    // let mut local = Arc::new(Style::new("$".into())); // highest

        let el = Element::new(&TEXT);
        // el.borrow_mut().local_style = Some(local.clone());

        assert_eq!(a.fg, None);
        assert_eq!(b.fg, None);
        assert_eq!(c.fg, None);

        let style = ComputedStyle::new(vec![a, b, c], &el);

        assert_eq!(style.fg(), WHITE);
        assert_eq!(style.bg(), BLACK);
    }

    #[test]
    fn computed_basic() {
        let mut a = Style::new("*".into()); // low priority
        a.set_fg("#0f0".into());
        a.set_bg("#0f0".into());
        let mut b = Style::new("text".into()); // med
        b.set_bg("#00f".into());
        let c = Style::new("#id".into()); // hi

        let el = Element::new(&TEXT);
        {
            let mut node = el.borrow_mut();
            node.id = Some("id".to_owned());
        }

        assert!(a.matches(&el));
        assert!(b.matches(&el));
        assert!(c.matches(&el));

        assert_eq!(a.fg, Some(RGBA::rgb(0, 255, 0)));
        assert_eq!(b.fg, None);
        assert_eq!(c.fg, None);

        assert_eq!(a.bg, Some(RGBA::rgb(0, 255, 0)));
        assert_eq!(b.bg, Some(RGBA::rgb(0, 0, 255)));
        assert_eq!(c.bg, None);

        let a = Arc::new(a);
        let b = Arc::new(b);
        let c = Arc::new(c);

        let styles = vec![a.clone(), b.clone(), c.clone()];
        let style = ComputedStyle::new(styles, &el);

        assert_eq!(style.styles[0], c);
        assert_eq!(style.styles[1], b);
        assert_eq!(style.styles[2], a);

        assert_eq!(style.fg(), RGBA::rgb(0, 255, 0));
        assert_eq!(style.bg(), RGBA::rgb(0, 0, 255));
    }

    #[test]
    fn computed_local() {
        let a = Style::new("*".into()); // low priority
        let mut b = Style::new("text".into()); // med
        b.set_bg("#f00".into());
        b.set_fg("#f00".into());
        let c = Style::new("#id".into()); // hi
        let mut local = Style::local(); // highest
        local.set_bg("#0f0".into());

        let el = Element::new(&TEXT);
        {
            let mut node = el.borrow_mut();
            node.local_style = Some(Arc::new(local));
            node.id = Some("id".to_owned());
        }

        assert!(a.matches(&el));
        assert!(b.matches(&el));
        assert!(c.matches(&el));

        assert_eq!(a.fg, None);
        assert_eq!(b.fg, Some(RGBA::rgb(255, 0, 0)));
        assert_eq!(c.fg, None);

        assert_eq!(a.bg, None);
        assert_eq!(b.bg, Some(RGBA::rgb(255, 0, 0)));
        assert_eq!(c.bg, None);

        let styles = vec![a, b, c];
        let rc_styles: Vec<Arc<Style>> = styles.iter().map(|s| Arc::new(s.clone())).collect();

        let style = ComputedStyle::new(rc_styles, &el);

        assert_eq!(style.fg(), RGBA::rgb(255, 0, 0));
        assert_eq!(style.bg(), RGBA::rgb(0, 255, 0));
    }
}
