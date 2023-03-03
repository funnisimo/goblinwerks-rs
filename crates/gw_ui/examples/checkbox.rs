use gw_app::*;
use gw_ui::css::*;
use gw_ui::ui::*;

struct MainScreen {
    ui: UI,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Align + Anchor")
                    .pos(5, 5)
                    .width(28)
                    .spacing(1);

                Checkbox::new(frame, |chk| {
                    chk.id("LEFT").text("Align+Anchor Left");
                });

                Checkbox::new(frame, |chk| {
                    chk.id("RIGHT")
                        .text("Align+Anchor Right")
                        .align(Align::RIGHT)
                        .anchor(Align::RIGHT);
                });

                Checkbox::new(frame, |chk| {
                    chk.id("CENTER")
                        .text("Align+Anchor Center")
                        .align(Align::CENTER)
                        .anchor(Align::CENTER);
                });
            });
            Frame::new(body, |frame| {
                frame.margin(1).width(28).pos(5, 13);

                Text::new(frame, |txt| {
                    txt.id("ALIGN_MSG").text("").width(24);
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Glyphs")
                    .pos(40, 5)
                    .width(28)
                    .spacing(1)
                    .id("GLYPHS");

                Checkbox::new(frame, |chk| {
                    chk.id("NORMAL").text("Default Glyphs");
                });

                Checkbox::new(frame, |chk| {
                    chk.id("CUSTOM").text("Custom Glyph").glyphs("_", "!"); // No padding between glyph and text
                });

                Checkbox::new(frame, |chk| {
                    chk.id("LARGER").text("Larger Glyphs").glyphs("[ ]", "[X]");
                    // Notice the space for padding
                });

                Checkbox::new(frame, |chk| {
                    chk.id("COLORED")
                        .text("Colored Glyphs")
                        .glyphs("{ }", "{#[blue]X#[]}"); // Can use color notation
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Wrapping")
                    .pos(5, 25)
                    .width(28)
                    .spacing(1);

                Checkbox::new(frame, |chk| {
                    chk.id("ONE").text("Single line of text");
                });

                Checkbox::new(frame, |chk| {
                    chk.id("TRUNC").nowrap().text("This should be a single line of text that is not wrapped, but is instead truncated.");
                });

                Checkbox::new(frame, |chk| {
                    chk.id("WRAP").text("This is a multi-line text that is word wrapped at the width it is configured with.");
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("Count")
                    .pos(40, 25)
                    .width(28)
                    .spacing(1);

                Checkbox::new(frame, |chk| {
                    chk.id("COUNT").text("Max 10").count("#", 10);
                });

                Checkbox::new(frame, |chk| {
                    chk.id("COUNT_20")
                        .glyphs("[  ]", "[XX]")
                        .count("[##]", 30)
                        .text("Max 30.");
                });
            });
        });

        ui.dump();

        Box::new(MainScreen { ui })
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, _ecs: &mut Ecs) {
        self.ui.update_styles();
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.ui.input(app, ev) {
            return result;
        }
        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: String, _value: Option<MsgData>) -> ScreenResult {
        log(format!("message - {}", id));
        match id.as_str() {
            "LEFT" | "RIGHT" | "CENTER" => {
                self.ui
                    .find_by_id("ALIGN_MSG")
                    .unwrap()
                    .set_text(&format!("{} clicked", id));
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.ui.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Checkboxes Example")
        .file(
            "resources/styles.css",
            Box::new(|path: &str, data: Vec<u8>, app: &mut Ecs| {
                let r = load_stylesheet_data(path, data, app);
                if r.is_ok() {
                    STYLES.lock().unwrap().dump();
                }
                r
            }),
        )
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
