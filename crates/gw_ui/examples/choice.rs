use gw_app::color::init_colors;
use gw_app::messages::Messages;
use gw_app::*;
use gw_ui::css::*;
use gw_ui::{
    screens::{Choice, MultiChoice},
    ui::*,
};
use std::collections::HashMap;

fn map_as_text(data: Option<HashMap<Key, MsgData>>) -> String {
    match data {
        None => "Cancelled".to_owned(),
        Some(map) => map
            .into_iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    }
}

fn data_as_text(data: Option<MsgData>) -> String {
    match data {
        None => "Cancelled.".to_owned(),
        Some(MsgData::Text(val)) => val,
        Some(MsgData::List(val)) => val
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        Some(MsgData::Map(data)) => data
            .into_iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        _ => "Unknown.".to_owned(),
    }
}

struct MainScreen {
    ui: UI,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let ui = page((80, 50), "DEFAULT", |body| {
            Frame::new(body, |frame| {
                frame.margin(1).title("Single Select").pos(5, 5);

                Button::new(frame, |btn| {
                    btn.id("MESSAGE")
                        .text("[ Show Message Based Choice ]")
                        .activate_key(VirtualKeyCode::Return);
                });

                Button::new(frame, |btn| {
                    btn.id("CLOSURE")
                        .text("[ Show Closure Based Choice ]")
                        .pad_top(1)
                        .activate_key(VirtualKeyCode::Return)
                        .activate(Box::new(|_: &Element, _: &Element| {
                            println!("Activate closure - Single");
                            Some(UiAction::Screen(Choice::builder("ANYTHING") // Id can be anything because we send the message directly
                                .items(vec!["Football", "Soccer", "Rugby", "Cricket"])
                                .class("blue-back")
                                .done(Box::new(move |app: &mut Ecs, data: Option<MsgData>| {
                                    let mut msgs = app.resources.get_mut::<Messages>().unwrap();
                                    msgs.push("SINGLE", data) // This is what the default implementation does
                                }))
                                .build()))
                        }));
                });

                Text::new(frame, |txt| {
                    txt.id("TEXT")
                        .width(20)
                        .text("Nothing.")
                        .pad_top(1)
                        .height(2);
                });
            });

            Frame::new(body, |frame| {
                frame.margin(1).title("Multi Select").pos(40, 5);

                Button::new(frame, |btn| {
                    btn.id("MESSAGE_MULTI")
                        .text("[ Show Message Based Choice ]")
                        .activate_key(VirtualKeyCode::Return);
                });

                Button::new(frame, |btn| {
                    btn.id("CLOSURE_MULTI")
                        .text("[ Show Closure Based Choice ]")
                        .pad_top(1)
                        .activate_key(VirtualKeyCode::Return)
                        .activate(Box::new(|root: &Element, _: &Element| {
                            println!("Activate closure");
                            let ui_root = root.clone();
                            Some(UiAction::Screen(MultiChoice::builder("ANYTHING") // Id can be anything because we send the message directly
                                .items(vec!["Darts", "Field Hockey", "Biathalon", "Luge"])
                                .class("blue-back")
                                .done(Box::new(move |_: &mut Ecs, data: Option<HashMap<Key,MsgData>>| {
                                    ui_root.find_by_id("TEXT_MULTI").unwrap().set_text(&map_as_text(data));
                                }))
                                .build()))
                        }));
                });

                Text::new(frame, |txt| {
                    txt.id("TEXT_MULTI")
                        .width(20)
                        .text("Nothing.")
                        .pad_top(1)
                        .height(2);
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
            println!("- input={:?}", result);
            return result;
        }
        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: String, value: Option<MsgData>) -> ScreenResult {
        match id.as_str() {
            "MESSAGE" => {
                return ScreenResult::Push(
                    Choice::builder("SINGLE")
                        .items(vec!["Sandwich", "Kebabs", "Sushi", "Taco"])
                        .class("blue-back")
                        .build(),
                );
            }
            "SINGLE" => {
                self.ui
                    .find_by_id("TEXT")
                    .unwrap()
                    .set_text(&data_as_text(value));
            }
            "MESSAGE_MULTI" => {
                return ScreenResult::Push(
                    MultiChoice::builder("MULTI") // Id can be anything because we send the message directly
                        .items(vec!["Football", "Soccer", "Rugby", "Cricket"])
                        .class("blue-back")
                        .build(),
                );
            }
            "MULTI" => {
                self.ui
                    .find_by_id("TEXT_MULTI")
                    .unwrap()
                    .set_text(&data_as_text(value));
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
    init_colors();

    let app = AppBuilder::new(1024, 768)
        .title("Choices Example")
        .file("resources/styles.css", Box::new(load_stylesheet_data))
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}
