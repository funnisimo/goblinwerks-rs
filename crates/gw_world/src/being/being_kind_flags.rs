use bitflags::bitflags;
use gw_util::fl;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct BeingKindFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!
        const HERO = fl!(0);
        const UNIQUE = fl!(1);      /* Unique Monster */
        const QUESTOR = fl!(2);      /* Quest Monster */
        const GUARDIAN = fl!(3);      /* Dungeon guardian*/

        const MALE = fl!(4);      /* Can be Male gender */
        const FEMALE = fl!(5);      /* Can be Female gender */
        const OBJECT = fl!(6);      /* Uses "it" pronouns */

        const CAN_BE_REPELLED = fl!(7);
        const CAN_BE_TURNED = fl!(8);
        const WILL_NOT_ATTACK = fl!(9);
        const CANNOT_BE_ATTACKED = fl!(10);

        const FORCE_DEPTH = fl!(11);      /* Start at "correct" depth */
        const FORCE_MAXHP = fl!(12);      /* Start with max hitpoints */
        const FORCE_SLEEP = fl!(13);      /* Start out sleeping */

        // const HAS_LITE = fl!(13);      /* Monster has lite */
        // const INVISIBLE = fl!(14);      /* Monster avoids vision */
        // const COLD_BLOOD = fl!(15);      /* Monster avoids infra */
        // const EMPTY_MIND = fl!(16);      /* Monster avoids telepathy */
        // const WEIRD_MIND = fl!(17);      /* Some (80%) monsters avoid telepathy */
        // const MULTIPLY = fl!(18);      /* Monster reproduces */
        // const REGENERATE = fl!(19);      /* Monster regenerates */
        // const POWERFUL = fl!(20);      /* Monster has strong breath */

        // const TRAIL = fl!(21);      /* Monster leaves a trail behind it */
        // const SNEAKY = fl!(22); 	/* Monster hides a lot of actions */
        // const ARMOR = fl!(23); 	/* Monster is fully armoured (Reduces acid damage/stops some arrows) */

        const PROPER_NAME = fl!(24);
        const IGNORE_WHEN_SEEN = fl!(25);

        // const WARRIOR = fl!(22);
        // const ARCHER = fl!(23);
        // const PRIEST = fl!(24); 	/* Monster has access to priest spells ? */
        // const MAGE = fl!(25); 	/* Monster has access to mage spells ? */

        // const HAS_AURA = fl!(26); 	/* Monster radiates an aura attack */
        // const HAS_WEB = fl!(27); 	/* Monster created in a web */
        // const NEED_LITE = fl!(28); 	/* Monster cannot see the player if player is not visible */

        const EVIL = fl!(29);      /* Evil */
        const GOOD = fl!(30);      /* Good - never summon evil / never summoned by evil */
        const NEUTRAL = fl!(31);

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl FromStr for BeingKindFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = BeingKindFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                // "EXAMPLE" => result |= ActorKindFlags::EXAMPLE,
                "HERO" => result |= BeingKindFlags::HERO,

                "UNIQUE" => result |= BeingKindFlags::UNIQUE,
                "QUESTOR" => result |= BeingKindFlags::QUESTOR,
                "GUARDIAN" => result |= BeingKindFlags::GUARDIAN,

                "MALE" => result |= BeingKindFlags::MALE,
                "FEMALE" => result |= BeingKindFlags::FEMALE,
                "OBJECT" => result |= BeingKindFlags::OBJECT,

                "CAN_BE_REPELLED" => result |= BeingKindFlags::CAN_BE_REPELLED,
                "CAN_BE_TURNED" => result |= BeingKindFlags::CAN_BE_TURNED,
                "WILL_NOT_ATTACK" => result |= BeingKindFlags::WILL_NOT_ATTACK,
                "CANNOT_BE_ATTACKED" => result |= BeingKindFlags::CANNOT_BE_ATTACKED,

                "FORCE_DEPTH" => result |= BeingKindFlags::FORCE_DEPTH,
                "FORCE_MAXHP" => result |= BeingKindFlags::FORCE_MAXHP,
                "FORCE_SLEEP" => result |= BeingKindFlags::FORCE_SLEEP,

                // "HAS_LITE" => result |= BeingKindFlags::HAS_LITE,
                // "INVISIBLE" => result |= BeingKindFlags::INVISIBLE,
                // "COLD_BLOOD" => result |= BeingKindFlags::COLD_BLOOD,
                // "EMPTY_MIND" => result |= BeingKindFlags::EMPTY_MIND,
                // "WEIRD_MIND" => result |= BeingKindFlags::WEIRD_MIND,
                // "MULTIPLY" => result |= BeingKindFlags::MULTIPLY,
                // "REGENERATE" => result |= BeingKindFlags::REGENERATE,
                // "POWERFUL" => result |= BeingKindFlags::POWERFUL,

                // "TRAIL" => result |= BeingKindFlags::TRAIL,
                // "SNEAKY" => result |= BeingKindFlags::SNEAKY,
                // "ARMOR" => result |= BeingKindFlags::ARMOR,
                "PROPER_NAME" => result |= BeingKindFlags::PROPER_NAME,
                "IGNORE_WHEN_SEEN" => result |= BeingKindFlags::IGNORE_WHEN_SEEN,

                // "WARRIOR" => result |= ActorKindFlags::WARRIOR,
                // "ARCHER" => result |= ActorKindFlags::ARCHER,
                // "PRIEST" => result |= ActorKindFlags::PRIEST,
                // "MAGE" => result |= ActorKindFlags::MAGE,
                // "HAS_AURA" => result |= BeingKindFlags::HAS_AURA,
                // "HAS_WEB" => result |= BeingKindFlags::HAS_WEB,
                // "NEED_LITE" => result |= BeingKindFlags::NEED_LITE,
                "EVIL" => result |= BeingKindFlags::EVIL,
                "GOOD" => result |= BeingKindFlags::GOOD,
                "NEUTRAL" => result |= BeingKindFlags::NEUTRAL,

                "" => {}
                _ => return Err(format!("Unknown ActorKindFlags: {}", s)),
            }
        }
        Ok(result)
    }
}

impl BeingKindFlags {
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

impl From<&str> for BeingKindFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

impl fmt::Display for BeingKindFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
