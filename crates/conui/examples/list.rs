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
                frame.margin(1).title("] Basic [").pos(5, 5).width(28);

                List::new(frame, |list| {
                    list.item("Testing lists");
                    list.with_item("Created with func", |_item| {});
                    list.item("More Testing");
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("] Sub-Lists [").pos(40, 5).width(28);

                List::new(frame, |list| {
                    list.with_item("A main point", |item| {
                        item.id("ITEM_1");
                        List::new(item, |sub| {
                            sub.item("Sub Item A").item("Sub Item B").item("Sub item C");
                        });
                    });
                    list.with_item("More Information", |sub| {
                        List::new(sub, |sublist| {
                            sublist
                                .item("Sub Item A")
                                .with_item("Sub Item B!", |subi| {
                                    subi.sublist(|subl| {
                                        subl.item("Weird").item("Wacky!");
                                    });
                                })
                                .item("Sub item C");
                        });
                    });
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("] Glyphs [").pos(40, 25).width(28);

                List::new(frame, |list| {
                    list.glyph("*");
                    list.item("Testing lists");
                    list.with_item("Created with func", |item| {
                        item.glyph(">");
                    });
                    list.item("More Testing");
                });
            });

            Frame::new(body, |frame| {
                frame
                    .margin(1)
                    .title("] Pad and Space [")
                    .pos(5, 15)
                    .width(28);

                List::new(frame, |list| {
                    list.with_item("Item with Padded sublist", |item| {
                        item.id("ITEM_1");
                        List::new(item, |sub| {
                            sub.pad_top(1).pad_bottom(1);
                            sub.item("Sub Item A").item("Sub Item B").item("Sub item C");
                        });
                    });
                    list.with_item("Item with Spaced sublist", |sub| {
                        List::new(sub, |sublist| {
                            sublist.spacing(1);
                            sublist
                                .item("Sub Item A")
                                .item("Subitem B")
                                .item("Sub item C");
                        });
                    });
                    list.with_item("Item with spaced and padded sublist", |item| {
                        item.sublist(|subl| {
                            subl.spacing(1).pad_top(1); // do not pad_bottom
                            subl.item("Weird").item("Wacky!");
                        });
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
        _value: Option<MsgData>,
    ) -> ScreenResult {
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

    fn render(&mut self, app: &mut AppContext) {
        self.ui.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("List Example")
        .file("resources/styles.css", Box::new(load_stylesheet_data))
        .vsync(false)
        .build();

    app.run_with(Box::new(|_: &mut AppContext| MainScreen::new()));
}
