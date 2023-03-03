use super::FovTarget;
use crate::fl;
use bitflags::bitflags;
use std::fmt;

///////////////////////////////////////////////////////////
// FOV FLAGS

bitflags! {
    #[derive(Default)]
    pub struct FovFlags: u32 {

        const VISIBLE = fl!(0);
        const WAS_VISIBLE = fl!(1);
        const CLAIRVOYANT_VISIBLE = fl!(2);
        const WAS_CLAIRVOYANT_VISIBLE = fl!(3);
        const TELEPATHIC_VISIBLE = fl!(4);
        const WAS_TELEPATHIC_VISIBLE = fl!(5);

        const ITEM_DETECTED = fl!(6);
        const WAS_ITEM_DETECTED = fl!(7);
        const ACTOR_DETECTED = fl!(8);
        const WAS_ACTOR_DETECTED = fl!(9);

        const REVEALED = fl!(10);
        const WAS_REVEALED = fl!(11);

        const IN_FOV = fl!(13);
        const WAS_IN_FOV = fl!(14);

        const MAGIC_MAPPED = fl!(12);
        const ALWAYS_VISIBLE = fl!(15);

        const ANY_KIND_OF_VISIBLE = Self::VISIBLE.bits | Self::CLAIRVOYANT_VISIBLE.bits | Self::TELEPATHIC_VISIBLE.bits;
        const WAS_ANY_KIND_OF_VISIBLE = Self::WAS_VISIBLE.bits | Self::WAS_CLAIRVOYANT_VISIBLE.bits | Self::WAS_TELEPATHIC_VISIBLE.bits;
        const IS_WAS_ANY_KIND_OF_VISIBLE = Self::VISIBLE.bits | Self::WAS_VISIBLE.bits | Self::CLAIRVOYANT_VISIBLE.bits | Self::WAS_CLAIRVOYANT_VISIBLE.bits | Self::TELEPATHIC_VISIBLE.bits | Self::WAS_TELEPATHIC_VISIBLE.bits;

        const IS_DETECTED = Self::ITEM_DETECTED.bits | Self::ACTOR_DETECTED.bits;
        const WAS_DETECTED = Self::WAS_ITEM_DETECTED.bits | Self::WAS_ACTOR_DETECTED.bits;

        const PROMOTE = Self::ANY_KIND_OF_VISIBLE.bits | Self::IS_DETECTED.bits | Self::IN_FOV.bits;
        const WAS_PROMOTE = Self::WAS_ANY_KIND_OF_VISIBLE.bits | Self::WAS_DETECTED.bits | Self::WAS_IN_FOV.bits;
    }
}

impl fmt::Display for FovFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

///////////////////////////////////////////////////////////
// FOV COMPONENT

// #[derive(Component, Clone, Debug)]
#[derive(Clone, Debug)]
pub struct FOV {
    pub flags: Vec<FovFlags>,
    width: u32,
    height: u32,
    pub range: u32,
    pub dirty: bool,
}

impl FOV {
    pub fn new(range: u32) -> FOV {
        FOV {
            flags: Vec::new(),
            width: 0,
            height: 0,
            range,
            dirty: true,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.flags = vec![FovFlags::empty(); (self.width * self.height) as usize];
        }
    }

    pub fn len(&self) -> usize {
        self.flags.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &FovFlags> {
        self.flags.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FovFlags> {
        self.flags.iter_mut()
    }

    pub fn promote_flags(&mut self) {
        for flags in self.flags.iter_mut() {
            let overlap = FovFlags::PROMOTE.intersection(*flags);
            let to_set = FovFlags::from_bits(overlap.bits << 1).unwrap();
            flags.remove(FovFlags::PROMOTE | FovFlags::WAS_PROMOTE); // remove all visibility flags
            flags.insert(to_set); // set the was flags

            if flags.contains(FovFlags::REVEALED) {
                flags.insert(FovFlags::WAS_REVEALED);
            }
            if flags.contains(FovFlags::ALWAYS_VISIBLE) {
                flags.insert(FovFlags::VISIBLE);
            }
        }
    }

    pub fn has_xy(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as u32) < self.width && y >= 0 && (y as u32) < self.height
    }

    pub fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        match self.has_xy(x, y) {
            false => None,
            true => Some(((y as u32) * self.width + x as u32) as usize),
        }
    }

    pub fn magic_map_all(&mut self) {
        for flag in self.flags.iter_mut() {
            flag.insert(FovFlags::MAGIC_MAPPED);
        }
    }

    pub fn reveal_all(&mut self) {
        for flag in self.flags.iter_mut() {
            flag.insert(FovFlags::REVEALED);
        }
    }

    pub fn always_visible_all(&mut self) {
        for flag in self.flags.iter_mut() {
            flag.insert(FovFlags::ALWAYS_VISIBLE);
        }
    }

    pub fn visibility_changed(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.visibility_changed_idx(idx),
        }
    }

    pub fn visibility_changed_idx(&self, idx: usize) -> bool {
        let current = self.flags[idx].intersection(FovFlags::ANY_KIND_OF_VISIBLE);
        let was = self.flags[idx].intersection(FovFlags::WAS_ANY_KIND_OF_VISIBLE);
        let was_to_current = FovFlags::from_bits(was.bits >> 1).unwrap();
        current != was_to_current
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::VISIBLE),
        }
    }

    pub fn is_visible_idx(&self, idx: usize) -> bool {
        match self.flags.get(idx) {
            None => false,
            Some(flags) => flags.contains(FovFlags::VISIBLE),
        }
    }

    pub fn was_visible(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::WAS_VISIBLE),
        }
    }

    pub fn set_visible(&mut self, x: i32, y: i32) {
        match self.to_idx(x, y) {
            None => {}
            Some(idx) => self.flags[idx].insert(FovFlags::VISIBLE | FovFlags::REVEALED),
        }
    }

    pub fn is_or_was_visible(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::VISIBLE | FovFlags::WAS_VISIBLE),
        }
    }

    pub fn is_or_was_visible_idx(&self, idx: usize) -> bool {
        match self.flags.get(idx) {
            None => false,
            Some(flags) => flags.contains(FovFlags::VISIBLE | FovFlags::WAS_VISIBLE),
        }
    }

    pub fn is_revealed(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::REVEALED),
        }
    }

    pub fn is_revealed_idx(&self, idx: usize) -> bool {
        match self.flags.get(idx) {
            None => false,
            Some(flags) => flags.contains(FovFlags::REVEALED),
        }
    }

    pub fn was_revealed(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::WAS_REVEALED),
        }
    }

    pub fn is_mapped(&self, x: i32, y: i32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.flags[idx].intersects(FovFlags::MAGIC_MAPPED),
        }
    }

    pub fn is_mapped_idx(&self, idx: usize) -> bool {
        match self.flags.get(idx) {
            None => false,
            Some(flags) => flags.contains(FovFlags::MAGIC_MAPPED),
        }
    }

    pub fn set_mapped(&mut self, x: i32, y: i32) {
        match self.to_idx(x, y) {
            None => {}
            Some(idx) => self.flags[idx].insert(FovFlags::MAGIC_MAPPED),
        }
    }

    pub fn set_mapped_idx(&mut self, idx: usize) {
        self.flags[idx].insert(FovFlags::MAGIC_MAPPED);
    }
}

impl FovTarget for FOV {
    fn set_visible(&mut self, x: i32, y: i32, pct: f32) {
        if pct <= 0.0 {
            return;
        }
        match self.to_idx(x, y) {
            None => {}
            Some(idx) => self.flags[idx].insert(FovFlags::VISIBLE | FovFlags::REVEALED),
        }
    }

    fn reset(&mut self, width: u32, height: u32) {
        self.resize(width, height);
        self.promote_flags();
    }
}

///////////////////////////////////////////////////////////
// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn basic() {
        let mut fov = FOV::new(10);
        fov.reset(10, 10);

        assert!(!fov.is_revealed(5, 6));
        assert!(!fov.is_visible(5, 6));
        assert!(!fov.was_revealed(5, 6));
        assert!(!fov.was_visible(5, 6));
        assert!(!fov.visibility_changed(5, 6));

        fov.set_visible(5, 6);
        assert!(fov.is_revealed(5, 6));
        assert!(fov.is_visible(5, 6));
        assert!(!fov.was_revealed(5, 6));
        assert!(!fov.was_visible(5, 6));
        assert!(fov.visibility_changed(5, 6));

        fov.reset(10, 10);
        assert!(fov.is_revealed(5, 6));
        assert!(!fov.is_visible(5, 6));
        assert!(fov.was_revealed(5, 6));
        assert!(fov.was_visible(5, 6));
        assert!(fov.visibility_changed(5, 6));

        fov.set_visible(5, 6);
        assert!(fov.is_revealed(5, 6));
        assert!(fov.is_visible(5, 6));
        assert!(fov.was_revealed(5, 6));
        assert!(fov.was_visible(5, 6));
        assert!(!fov.visibility_changed(5, 6));
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", FovFlags::VISIBLE), "VISIBLE");

        let flags = FovFlags::REVEALED | FovFlags::VISIBLE;
        assert_eq!(format!("{:?}", flags), "VISIBLE | REVEALED");
    }
}
