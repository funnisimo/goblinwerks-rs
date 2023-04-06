use gw_app::{color::init_colors, Ecs};
use gw_world::{
    being::{load_being_kinds_file, Being},
    effect::register_effect_parser,
    fov::{get_fov_mask, FovSource},
    map::Map,
    position::Position,
    tile::load_tiles_file,
};

use crate::{
    effect::{parse_gremlins, parse_mark, parse_winds},
    loader::load_level_file,
};

#[test]
fn fov_10_26() {
    init_colors();

    register_effect_parser("winds", parse_winds);
    register_effect_parser("gremlins", parse_gremlins);
    register_effect_parser("mark", parse_mark);

    let tiles = load_tiles_file("assets/tiles.jsonc");
    let actor_kinds = load_being_kinds_file("assets/actors.jsonc");
    let mut level = load_level_file("assets/maps/sosaria.jsonc", &tiles, &actor_kinds);

    {
        let map = level.resources.get::<Map>().unwrap();
        assert!(FovSource::is_opaque(&*map, 9, 26));
        assert!(FovSource::is_opaque(&*map, 9, 27));
        assert!(FovSource::is_opaque(&*map, 10, 27));
    }

    let entity = level
        .world
        .push((Position::new(10, 26), Being::new("HERO".to_string())));

    let mut ecs = Ecs::new();
    ecs.resources.insert(tiles);
    ecs.resources.insert(actor_kinds);
    ecs.resources.insert(level);

    let mask = get_fov_mask(&ecs, entity, 11);

    assert!(mask.in_fov(10, 26));
    assert!(mask.in_fov(10, 27));
    assert!(mask.in_fov(9, 27));
    assert!(!mask.in_fov(8, 27));

    println!("{:?}", mask);
    assert!(false);
}
