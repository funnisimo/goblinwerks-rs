use gw_app::color::init_colors;
use gw_world::being::load_being_kinds_file;

const FILE: &str = "assets/beings.jsonc";

fn main() {
    init_colors();

    let kinds = load_being_kinds_file(FILE);
    kinds.dump();

    let kind = kinds.get("PEASANT").unwrap();
    println!("Peasant Being Clone: {:?}", kind.being.clone());
}
