use gw_app::img::Images;
use gw_app::panel::subcell_console;
use gw_app::*;
use std::sync::Arc;

// doryen-rs/examples/subcell

// This example does NOT do subcell rendering
// subcell rendering is not supported by codepate437 natively
// you must implement a font that has subcell chars yourself

const FONT: &str = "assets/terminal_8x8.png";
const SKULL: &str = "assets/skull.png";

struct MyRoguelike {
    con: Panel,
    subcell: Panel,
    skull: Option<Arc<Image>>,
}

impl MyRoguelike {
    fn new() -> Box<dyn Screen> {
        let con = Panel::new(60, 80, FONT);

        Box::new(MyRoguelike {
            con,
            subcell: subcell_console(30, 40).with_extents(0.25, 0.25, 0.75, 0.75),
            skull: None,
        })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut Ecs) {
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
            self.skull = app.read_global::<Images>().get(SKULL);
        }
    }
}

fn main() {
    let app = AppBuilder::new(768, 1024)
        .title("SubCell Resolution Example")
        .font_with_transform(FONT, &codepage437::to_glyph, &codepage437::from_glyph)
        .image(SKULL)
        .build();
    app.run(MyRoguelike::new());
}
