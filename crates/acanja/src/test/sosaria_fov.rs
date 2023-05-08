use gw_app::{color::init_colors, Ecs};
use gw_ecs::Builder;
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

    let mut ecs = Ecs::empty();

    {
        let tiles = load_tiles_file("assets/tiles.jsonc");
        ecs.insert_global(tiles);
        let actor_kinds = load_being_kinds_file("assets/actors.jsonc");
        ecs.insert_global(actor_kinds);
        let level = load_level_file(&mut ecs, "assets/maps/sosaria.jsonc");

        let map = level.read_resource::<Map>();
        assert!(FovSource::is_opaque(&*map, 9, 26));
        assert!(FovSource::is_opaque(&*map, 9, 27));
        assert!(FovSource::is_opaque(&*map, 10, 27));
    }

    let entity = ecs
        .create_entity()
        .with(Position::new(10, 26))
        .with(Being::new("HERO".to_string()))
        .build();

    let mask = get_fov_mask(ecs.current_world(), entity, 11);

    assert!(mask.in_fov(10, 26));
    assert!(mask.in_fov(10, 27));
    assert!(mask.in_fov(9, 27));
    assert!(!mask.in_fov(8, 27));

    println!("{:?}", mask);
    assert!(false);
}
