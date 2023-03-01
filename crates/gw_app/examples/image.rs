use gw_app::color::RGBA;
use gw_app::*;
use std::rc::Rc;

const FONT: &str = "resources/terminal_8x8.png";
const SKULL: &str = "resources/skull.png";

const _WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);

struct MyRoguelike {
    con: Console,
    skull: Option<Rc<Image>>,
    angle: f32,
    scale_time: f32,
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Console::new(80, 50, FONT);

        Box::new(MyRoguelike {
            con,
            skull: None,
            angle: 0.0,
            scale_time: 0.0,
        })
    }
}

impl Screen for MyRoguelike {
    fn update(&mut self, _api: &mut AppContext, _ms: f64) -> ScreenResult {
        self.angle += 0.01;
        self.scale_time += 0.01;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        let buf_size = buffer.size();
        let scale = self.scale_time.cos();
        buffer.fill(None, None, Some((0, 0, 0, 255).into()));

        if let Some(ref img) = self.skull {
            draw::image(buffer).blit_ex(
                (buf_size.0 / 2) as f32,
                (buf_size.1 / 2) as f32,
                scale,
                scale,
                self.angle,
                img,
            );
        } else {
            self.skull = app.get_image(SKULL);
        }

        self.con.render(app)
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Image Example")
        .font(FONT)
        .image(SKULL)
        .vsync(false)
        .build();
    app.run_screen(MyRoguelike::new());
}
