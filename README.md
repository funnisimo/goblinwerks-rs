## conapp

Provides a simple application, screen, console system for games, mostly Roguelikes.

There are a number of examples in the examples folder.

Here is the code from the `minimal` example:

```rs
use conapp::*;

struct MainScreen {
    con: Console,
}

impl ScreenCreator for MainScreen {
    fn create(app: &mut AppContext) -> Box<dyn Screen> {
        let con = app
            .simple_console(80, 50, "resources/terminal_8x8.png")
            .expect("Failed to load font");
        Box::new(MainScreen { con })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, _app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(_) => ScreenResult::Quit,
            AppEvent::MouseDown(_) => ScreenResult::Quit,
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.fill(Some('.' as u32), Some(WHITE), Some(BLACK));
        draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .print(10, 10, "Hello World");
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768).title("Minimal Example").build();
    app.run::<MainScreen>();
}
```
