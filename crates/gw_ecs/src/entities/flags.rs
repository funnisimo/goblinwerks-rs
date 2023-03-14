use bitflags::bitflags;
use gw_util::fl;
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct ChangeFlags: u32 {
        const DEAD = fl!(0);

        const ADDED = fl!(1);
        const CHANGED = fl!(2);
        const DELETED = fl!(3);
    }
}

impl fmt::Display for ChangeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
