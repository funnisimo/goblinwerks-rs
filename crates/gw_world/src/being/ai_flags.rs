use bitflags::bitflags;
use gw_util::fl;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

////////////////////////////////////////////
// AI Flags

bitflags! {
    #[derive(Default, Deserialize, Serialize)]
    pub struct AIFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const AIMLESS_MOVE = fl!(0);    // Will move around, but not too much
        const WANDER = fl!(1);          // Will pick a point on the map periodically to go and visit

        const STUPID = fl!(3);      /* Monster is stupid */
        const SMART = fl!(4);      /* Monster is smart */
        const TOWNSFOLK = fl!(5);      /* Use townsfolk AI */

        const NEVER_BLOW = fl!(6);      /* Never make physical blow */
        const NEVER_MISS = fl!(7);      /* Never miss when attacking */
        const EVASIVE = fl!(8);      /* Evade melee blows / missiles / bolts */
        const SCENT = fl!(9);      /* Track player by scent */
        const SUPER_SCENT = fl!(10);      /* Track player by scent - better range */
        const WATER_SCENT = fl!(11);      /* Track player by scent through water */

        const LOW_MANA_RUN = fl!(12);	/* Monster will run if low on mana */
        const TAKE_HIT_RUN = fl!(13);	/* Monster will run if it takes any damage */

        const OPEN_DOOR = fl!(14);      /* Monster can open doors */
        const BASH_DOOR = fl!(15);      /* Monster can bash doors */
        const PASS_WALL = fl!(16);      /* Monster can pass walls */
        const KILL_WALL = fl!(17);      /* Monster can destroy walls */
        const ARCHER = fl!(18);      /* Monster has extra ammo */
        const EAT_BODY = fl!(19);      /* Monster can eat body parts */
        const TAKE_ITEM = fl!(20);      /* Monster can pick up items */

        const OOZE = fl!(27);      /* Oozes through things */
        const HUGE = fl!(28);      /* Huge (breaks things?) */
        const NONVOCAL = fl!(29);      /* Non-Vocal */
        const NONLIVING = fl!(30);      /* Non-Living */

    }
}

impl AIFlags {
    pub fn apply(&mut self, flags: &str) {
        for val in flags.split("|") {
            if val.trim().starts_with("!") {
                match AIFlags::from_str(&val[1..]) {
                    Ok(flag) => self.remove(flag),
                    Err(_) => {}
                }
            } else {
                match AIFlags::from_str(val) {
                    Ok(flag) => self.insert(flag),
                    Err(_) => {}
                }
            }
        }
    }
}

impl FromStr for AIFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = AIFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "AIMLESS_MOVE" => result |= AIFlags::AIMLESS_MOVE,
                "WANDER" => result |= AIFlags::WANDER,

                "NEVER_BLOW" => result |= AIFlags::NEVER_BLOW, /* Never make physical blow */
                "STUPID" => result |= AIFlags::STUPID,         /* Monster is stupid */
                "SMART" => result |= AIFlags::SMART,           /* Monster is smart */

                "NEVER_MISS" => result |= AIFlags::NEVER_MISS, /* Never miss when attacking */
                "EVASIVE" => result |= AIFlags::EVASIVE, /* Evade melee blows / missiles / bolts */
                "SCENT" => result |= AIFlags::SCENT,     /* Track player by scent */
                "SUPER_SCENT" => result |= AIFlags::SUPER_SCENT, /* Track player by scent - better range */
                "WATER_SCENT" => result |= AIFlags::WATER_SCENT, /* Track player by scent through water */
                "TOWNSFOLK" => result |= AIFlags::TOWNSFOLK,     /* Use townsfolk AI */

                "LOW_MANA_RUN" => result |= AIFlags::LOW_MANA_RUN, /* Monster will run if low on mana */
                "TAKE_HIT_RUN" => result |= AIFlags::TAKE_HIT_RUN, /* Monster will run if it takes damage */

                "OPEN_DOOR" => result |= AIFlags::OPEN_DOOR, /* Monster can open doors */
                "BASH_DOOR" => result |= AIFlags::BASH_DOOR, /* Monster can bash doors */
                "PASS_WALL" => result |= AIFlags::PASS_WALL, /* Monster can pass walls */
                "KILL_WALL" => result |= AIFlags::KILL_WALL, /* Monster can destroy walls */
                "ARCHER" => result |= AIFlags::ARCHER,       /* Monster has extra ammo */
                "EAT_BODY" => result |= AIFlags::EAT_BODY,   /* Monster can eat body parts */
                "TAKE_ITEM" => result |= AIFlags::TAKE_ITEM, /* Monster can pick up items */

                "OOZE" => result |= AIFlags::OOZE, /* Oozes through things */
                "HUGE" => result |= AIFlags::HUGE, /* Huge (breaks things?) */
                "NONVOCAL" => result |= AIFlags::NONVOCAL, /* Non-Vocal */
                "NONLIVING" => result |= AIFlags::NONLIVING, /* Non-Living */

                "" => {}
                _ => return Err(format!("Unknown ActorFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for AIFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

impl fmt::Display for AIFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
