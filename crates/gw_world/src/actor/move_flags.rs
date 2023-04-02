use bitflags::bitflags;
use gw_util::fl;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct MoveFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!
        // const HERO = fl!(0);

        const NO_MOVE = fl!(0);
        const RAND25 = fl!(1);
        const RAND50 = fl!(2);
        const RAND100 = fl!(3);

        const MOVE_FAST = fl!(4);
        const MOVE_SLOW = fl!(5);

        const CAN_DIG = fl!(6);      /* Monster can dig */
        const CAN_SWIM = fl!(7);      /* Monster can swim */
        const MUST_SWIM = fl!(8);      /* Monster must swim */
        const CAN_CLIMB = fl!(9);      /* Monster can climb walls */
        const CAN_FLY = fl!(10);      /* Monster can fly */
        const MUST_FLY = fl!(11);      /* Monster must fly */


        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl FromStr for MoveFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = MoveFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                // "EXAMPLE" => result |= MoveFlags::EXAMPLE,
                "NO_MOVE" => result |= MoveFlags::NO_MOVE,
                "RAND25" => result |= MoveFlags::RAND25,
                "RAND50" => result |= MoveFlags::RAND50,
                "RAND100" => result |= MoveFlags::RAND100,

                "MOVE_FAST" => result |= MoveFlags::MOVE_FAST,
                "MOVE_SLOW" => result |= MoveFlags::MOVE_SLOW,

                "CAN_DIG" => result |= MoveFlags::CAN_DIG,
                "CAN_SWIM" => result |= MoveFlags::CAN_SWIM,
                "MUST_SWIM" => result |= MoveFlags::MUST_SWIM,
                "CAN_CLIMB" => result |= MoveFlags::CAN_CLIMB,
                "CAN_FLY" => result |= MoveFlags::CAN_FLY,
                "MUST_FLY" => result |= MoveFlags::MUST_FLY,

                "" => {}
                _ => return Err(format!("Unknown MoveFlags: {}", s)),
            }
        }
        Ok(result)
    }
}

impl MoveFlags {
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

impl From<&str> for MoveFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

impl fmt::Display for MoveFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
