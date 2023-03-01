use crate::ui::Element;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref CSS_RE: Regex = Regex::new(r"(?:(\w+|\*|\$)|#(\w+)|\.([^\.: ]+))|(?::(?:(?:not\(\.([^\)]+)\))|(?:not\(:([^\)]+)\))|([^\.: ]+)))").unwrap();
}

#[derive(PartialEq, Debug, Clone)]
pub enum Matcher {
    All,
    Local,
    Id(String),
    Tag(String),
    Class(String),
    NotClass(String),
    Prop(String),
    NotProp(String),

    Empty,
    FirstChild,
    LastChild,
    OnlyChild,

    Even,
    Odd,

    Current(Selector),
    Parent(Selector),
    Ancestor(Selector),
}

impl Matcher {
    pub fn matches(&self, el: &Element) -> bool {
        let data = el.borrow();
        match self {
            Matcher::All => true,
            Matcher::Local => true,
            Matcher::Id(id) => match data.id.as_ref() {
                None => false,
                Some(has) => has == id,
            },
            Matcher::Tag(tag) => data.tag.as_str() == tag,
            Matcher::Class(class) => data.classes.contains(class),
            Matcher::NotClass(class) => !data.classes.contains(class),
            Matcher::Prop(prop) => data.props.contains(prop),
            Matcher::NotProp(prop) => !data.props.contains(prop),

            Matcher::Empty => data.children.is_empty(),
            Matcher::FirstChild => match data.parent_element() {
                None => false,
                Some(parent) => match parent.first_child() {
                    None => false,
                    Some(first) => first.is(el),
                },
            },
            Matcher::LastChild => match data.parent_element() {
                None => false,
                Some(parent) => match parent.last_child() {
                    None => false,
                    Some(last) => last.is(el),
                },
            },
            Matcher::OnlyChild => match data.parent_element() {
                None => false,
                Some(parent) => parent.child_count() == 1,
            },

            Matcher::Even => match data.parent_element() {
                None => false,
                Some(parent) => match parent.child_position(el) {
                    None => false,
                    Some(n) => n % 2 == 0,
                },
            },
            Matcher::Odd => match data.parent_element() {
                None => false,
                Some(parent) => match parent.child_position(el) {
                    None => false,
                    Some(n) => n % 2 == 1,
                },
            },

            Matcher::Current(selector) => selector.matches(el),
            Matcher::Parent(selector) => match data.parent_element() {
                None => false,
                Some(parent) => selector.matches(&parent),
            },
            Matcher::Ancestor(selector) => {
                let mut parent = data.parent_element();
                loop {
                    match parent {
                        None => return false,
                        Some(parent_el) => {
                            if selector.matches(&parent_el) {
                                return true;
                            }
                            parent = parent_el.parent();
                        }
                    }
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Matcher::All => 0,
            Matcher::Local => 10000,
            Matcher::Id(_) => 1000,
            Matcher::Tag(_) => 100,
            Matcher::Class(_) => 10,
            Matcher::NotClass(_) => 10,
            Matcher::Prop(_) => 2,
            Matcher::NotProp(_) => 2,

            Matcher::Empty => 2,
            Matcher::FirstChild => 2,
            Matcher::LastChild => 2,
            Matcher::OnlyChild => 2,

            Matcher::Even => 1,
            Matcher::Odd => 1,

            Matcher::Current(sel) => sel.score(),
            Matcher::Parent(sel) => sel.score(),
            Matcher::Ancestor(sel) => sel.score(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Selector {
    matchers: Vec<Matcher>,
}

impl Selector {
    pub fn new(text: &str) -> Self {
        text.parse().unwrap()
    }

    pub fn score(&self) -> u32 {
        self.matchers.iter().fold(0, |out, m| out + m.score())
    }

    fn push(&mut self, matcher: Matcher) {
        self.matchers.push(matcher)
    }

    pub fn matches(&self, el: &Element) -> bool {
        self.matchers.iter().all(|m| m.matches(el))
    }

    pub fn is_base_match(&self, el: &Element) -> bool {
        self.matchers.iter().all(|m| match m {
            Matcher::All => el.is_root(),
            Matcher::Local => true,

            Matcher::Tag(tag) => el.has_tag(tag),
            Matcher::Id(id) => el.has_id(id),
            Matcher::Prop(_) => true,
            Matcher::Class(_) => true,
            Matcher::NotClass(_) => true,
            Matcher::NotProp(_) => true,

            Matcher::Empty => m.matches(el),
            Matcher::FirstChild => m.matches(el),
            Matcher::LastChild => m.matches(el),
            Matcher::OnlyChild => m.matches(el),

            Matcher::Even => m.matches(el),
            Matcher::Odd => m.matches(el),

            Matcher::Current(sel) => sel.is_base_match(el),
            Matcher::Parent(sel) => match el.parent() {
                None => false,
                Some(parent) => sel.is_base_match(&parent),
            },
            Matcher::Ancestor(sel) => {
                let mut current = el.parent();
                let mut res = false;
                while res == false && current.is_some() {
                    let parent = current.unwrap();
                    if sel.is_base_match(&parent) {
                        res = true;
                    }
                    current = parent.parent();
                }
                res
            }
        })
    }
}

impl From<&str> for Selector {
    fn from(text: &str) -> Self {
        match Selector::from_str(text) {
            Ok(sel) => sel,
            Err(e) => panic!("{}", e),
        }
    }
}

impl FromStr for Selector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("Parse Selector - {}", s);
        let mut selector = Selector {
            matchers: Vec::new(),
        };
        let s = s.trim();
        let parts: Vec<&str> = s.split(" ").map(|t| t.trim()).collect();
        if parts.len() > 3 {
            return Err(format!("Unhandled CSS format - {}", s));
        }
        if parts.len() == 3 {
            if parts[1] != ">" {
                return Err(format!("We only support child '>' notation - {}", s));
            }
            let parent: Selector = parts[0].parse().unwrap();
            let current: Selector = parts[2].parse().unwrap();

            selector.matchers.push(Matcher::Parent(parent));
            selector.matchers.push(Matcher::Current(current));
            return Ok(selector);
        } else if parts.len() == 2 {
            let ancestor: Selector = parts[0].parse().unwrap();
            let current: Selector = parts[1].parse().unwrap();

            selector.matchers.push(Matcher::Ancestor(ancestor));
            selector.matchers.push(Matcher::Current(current));
            return Ok(selector);
        }

        if s.len() == 0 || s == "*" {
            selector.matchers.push(Matcher::All);
            return Ok(selector);
        }

        // RE
        for captures in CSS_RE.captures_iter(s) {
            if let Some(tag) = captures.get(1) {
                let tag = tag.as_str();
                if tag == "*" {
                } else if tag == "$" {
                    selector.push(Matcher::Local);
                } else {
                    selector.push(Matcher::Tag(tag.to_owned()));
                }
            }
            if let Some(id) = captures.get(2) {
                selector.push(Matcher::Id(id.as_str().to_owned()));
            }
            if let Some(class) = captures.get(3) {
                let class_txt = class.as_str();
                if class_txt.starts_with("not(") {
                    let class = &class_txt[4..class_txt.len() - 1];
                    selector.push(Matcher::NotClass(class.to_owned()));
                } else {
                    selector.push(Matcher::Class(class_txt.to_owned()));
                }
            }
            if let Some(prop) = captures.get(6) {
                let prop_txt = prop.as_str();
                if prop_txt == "even" {
                    selector.push(Matcher::Even);
                } else if prop_txt == "odd" {
                    selector.push(Matcher::Odd);
                } else if prop_txt == "first-child" {
                    selector.push(Matcher::FirstChild);
                } else if prop_txt == "last-child" {
                    selector.push(Matcher::LastChild);
                } else if prop_txt == "empty" {
                    selector.push(Matcher::Empty);
                } else if prop_txt == "unchecked" {
                    selector.push(Matcher::NotProp("checked".to_owned()));
                } else if prop_txt == "enabled" {
                    selector.push(Matcher::NotProp("disabled".to_owned()));
                } else if prop_txt == "valid" {
                    selector.push(Matcher::NotProp("invalid".to_owned()));
                } else if prop_txt.starts_with("not(") {
                    let prop = &prop_txt[4..prop_txt.len() - 1];
                    selector.push(Matcher::NotProp(prop.to_owned()));
                } else {
                    selector.push(Matcher::Prop(prop_txt.to_owned()));
                }
            }
            // println!("{:?}", captures);
        }

        if selector.matchers.len() == 0 {
            return Err(format!("Failed to parse Selector: {}", s));
        }

        Ok(selector)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_parse() {
        let id = Selector::new("#id");
        assert_eq!(id.matchers, vec![Matcher::Id("id".to_owned())]);

        let tag = Selector::new("tag");
        assert_eq!(tag.matchers, vec![Matcher::Tag("tag".to_owned())]);

        let star_class = Selector::new("*.class");
        assert_eq!(
            star_class.matchers,
            vec![Matcher::Class("class".to_owned())]
        );

        let class = Selector::new(".class");
        assert_eq!(class.matchers, vec![Matcher::Class("class".to_owned())]);

        let not_class = Selector::new(".not(class)");
        assert_eq!(
            not_class.matchers,
            vec![Matcher::NotClass("class".to_owned())]
        );

        let double_class = Selector::new(".classA.classB");
        assert_eq!(
            double_class.matchers,
            vec![
                Matcher::Class("classA".to_owned()),
                Matcher::Class("classB".to_owned())
            ]
        );

        let prop = Selector::new(":prop");
        assert_eq!(prop.matchers, vec![Matcher::Prop("prop".to_owned())]);

        let star_prop = Selector::new("*:prop");
        assert_eq!(star_prop.matchers, vec![Matcher::Prop("prop".to_owned())]);

        let not_prop = Selector::new(":not(prop)");
        assert_eq!(not_prop.matchers, vec![Matcher::NotProp("prop".to_owned())]);

        let double_prop = Selector::new(":propA:propB");
        assert_eq!(
            double_prop.matchers,
            vec![
                Matcher::Prop("propA".to_owned()),
                Matcher::Prop("propB".to_owned())
            ]
        );

        let tag_prop = Selector::new("tag:prop");
        assert_eq!(
            tag_prop.matchers,
            vec![
                Matcher::Tag("tag".to_owned()),
                Matcher::Prop("prop".to_owned())
            ]
        );

        let all = Selector::new("tag#id.class:prop");
        assert_eq!(
            all.matchers,
            vec![
                Matcher::Tag("tag".to_owned()),
                Matcher::Id("id".to_owned()),
                Matcher::Class("class".to_owned()),
                Matcher::Prop("prop".to_owned())
            ]
        );
    }

    #[test]
    fn parent_parse() {
        let div_p = Selector::new("div > p");
        assert_eq!(
            div_p.matchers,
            vec![
                Matcher::Parent(Selector::new("div")),
                Matcher::Current(Selector::new("p")),
            ]
        );

        match "div + p".parse::<Selector>() {
            Err(_) => {}
            Ok(_) => panic!("invalid selector parse - should error"),
        }
    }

    #[test]
    fn ancestor_parse() {
        let div_p = Selector::new("div p");
        assert_eq!(
            div_p.matchers,
            vec![
                Matcher::Ancestor(Selector::new("div")),
                Matcher::Current(Selector::new("p")),
            ]
        );
    }
}
