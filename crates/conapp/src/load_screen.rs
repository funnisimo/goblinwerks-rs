use crate::{color::RGBA, console, draw};
use crate::{AppContext, Console, Screen, ScreenResult};

pub type ScreenCreator = dyn FnOnce(&mut AppContext) -> Box<dyn Screen>;

pub struct LoadingScreen {
    con: Console,
    alpha: u8,
    next: Option<Box<ScreenCreator>>,
}
impl LoadingScreen {
    pub fn new(next: Box<ScreenCreator>) -> Box<Self> {
        let con = Console::new(40, 30, "DEFAULT");
        Box::new(LoadingScreen {
            con,
            alpha: 255,
            next: Some(next),
        })
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, app: &mut AppContext, ms: f64) -> ScreenResult {
        if !app.has_files_to_load() {
            match self.next.take() {
                None => {
                    console("Pop Loading Screen");
                    return ScreenResult::Pop;
                }
                Some(next) => {
                    console("Replace loading screen");
                    return ScreenResult::Replace(next(app));
                }
            }
        }

        if self.alpha == 0 {
            self.alpha = 255;
        } else {
            self.alpha = self.alpha.saturating_sub(ms.floor() as u8);
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        let buf = self.con.buffer_mut();
        buf.clear(true, true, true);

        let fg = RGBA::rgba(255, 255, 255, self.alpha);

        draw::plain(buf)
            .align(draw::TextAlign::Center)
            .fg(fg)
            .print(20, 15, "Loading...");

        self.con.render(app);
    }
}
