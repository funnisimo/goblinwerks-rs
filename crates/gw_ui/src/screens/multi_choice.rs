use crate::screens::MsgBox;
use crate::ui::dialog;
use crate::ui::Align;
use crate::ui::Button;
use crate::ui::Checkbox;
use crate::ui::Div;
use crate::ui::Frame;
use crate::ui::ParentNode;
use crate::ui::Select;
use crate::ui::Span;
use crate::ui::Text;
use crate::ui::UiAction;
use crate::ui::UI;
use crate::ui::{Keyed, Margined, Padded, Positioned};
use gw_app::messages::Messages;
use gw_app::VirtualKeyCode;
use gw_app::{AppEvent, Ecs, Screen, ScreenResult};
use gw_util::value::{Key, Value};
use std::collections::HashMap;

/// Called when the multi choice dialog is closed - data is a map of the key to value for the checked items.
/// None for cancelled, empty for None checked.
pub type MultiChoiceResultFn = dyn FnOnce(&mut Ecs, Option<HashMap<Key, Value>>) -> ();

#[derive(Debug)]
pub struct MultiChoiceItem {
    text: String,
    key: Key,
    count: u16,
}

impl MultiChoiceItem {
    fn new(text: &str, key: Key, count: u16) -> Self {
        MultiChoiceItem {
            text: text.to_string(),
            key,
            count,
        }
    }
}

impl From<String> for MultiChoiceItem {
    fn from(text: String) -> Self {
        MultiChoiceItem::new(&text, text.clone().into(), 1)
    }
}

impl From<&str> for MultiChoiceItem {
    fn from(text: &str) -> Self {
        MultiChoiceItem::new(text, text.into(), 1)
    }
}

impl From<(&str, Key)> for MultiChoiceItem {
    fn from(info: (&str, Key)) -> Self {
        MultiChoiceItem::new(info.0, info.1, 1)
    }
}

impl From<(String, Key)> for MultiChoiceItem {
    fn from(info: (String, Key)) -> Self {
        MultiChoiceItem::new(&info.0, info.1, 1)
    }
}

impl From<(&str, Key, u16)> for MultiChoiceItem {
    fn from(info: (&str, Key, u16)) -> Self {
        MultiChoiceItem::new(info.0, info.1, info.2)
    }
}

impl From<(String, Key, u16)> for MultiChoiceItem {
    fn from(info: (String, Key, u16)) -> Self {
        MultiChoiceItem::new(&info.0, info.1, info.2)
    }
}

pub struct MultiChoiceBuilder {
    id: String,
    done: Option<Box<MultiChoiceResultFn>>,
    title: String,
    prompt: String,
    selected: Vec<bool>,
    items: Vec<MultiChoiceItem>,
    class: String,
    page_size: (u32, u32),
    font: String,
    checkbox: bool,
    count: Option<String>,
    glyphs: Option<(String, String)>,
}

impl MultiChoiceBuilder {
    fn new(id: &str) -> Self {
        MultiChoiceBuilder {
            id: id.to_owned(),
            title: "".to_owned(),
            prompt: "".to_owned(),
            selected: Vec::new(),
            items: Vec::new(),
            class: "choice".to_owned(),
            page_size: (80, 50),
            font: "DEFAULT".to_owned(),
            done: None,
            checkbox: false,
            count: None,
            glyphs: None,
        }
    }

    pub fn title<S: ToString>(mut self, title: S) -> Self {
        self.title = format!("] {} [", title.to_string());
        self
    }

    pub fn prompt<S: ToString>(mut self, prompt: S) -> Self {
        self.prompt = prompt.to_string();
        self
    }

    pub fn items<D: Into<MultiChoiceItem>>(mut self, items: Vec<D>) -> Self {
        for item in items {
            self.items.push((item).into());
        }
        self.selected.resize(self.items.len(), false);
        self
    }

    pub fn item<D: Into<MultiChoiceItem>>(mut self, item: D) -> Self {
        self.items.push(item.into());
        self.selected.resize(self.items.len(), false);
        self
    }

    pub fn class<S: ToString>(mut self, class: S) -> Self {
        self.class = class.to_string();
        self
    }

    pub fn glyphs(mut self, off_glyph: &str, on_glyph: &str) -> Self {
        self.glyphs = Some((off_glyph.to_string(), on_glyph.to_string()));
        self.checkbox = true;
        self
    }

    pub fn checkbox(mut self) -> Self {
        self.checkbox = true;
        self
    }

    pub fn count(mut self, glyph: &str) -> Self {
        self.count = Some(glyph.to_string());
        self.checkbox = true;
        self
    }

    pub fn page_size(mut self, page_size: (u32, u32)) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn font<S: ToString>(mut self, font: S) -> Self {
        self.font = font.to_string();
        self
    }

    pub fn done(mut self, func: Box<MultiChoiceResultFn>) -> Self {
        self.done = Some(func);
        self
    }

    pub fn build(mut self) -> Box<MultiChoice> {
        if self.done.is_none() {
            let id = self.id.clone();
            self.done = Some(Box::new(
                move |app: &mut Ecs, data: Option<HashMap<Key, Value>>| {
                    let res = match data {
                        None => None,
                        Some(x) => Some(Value::Map(x)),
                    };
                    let mut msgs = app.write_global::<Messages>();
                    msgs.push(id.as_ref(), res)
                },
            ));
        }

        Box::new(MultiChoice::new(self))
    }
}

pub struct MultiChoice {
    config: MultiChoiceBuilder,
    selected: Vec<bool>,
    ui: UI,
}

impl MultiChoice {
    pub fn builder(id: &str) -> MultiChoiceBuilder {
        MultiChoiceBuilder::new(id)
    }

    fn new(config: MultiChoiceBuilder) -> Self {
        let ui = dialog(config.page_size, config.font.as_str(), |dlg| {
            dlg.class("choice")
                .class(&config.class)
                .bind_key(VirtualKeyCode::Return, UiAction::activate("OK"))
                .bind_key(VirtualKeyCode::Escape, UiAction::activate("CANCEL"));

            Frame::new(dlg, |frame| {
                frame
                    .class("choice")
                    .class(&config.class)
                    .margin(1)
                    .pad(1)
                    .spacing(1)
                    .id("FRAME");

                if config.title.len() > 0 {
                    frame.title(&config.title);
                }

                if config.prompt.len() > 0 {
                    Text::new(frame, |txt| {
                        txt.text(&config.prompt);
                        txt.class("choice").class(&config.class);
                    });
                }

                if config.checkbox {
                    init_checkbox(&config, frame);
                } else {
                    init_select(&config, frame);
                }

                Span::new(frame, |span| {
                    span.anchor(Align::Max).spacing(2);

                    Button::new(span, |cancel| {
                        cancel
                            .id("CANCEL")
                            .text("[Cancel]")
                            .width(8)
                            .class("choice")
                            .class("cancel")
                            .class(&config.class);
                    });

                    Button::new(span, |ok| {
                        ok.id("OK")
                            .text("[  Ok  ]")
                            .width(8)
                            .class("choice")
                            .class("ok")
                            .class(&config.class);
                    });
                });
            });
        });

        ui.dump();

        MultiChoice {
            // active: None,
            selected: config.selected.clone(),
            ui,
            config,
        }
    }

    pub fn select_index(mut self, index: usize) -> Self {
        self.selected[index] = true;
        self
    }
}

#[allow(unused_variables)]
impl Screen for MultiChoice {
    fn is_full_screen(&self) -> bool {
        self.ui.is_full_screen()
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        self.ui.input(app, ev);
        ScreenResult::Continue
    }

    fn message(&mut self, app: &mut Ecs, id: &str, value: Option<Value>) -> ScreenResult {
        match id.as_ref() {
            "OK" => {
                let ret = get_value(self);
                if ret.is_none() || ret.as_ref().unwrap().is_empty() {
                    return ScreenResult::Push(
                        MsgBox::builder("MSG_BOX")
                            .msg("You must select something.")
                            .class("error")
                            .build(),
                    );
                }
                println!("MultiChoice - {}, {:?}", &self.config.id, ret);
                if let Some(done) = self.config.done.take() {
                    done(app, ret);
                }

                ScreenResult::Pop
            }
            "CANCEL" => {
                if let Some(done) = self.config.done.take() {
                    done(app, None);
                }
                ScreenResult::Pop
            }
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, ctx: &mut Ecs) {
        self.ui.render(ctx);
    }

    // fn teardown(&mut self, ctx: &mut  AppContext) {}
}

fn init_select(config: &MultiChoiceBuilder, parent: &dyn ParentNode) {
    Select::new(parent, |sel| {
        sel.id("SELECT");
        sel.class("choice").class(&config.class);
        sel.multiple();

        for (idx, txt) in config.items.iter().enumerate() {
            sel.with_item(&txt.text, |opt| {
                opt.value(idx.into());
                opt.class("choice").class(&config.class);
            });
        }
    });
}

fn init_checkbox(config: &MultiChoiceBuilder, parent: &dyn ParentNode) {
    Div::new(parent, |div| {
        div.id("CHECKS");

        for (idx, item) in config.items.iter().enumerate() {
            Checkbox::new(div, |chk| {
                chk.text(&item.text);
                chk.id(&format!("{}", idx));
                if let Some(ref glyph) = config.count {
                    chk.count(glyph, item.count);
                }
                if let Some(ref glyphs) = config.glyphs {
                    chk.glyphs(&glyphs.0, &glyphs.1);
                }
            });
        }
    });
}

fn get_value(dlg: &MultiChoice) -> Option<HashMap<Key, Value>> {
    if dlg.config.checkbox {
        let mut ret: HashMap<Key, Value> = HashMap::new();

        let div = dlg.ui.find_by_id("CHECKS").unwrap();

        for child in div.children() {
            let id = child.id().as_ref().unwrap().clone();
            let idx: usize = id.parse().unwrap();

            let item = &dlg.config.items[idx];

            if child.has_prop("checked") {
                if dlg.config.count.is_some() {
                    ret.insert(item.key.clone(), child.value().unwrap());
                } else {
                    ret.insert(item.key.clone(), true.into());
                }
            }
        }
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    } else {
        let ret = match dlg.ui.find_by_id("SELECT").unwrap().value() {
            None => None, // TODO - return ScreenResult::Push(MsgBox::builder().msg("You must select something.").class("error").build()),
            Some(Value::List(list)) => {
                let mut map: HashMap<Key, Value> = HashMap::new();
                for data in list {
                    let idx: usize = data.try_into().unwrap();
                    let key = dlg.config.items[idx].key.clone();
                    map.insert(key, Value::Boolean(true));
                }
                Some(map)
            }
            Some(Value::Map(map)) => Some(map),
            _ => panic!("Unexpected value from select"),
        };
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn select() {
        let mut dlg = MultiChoice::builder("MULTI")
            .item("ONE")
            .item("TWO")
            .build();

        assert!(dlg.ui.focused().is_some());

        let focused = dlg.ui.focused().unwrap();
        assert!(focused.has_id("SELECT"));
        assert_eq!(focused.value(), None);

        dlg.ui.handle_key(&VirtualKeyCode::Space.into());
        let mut list: Vec<Value> = Vec::new();
        list.push(0_usize.into());
        assert_eq!(focused.value(), Some(Value::List(list)));

        let val = get_value(&dlg).unwrap();
        let mut map: HashMap<Key, Value> = HashMap::new();
        map.insert("ONE".into(), true.into());
        assert_eq!(val, map);
    }

    #[test]
    fn select_val() {
        let mut dlg = MultiChoice::builder("MULTI")
            .item(("ONE", "K1".into()))
            .item(("TWO", "K2".into()))
            .build();

        assert!(dlg.ui.focused().is_some());

        let focused = dlg.ui.focused().unwrap();
        assert!(focused.has_id("SELECT"));
        assert_eq!(focused.value(), None);

        dlg.ui.handle_key(&VirtualKeyCode::Space.into());
        let mut list: Vec<Value> = Vec::new();
        list.push(0_usize.into());
        assert_eq!(focused.value(), Some(Value::List(list)));

        let val = get_value(&dlg).unwrap();
        let mut map: HashMap<Key, Value> = HashMap::new();
        map.insert("K1".into(), true.into());
        assert_eq!(val, map);
    }

    #[test]
    fn check() {
        let mut dlg = MultiChoice::builder("MULTI")
            .checkbox()
            .item("ONE")
            .item("TWO")
            .build();

        assert!(dlg.ui.focused().is_some());

        let focused = dlg.ui.focused().unwrap();
        assert!(focused.has_tag("checkbox"));
        assert_eq!(focused.value(), None);

        dlg.ui.handle_key(&VirtualKeyCode::Space.into());
        assert!(focused.has_prop("checked"));
        assert!(focused.has_id("0"));
        assert_eq!(focused.value(), Some(true.into()));

        let val = get_value(&dlg).unwrap();
        let mut map: HashMap<Key, Value> = HashMap::new();
        map.insert("ONE".into(), true.into());
        assert_eq!(val, map);
    }

    #[test]
    fn check_val() {
        let mut dlg = MultiChoice::builder("MULTI")
            .checkbox()
            .item(("ONE", "K1".into()))
            .item(("TWO", "K2".into()))
            .build();

        assert!(dlg.ui.focused().is_some());

        let focused = dlg.ui.focused().unwrap();
        assert!(focused.has_tag("checkbox"));
        assert_eq!(focused.value(), None);

        dlg.ui.handle_key(&VirtualKeyCode::Space.into());
        assert!(focused.has_prop("checked"));
        assert!(focused.has_id("0"));
        assert_eq!(focused.value(), Some(true.into()));

        let val = get_value(&dlg).unwrap();
        let mut map: HashMap<Key, Value> = HashMap::new();
        map.insert("K1".into(), true.into());
        assert_eq!(val, map);
    }

    #[test]
    fn check_count() {
        let mut dlg = MultiChoice::builder("MULTI")
            .checkbox()
            .count("#")
            .item(("ONE", "K1".into(), 2))
            .item(("TWO", "K2".into(), 3))
            .build();

        assert!(dlg.ui.focused().is_some());

        let focused = dlg.ui.focused().unwrap();
        assert!(focused.has_tag("checkbox"));
        assert_eq!(focused.value(), None);

        dlg.ui.handle_key(&VirtualKeyCode::Space.into());
        assert!(focused.has_prop("checked"));
        assert!(focused.has_id("0"));
        assert_eq!(focused.value(), Some(2_i32.into()));

        let val = get_value(&dlg).unwrap();
        let mut map: HashMap<Key, Value> = HashMap::new();
        map.insert("K1".into(), 2_i32.into());
        assert_eq!(val, map);
    }
}
