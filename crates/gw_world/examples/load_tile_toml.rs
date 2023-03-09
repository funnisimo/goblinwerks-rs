use gw_app::color::init_colors;
use gw_util::toml::parse_file;
use gw_world::tile::{load_tile_data, Tiles};

const FILE: &str = "assets/tiles.toml";

fn main() {
    init_colors();

    let toml = parse_file(FILE).expect("Failed to parse TOML file.");

    let mut tiles = Tiles::default();
    let count = load_tile_data(&mut tiles, toml).expect("Failed to load toml file.");

    println!("Loaded {} tiles.", count);
}
