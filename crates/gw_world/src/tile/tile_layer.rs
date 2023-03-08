use enum_utils;
use std::convert::From;
use std::str::FromStr;

/// These are the basic tile types that every tile has to fall into.
/// They are mostly used by map generation algos.
#[derive(Debug, PartialEq, enum_utils::FromStr, Default)]
#[enumeration(case_insensitive)]
pub enum TileLayer {
    #[default]
    GROUND,
    FEATURE,
    ITEM,
    LIQUID,
    ACTOR,
    GAS,
    FX,
}

impl From<&str> for TileLayer {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(_) => panic!("Unkown TileLayer: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn from_str() {
        assert_eq!(TileLayer::GROUND, "ground".into());
        assert_eq!(TileLayer::LIQUID, "liquid".into());
        assert_eq!(TileLayer::GAS, "Gas".parse().unwrap());
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", TileLayer::ACTOR), "ACTOR");
    }
}
