// use enum_utils;
// use std::convert::From;
// use std::str::FromStr;

// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, enum_utils::FromStr, Eq, Hash, Default, Clone, Copy)]
// #[enumeration(case_insensitive)]
// pub enum TileAction {
//     #[default]
//     DEFAULT, // for unangband
//     OPEN,
//     CLOSE,
//     BASH,
//     SPIKE,
//     DISARM,
//     CLIMB, // enter shop too
//     DESCEND,
//     TUNNEL,
//     PUSH,   // push the tile around the room (should this be an item instead)
//     ENTER,  // for trap springing, messages, ...
//     EXIT,   // when someone leaves tile
//     DROP,   // item placed
//     PICKUP, // item pickup
//     DISCOVER,
//     DIG,   // Is this the same as tunnel?
//     SOLID, // liquid becomes solid
//     USE,
//     TIMED, // HOW TO DO THIS?

//     HURT_ROCK,
//     HURT_FIRE,
//     HURT_ACID,
//     HURT_COLD,
//     HURT_POIS,
//     HURT_ELEC,
//     HURT_WATER,
//     HURT_STEAM,

//     // Unangband - What are these for?
//     CHASM,
//     TREE,
//     INNER,
//     OUTER,
//     PATH, // becomes part of path?
// }

// impl From<&str> for TileAction {
//     fn from(s: &str) -> Self {
//         match Self::from_str(s) {
//             Ok(flag) => flag,
//             Err(_) => panic!("Invalid TileAction: {}", s),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use crate::prelude::*;

//     #[test]
//     fn from_str() {
//         assert_eq!(TileAction::OPEN, "OPEN".into());
//         assert_eq!(TileAction::CLOSE, "close".into());
//         assert_eq!(TileAction::PUSH, "Push".parse().unwrap());
//     }

//     #[test]
//     fn to_string() {
//         assert_eq!(format!("{:?}", TileAction::OPEN), "OPEN");
//     }
// }
