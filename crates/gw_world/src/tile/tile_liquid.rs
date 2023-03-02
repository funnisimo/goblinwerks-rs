use enum_utils;
use std::convert::From;
use std::str::FromStr;

/// These are the basic tile types that every tile has to fall into.
/// They are mostly used by map generation algos.
#[derive(Debug, PartialEq, enum_utils::FromStr, Default)]
#[enumeration(case_insensitive)]
pub enum TileLiquid {
    #[default]
    NONE,
    WATER,
    MUD,
    LAVA,
    ICE,
    ACID,
    OIL,
    CHASM,
}

impl From<&str> for TileLiquid {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(_) => panic!("Unkown TileLiquid: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn from_str() {
        assert_eq!(TileLiquid::NONE, "NONE".into());
        assert_eq!(TileLiquid::WATER, "water".into());
        assert_eq!(TileLiquid::LAVA, "Lava".parse().unwrap());
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", TileLiquid::ICE), "ICE");
    }
}
