use acanja::effect::{parse_gremlins, parse_mark, parse_winds};
use gw_app::color::init_colors;
use gw_util::json::parse_file;
use gw_world::{
    effect::register_effect_parser,
    tile::{load_tile_data, Tiles},
};

const FILE: &str = "assets/tiles.jsonc";

fn main() {
    init_colors();

    register_effect_parser("winds", parse_winds);
    register_effect_parser("gremlins", parse_gremlins);
    register_effect_parser("mark", parse_mark);

    let json = parse_file(FILE).expect("Failed to parse JSON file.");

    let mut tiles = Tiles::default();
    let count = load_tile_data(&mut tiles, json).expect("Failed to load toml file.");

    println!("Loaded {} tiles.", count);
}
