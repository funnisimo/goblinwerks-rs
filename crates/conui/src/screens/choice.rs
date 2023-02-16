use crate::ui::dialog;
use crate::ui::Align;
use crate::ui::Button;
use crate::ui::Frame;
use crate::ui::ParentNode;
use crate::ui::Radio;
use crate::ui::Select;
use crate::ui::Span;
use crate::ui::Text;
use crate::ui::UiAction;
use crate::ui::UI;
use crate::ui::{Keyed, Margined, Padded, Positioned};
use conapp::MsgData;
use conapp::VirtualKeyCode;
use conapp::{AppContext, AppEvent, Screen, ScreenResult};

/// Called when the choice dialog is closed - data is the value of the option(s) that is/are checked.
/// None for cancelled.
pub type ChoiceResultFn = dyn FnOnce(&mut AppContext, Option<MsgData>) -> ();

#[derive(Debug)]
pub struct ChoiceItem {
    text: String,
    value: MsgData,
}

impl ChoiceItem {
    fn new(text: &str, value: MsgData) -> Self {
        ChoiceItem {
            text: text.to_string(),
            value,
        }
    }
}

impl From<String> for ChoiceItem {
    fn from(text: String) -> Self {
        ChoiceItem {
            value: text.clone().into(),
            text,
        }
    }
}

impl From<&str> for ChoiceItem {
    fn from(text: &str) -> Self {
        ChoiceItem::new(text, text.into())
    }
}

impl From<(&str, MsgData)> for ChoiceItem {
    fn from(info: (&str, MsgData)) -> Self {
        ChoiceItem::new(info.0, info.1)
    }
}

impl From<(String, MsgData)> for ChoiceItem {
    fn from(info: (String, MsgData)) -> Self {
        ChoiceItem::new(&info.0, info.1)
    }
}

pub struct ChoiceBuilder {
    id: String,
    done: Option<Box<ChoiceResultFn>>,
    title: String,
    prompt: String,
    selected: Vec<bool>,
    items: Vec<ChoiceItem>,
    class: String,
    page_size: (u32, u32),
    font: String,
    radio: bool,
}

impl ChoiceBuilder {
    fn new(id: &str) -> Self {
        ChoiceBuilder {
            id: id.to_owned(),
            title: "".to_owned(),
            prompt: "".to_owned(),
            selected: Vec::new(),
            items: Vec::new(),
            class: "choice".to_owned(),
            page_size: (80, 50),
            font: "DEFAULT".to_owned(),
            done: None,
            radio: false,
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

    pub fn items<D: Into<ChoiceItem>>(mut self, items: Vec<D>) -> Self {
        for item in items {
            self.items.push((item).into());
        }
        self.selected.resize(self.items.len(), false);
        self
    }

    pub fn item<D: Into<ChoiceItem>>(mut self, item: D) -> Self {
        self.items.push(item.into());
        self.selected.resize(self.items.len(), false);
        self
    }

    pub fn class<S: ToString>(mut self, class: S) -> Self {
        self.class = class.to_string();
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

    pub fn radio(mut self) -> Self {
        self.radio = true;
        self
    }

    pub fn done(mut self, func: Box<ChoiceResultFn>) -> Self {
        self.done = Some(func);
        self
    }

    pub fn build(mut self) -> Box<Choice> {
        if self.done.is_none() {
            let id = self.id.clone();
            self.done = Some(Box::new(
                move |app: &mut AppContext, data: Option<MsgData>| {
                    app.send_message(id.as_ref(), data)
                },
            ));
        }

        Box::new(Choice::new(self))
    }
}

pub struct Choice {
    config: ChoiceBuilder,
    selected: Vec<bool>,
    ui: UI,
}

impl Choice {
    pub fn builder(id: &str) -> ChoiceBuilder {
        ChoiceBuilder::new(id)
    }

    fn new(config: ChoiceBuilder) -> Self {
        let ui = dialog(config.page_size, config.font.as_str(), |dlg| {
            dlg.class("choice")
                .class(&config.class)
                .bind_key(VirtualKeyCode::Return, UiAction::activate("OK".to_owned()))
                .bind_key(
                    VirtualKeyCode::Escape,
                    UiAction::activate("CANCEL".to_owned()),
                );

            Frame::new(dlg, |frame| {
                frame.class("choice").class(&config.class).margin(1).pad(1);

                if config.title.len() > 0 {
                    frame.title(&config.title);
                }

                if config.prompt.len() > 0 {
                    Text::new(frame, |txt| {
                        txt.text(&config.prompt).pad_bottom(1);
                        txt.class("choice").class(&config.class);
                    });
                }

                if config.radio {
                    init_radio(&config, frame);
                } else {
                    init_select(&config, frame);
                }

                Span::new(frame, |span| {
                    span.pad_top(1).anchor(Align::Max).spacing(2);

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

        Choice {
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
impl Screen for Choice {
    fn is_full_screen(&self) -> bool {
        self.ui.is_full_screen()
    }

    fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        self.ui.input(app, ev);
        ScreenResult::Continue
    }

    fn message(
        &mut self,
        app: &mut AppContext,
        id: String,
        value: Option<MsgData>,
    ) -> ScreenResult {
        match id.as_ref() {
            "OK" => {
                let ret = self.ui.find_by_id("SELECT").unwrap().value();
                if ret.is_none() {
                    // TODO - return ScreenResult::Push(MsgBox::builder().msg("You must select something.").class("error").build()),
                }
                println!("Choice - {}, {:?}", &self.config.id, ret);
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

    fn render(&mut self, ctx: &mut AppContext) {
        self.ui.render(ctx);
    }

    // fn teardown(&mut self, ctx: &mut  AppContext) {}
}

fn init_select(config: &ChoiceBuilder, parent: &dyn ParentNode) {
    Select::new(parent, |sel| {
        sel.id("SELECT");
        sel.class("choice").class(&config.class);

        for txt in config.items.iter() {
            sel.with_item(&txt.text, |opt| {
                opt.value(txt.value.clone());
                opt.class("choice").class(&config.class);
            });
        }
    });
}

fn init_radio(config: &ChoiceBuilder, parent: &dyn ParentNode) {
    Radio::new(parent, |sel| {
        sel.id("SELECT");
        sel.class("choice").class(&config.class);

        for txt in config.items.iter() {
            sel.with_item(&txt.text, |opt| {
                opt.value(txt.value.clone());
                opt.class("choice").class(&config.class);
            });
        }
    });
}
