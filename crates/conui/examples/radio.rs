use conapp::*;
use conui::css::*;
use conui::ui::*;

struct MainScreen {
    ui: UI,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame.margin(1).title("Basic").pos(5, 5).width(28);

                Radio::new(frame, |radio| {
                    radio
                        .id("PICK")
                        .item("Choice A")
                        .item("Choice B")
                        .item("Choice C");
                });
            });
            Frame::new(body, |frame| {
                frame.margin(1).width(28).pos(5, 11);

                Text::new(frame, |txt| {
                    txt.id("PICK_MSG").text("").width(24);
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("Glyphs").pos(40, 5).width(28);

                Radio::new(frame, |radio| {
                    radio
                        .id("GLYPH")
                        .glyphs(" ", "+")
                        .item("Choice A")
                        .with_item("Choice B", |item| {
                            item.glyphs("~", "!");
                        })
                        .item("Choice C");
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("Wrapping").pos(5, 25).width(28);

                Radio::new(frame, |radio| {
                    radio
                        .id("WRAP")
                        .item("This is a long radio element and the text will be wrapped.")
                        .with_item("This will be truncated even though it is longer.", |item| {
                            item.nowrap();
                        })
                        .item("Just a simple item.");
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Space and Spacing")
                    .pos(40, 25)
                    .width(28);

                Radio::new(frame, |radio| {
                    radio
                        .id("SPACE")
                        .spacing(1)
                        .space(0)
                        .item("Choice A")
                        .item("Choice B")
                        .with_item("Choice C", |item| {
                            item.space(2);
                        });
                });
            });
        });

        ui.dump();

        Box::new(MainScreen { ui })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
            return result;
        }
        ScreenResult::Continue
    }

    fn message(
        &mut self,
        _app: &mut AppContext,
        id: String,
        value: Option<MsgData>,
    ) -> ScreenResult {
        log(format!("message - {}", id));
        match id.as_str() {
            "PICK" => {
                self.ui
                    .find_by_id("PICK_MSG")
                    .unwrap()
                    .set_text(&format!("'{}' clicked", value.unwrap().to_string()));
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        self.ui.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Checkboxes Example")
        .file(
            "resources/styles.css",
            Box::new(|data: Vec<u8>, app: &mut AppContext| {
                let r = load_stylesheet_data(data, app);
                if r.is_ok() {
                    STYLES.lock().unwrap().dump();
                }
                r
            }),
        )
        .vsync(false)
        .build();

    app.run_with(Box::new(|_: &mut AppContext| MainScreen::new()));
}
