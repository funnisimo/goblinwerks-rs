use conapp::*;

fn main() {
    // create the game window (native) or canvas (web)
    let app = App::new(AppConfig {
        size: (800, 600),
        title: "my game".to_owned(),
        ..AppConfig::default()
    });

    // start game loop
    app.run(move |app: &mut App| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            // exit on key or mouse press
            match evt {
                &AppEvent::KeyUp(_) => {
                    App::exit();
                }
                &AppEvent::MousePos(ref pos) => {
                    let res = app.viewport_size();

                    let pct_x = pos.0 as f32 / res.0 as f32;
                    let pct_y = pos.1 as f32 / res.1 as f32;
                    console(format!("{:?} / {:?} = {:.3},{:.3}", pos, res, pct_x, pct_y));
                }
                &AppEvent::MouseUp(_) => {
                    App::exit();
                }
                _ => (),
            }
        }
    });
}
