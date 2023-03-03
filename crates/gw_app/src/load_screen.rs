use crate::ecs::Time;
use crate::ecs::{systems::ResourceSet, Read};
use crate::loader::Loader;
use crate::screen::BoxedScreen;
use crate::{color::RGBA, draw, log};
use crate::{Console, Ecs, Screen, ScreenResult};

pub struct LoadingScreen {
    con: Console,
    alpha: u8,
    next: Option<BoxedScreen>,
}
impl LoadingScreen {
    pub fn new(next: BoxedScreen) -> Box<Self> {
        let con = Console::new(20, 10, "DEFAULT");
        Box::new(LoadingScreen {
            con,
            alpha: 255,
            next: Some(next),
        })
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        let (loader, time) = <(Read<Loader>, Read<Time>)>::fetch(&mut ecs.resources);

        let ms = time.delta.floor();

        if !loader.has_files_to_load() {
            match self.next.take() {
                None => {
                    log("Pop Loading Screen");
                    return ScreenResult::Pop;
                }
                Some(next) => {
                    log("Replace loading screen");
                    return ScreenResult::Replace(next);
                }
            }
        }

        if self.alpha == 0 {
            self.alpha = 255;
        } else {
            self.alpha = self.alpha.saturating_sub(ms as u8);
        }
        ScreenResult::Continue
    }

    fn render(&mut self, ecs: &mut Ecs) {
        let buf = self.con.buffer_mut();
        buf.fill(Some(0), None, Some(RGBA::rgb(32, 32, 64)));

        let fg = RGBA::rgba(255, 255, 255, self.alpha);

        draw::plain(buf)
            .align(draw::TextAlign::Center)
            .fg(fg)
            .print(10, 5, "Loading...");

        self.con.render(ecs);
    }
}
