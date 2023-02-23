use conapp::console::subcell_console;
use conapp::*;
use std::rc::Rc;

// doryen-rs/examples/subcell

// This example does NOT do subcell rendering
// subcell rendering is not supported by codepate437 natively
// you must implement a font that has subcell chars yourself

const FONT: &str = "resources/terminal_8x8.png";
const SKULL: &str = "resources/skull.png";

struct MyRoguelike {
    con: Console,
    subcell: Console,
    skull: Option<Rc<Image>>,
}

impl MyRoguelike {
    fn new() -> Box<dyn Screen> {
        let con = Console::new(60, 80, FONT);

        Box::new(MyRoguelike {
            con,
            subcell: subcell_console(30, 40).with_extents(0.25, 0.25, 0.75, 0.75),
            skull: None,
        })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut AppContext) {
        // text
        let buffer = self.con.buffer_mut();
        buffer.fill(
            Some('.' as u32),
            Some(RGBA::rgba(255, 0, 255, 255)),
            Some(RGBA::rgb(32, 32, 32)),
        );
        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg(RGBA::rgb(0, 128, 255))
            .print_lines(30, 10, "This is a 60x80 png\non a 30x40 console.");

        self.con.render(app);

        if let Some(ref skull) = self.skull {
            // image
            self.subcell.buffer_mut().clear(true, true, true);
            draw::subcell(self.subcell.buffer_mut())
                .transparent(RGBA::rgba(0, 0, 0, 255))
                .blit(skull, 0, 0, 0, 0, None, None);
            self.subcell.render(app);
        } else {
            self.skull = app.get_image(SKULL);
        }
    }
}

fn main() {
    let app = AppBuilder::new(768, 1024)
        .title("SubCell Resolution Example")
        .font(FONT)
        .image(SKULL)
        .build();
    app.run_screen(MyRoguelike::new());
}
