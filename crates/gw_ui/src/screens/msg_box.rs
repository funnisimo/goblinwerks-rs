use crate::ui::dialog;
use crate::ui::Align;
use crate::ui::Button;
use crate::ui::Frame;
use crate::ui::Span;
use crate::ui::Text;
use crate::ui::UiAction;
use crate::ui::UI;
use crate::ui::{Keyed, Margined, Padded, Positioned};
use gw_app::MsgData;
use gw_app::VirtualKeyCode;
use gw_app::{AppContext, AppEvent, Screen, ScreenResult};

/// Called when the msgbox is closed - data is Some(true) if ok is clicked.
/// None for cancelled.
pub type MsgBoxResultFn = dyn FnOnce(&mut AppContext, Option<MsgData>) -> ();

#[derive(PartialEq)]
pub enum MsgBoxStyle {
    Ok,
    OkCancel,
    YesNo,
}

pub struct MsgBoxBuilder {
    id: String,
    done: Option<Box<MsgBoxResultFn>>,
    title: String,
    prompt: String,
    class: String,
    page_size: (u32, u32),
    font: String,
    style: MsgBoxStyle,
}

impl MsgBoxBuilder {
    fn new(id: &str) -> Self {
        MsgBoxBuilder {
            id: id.to_owned(),
            title: "".to_owned(),
            prompt: "".to_owned(),
            class: "msg_box".to_owned(),
            page_size: (80, 50),
            font: "DEFAULT".to_owned(),
            done: None,
            style: MsgBoxStyle::Ok,
        }
    }

    pub fn title<S: ToString>(mut self, title: S) -> Self {
        self.title = format!("] {} [", title.to_string());
        self
    }

    pub fn msg<S: ToString>(mut self, prompt: S) -> Self {
        self.prompt = prompt.to_string();
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

    pub fn style(mut self, style: MsgBoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn done(mut self, func: Box<MsgBoxResultFn>) -> Self {
        self.done = Some(func);
        self
    }

    pub fn build(mut self) -> Box<MsgBox> {
        if self.done.is_none() {
            let id = self.id.clone();
            self.done = Some(Box::new(
                move |app: &mut AppContext, data: Option<MsgData>| {
                    app.send_message(id.as_ref(), data)
                },
            ));
        }

        Box::new(MsgBox::new(self))
    }
}

pub struct MsgBox {
    config: MsgBoxBuilder,
    ui: UI,
}

impl MsgBox {
    pub fn builder(id: &str) -> MsgBoxBuilder {
        MsgBoxBuilder::new(id)
    }

    fn new(config: MsgBoxBuilder) -> Self {
        let ui = dialog(config.page_size, config.font.as_str(), |dlg| {
            dlg.class("msg_box")
                .class(&config.class)
                .bind_key(
                    VirtualKeyCode::Return,
                    UiAction::message("OK".to_owned(), None),
                )
                .bind_key(
                    VirtualKeyCode::Escape,
                    UiAction::message("CANCEL".to_owned(), None),
                );

            Frame::new(dlg, |frame| {
                frame.class("msg_box").class(&config.class).margin(1).pad(1);

                if config.title.len() > 0 {
                    frame.title(&config.title);
                }

                Text::new(frame, |txt| {
                    txt.text(&config.prompt).pad_bottom(1);
                    txt.class("msg_box").class(&config.class);
                });

                Span::new(frame, |span| {
                    span.pad_top(1).anchor(Align::Max).spacing(2);

                    let (cancel_text, ok_text) = match config.style {
                        MsgBoxStyle::YesNo => ("[  No  ]", "[  Yes ]"),
                        _ => ("[Cancel]", "[  Ok  ]"),
                    };

                    if config.style != MsgBoxStyle::Ok {
                        Button::new(span, |cancel| {
                            cancel
                                .id("CANCEL")
                                .text(cancel_text)
                                .width(8)
                                .class("msg_box")
                                .class("cancel")
                                .class(&config.class);
                        });
                    }

                    Button::new(span, |ok| {
                        ok.id("OK")
                            .text(ok_text)
                            .width(8)
                            .class("msg_box")
                            .class("ok")
                            .class(&config.class);
                    });
                });
            });
        });

        ui.dump();

        MsgBox { ui, config }
    }
}

#[allow(unused_variables)]
impl Screen for MsgBox {
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
                println!("MsgBox - {}, ok", &self.config.id);
                if let Some(done) = self.config.done.take() {
                    done(app, Some(true.into()));
                }

                ScreenResult::Pop
            }
            "CANCEL" => {
                println!("MsgBox - {}, cancel", &self.config.id);
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
