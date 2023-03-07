use gw_app::color::RGBA;
use gw_app::ecs::WindowInfo;
use gw_app::*;

// doryen-rs/examples/resize

const FONT: &str = "assets/terminal_8x8.png";

struct MyRoguelike {
    con: Panel,
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Panel::new(1024 / 8, 768 / 8, FONT);
        Box::new(MyRoguelike { con })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut Ecs) {
        let buffer = self.con.buffer_mut();
        let con_size = buffer.size();

        draw::frame(buffer)
            .fg(RGBA::rgba(128, 128, 128, 255))
            .bg(RGBA::rgba(0, 0, 0, 255))
            .fill(Some(' ' as u32), None, None)
            .draw(0, 0, con_size.0, con_size.1);

        buffer.area(
            10,
            10,
            5,
            5,
            Some('&' as u32),
            Some(RGBA::rgba(255, 64, 64, 255)),
            Some(RGBA::rgba(128, 32, 32, 255)),
        );

        draw::plain(buffer).align(TextAlign::Center).print(
            (con_size.0 / 2) as i32,
            (con_size.1 / 2) as i32,
            &format!("console: {} x {}", con_size.0, con_size.1),
        );

        let screen_size = app.resources.get::<WindowInfo>().unwrap().size;

        draw::plain(buffer).align(TextAlign::Center).print(
            (con_size.0 / 2) as i32,
            (con_size.1 / 2) as i32 + 2,
            &format!("screen: {} x {}", screen_size.0, screen_size.1),
        );

        let pot_size = buffer.pot_size();

        draw::plain(buffer).align(TextAlign::Center).print(
            (con_size.0 / 2) as i32,
            (con_size.1 / 2) as i32 + 4,
            &format!("pot size: {} x {}", pot_size.0, pot_size.1),
        );

        // buffer.back(
        //     self.mouse_pos.0 as i32,
        //     self.mouse_pos.1 as i32,
        //     RGBA::rgba(255, 255, 255, 255),
        // );

        self.con.render(app)
    }

    fn resize(&mut self, api: &mut Ecs) {
        let info = api.resources.get::<WindowInfo>().unwrap();

        if self.con.ready() {
            let font_char = self.con.font_char_size();
            let width = info.size.0 / font_char.0;
            let height = info.size.1 / font_char.1;
            self.con.resize(width, height);
        }
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Resize Window Example")
        .font_with_transform(FONT, &codepage437::to_glyph, &codepage437::from_glyph)
        .build();
    app.run(MyRoguelike::new());
}
