use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct TileMove: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const BLOCKS_MOVE = fl!(0);
        const BLOCKS_FLY = fl!(1);
        const CAN_DIG = fl!(2);
        const CAN_SWIM = fl!(3);
        const CAN_OOZE = fl!(4);
        const CAN_CLIMB = fl!(5);
        const CAN_PASS = fl!(6);

        const HIDE_ITEM = fl!(7);
        const HIDE_DIG = fl!(8);
        const HIDE_SWIM = fl!(9);
        const HIDE_SNEAK = fl!(10);

        const BLOCKS_VISION = fl!(11);
        const BLOCKS_MISSILES = fl!(12);
        const BLOCKS_DROP = fl!(13); // Blocks items essentially
        const BLOCKS_PICKUP = fl!(14);
        const BLOCKS_MONS = fl!(15); // especially auto generation
        const BLOCKS_RUN = fl!(16);
        const BLOCKS_DIAGONAL = fl!(17);

        const EASY_CLIMB = fl!(18);
        const EASY_HIDE = fl!(19);

        const SHALLOW = fl!(24);
        const DEEP = fl!(25);
        const VERY_DEEP = fl!(26);   // FILLED from unangband

        const SLOW = fl!(27);   // generally movement is slower
        const FAST = fl!(28);   // generally movement is faster

        const MUST_CLIMB = fl!(29);

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const BLOCKS_ALL = Self::BLOCKS_MOVE.bits | Self::BLOCKS_FLY.bits | Self::BLOCKS_VISION.bits | Self::BLOCKS_MISSILES.bits | Self::BLOCKS_DROP.bits | Self::BLOCKS_PICKUP.bits | Self::BLOCKS_MONS.bits | Self::BLOCKS_RUN.bits | Self::BLOCKS_DIAGONAL.bits;
    }
}

impl TileMove {
    pub fn apply(&mut self, flags: &str) {
        for val in flags.split("|") {
            if val.trim().starts_with("!") {
                match Self::from_str(&val[1..]) {
                    Ok(flag) => self.remove(flag),
                    Err(_) => {}
                }
            } else {
                match Self::from_str(val) {
                    Ok(flag) => self.insert(flag),
                    Err(_) => {}
                }
            }
        }
    }
}

impl fmt::Display for TileMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TileMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = TileMove::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "BLOCKS_MOVE" => result |= TileMove::BLOCKS_MOVE,
                "BLOCKS_FLY" => result |= TileMove::BLOCKS_FLY,
                "CAN_DIG" => result |= TileMove::CAN_DIG,
                "CAN_SWIM" => result |= TileMove::CAN_SWIM,
                "CAN_OOZE" => result |= TileMove::CAN_OOZE,
                "CAN_CLIMB" => result |= TileMove::CAN_CLIMB,
                "CAN_PASS" => result |= TileMove::CAN_PASS,

                "HIDE_ITEM" => result |= TileMove::HIDE_ITEM,
                "HIDE_DIG" => result |= TileMove::HIDE_DIG,
                "HIDE_SWIM" => result |= TileMove::HIDE_SWIM,
                "HIDE_SNEAK" => result |= TileMove::HIDE_SNEAK,

                "BLOCKS_VISION" => result |= TileMove::BLOCKS_VISION,
                "BLOCKS_MISSILES" => result |= TileMove::BLOCKS_MISSILES,
                "BLOCKS_DROP" => result |= TileMove::BLOCKS_DROP, // Blocks items essentially
                "BLOCKS_PICKUP" => result |= TileMove::BLOCKS_PICKUP,
                "BLOCKS_MONS" => result |= TileMove::BLOCKS_MONS, // especially auto generation
                "BLOCKS_RUN" => result |= TileMove::BLOCKS_RUN,

                "EASY_CLIMB" => result |= TileMove::EASY_CLIMB,
                "EASY_HIDE" => result |= TileMove::EASY_HIDE,

                "SHALLOW" => result |= TileMove::SHALLOW,
                "DEEP" => result |= TileMove::DEEP,
                "VERY_DEEP" => result |= TileMove::VERY_DEEP, // FILLED from unangband

                "SLOW" => result |= TileMove::SLOW, // generally movement is slower
                "FAST" => result |= TileMove::FAST, // generally movement is faster

                "MUST_CLIMB" => result |= TileMove::MUST_CLIMB,

                "BLOCKS_ALL" => result |= TileMove::BLOCKS_ALL,
                "" => {}
                _ => return Err(format!("Unknown TileFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for TileMove {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn from_str() {
        assert_eq!(TileMove::BLOCKS_MOVE, "BLOCKS_MOVE".into());
        assert_eq!(TileMove::CAN_DIG, "can_dig".into());
        assert_eq!(TileMove::MUST_CLIMB, "Must_Climb".parse().unwrap());

        assert_eq!(
            TileMove::BLOCKS_FLY | TileMove::HIDE_ITEM,
            "blocks_fly | hide_item".parse().unwrap()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", TileMove::VERY_DEEP), "VERY_DEEP");

        let flags = TileMove::DEEP | TileMove::CAN_OOZE;
        assert_eq!(format!("{:?}", flags), "CAN_OOZE | DEEP");
    }
}
