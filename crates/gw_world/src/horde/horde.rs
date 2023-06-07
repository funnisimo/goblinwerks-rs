use super::horde_flags::HordeFlags;
use crate::being::BeingKind;
use gw_ecs::prelude::Component;
use gw_util::frequency::Frequency;
use std::{
    fmt::{Debug, Formatter, Result},
    ops::Deref,
    sync::Arc,
};

pub struct Horde {
    pub(super) leader: Arc<BeingKind>,
    pub(super) frequency: Frequency, // TODO - Need Frequency type
    pub(super) members: Vec<(Arc<BeingKind>, u32)>,
    pub(super) spawn_tile: Option<String>,
    pub(super) machine_id: u32,
    pub(super) flags: HordeFlags,
    pub(super) tags: Vec<String>,
}

impl Horde {
    pub(super) fn new(leader: Arc<BeingKind>) -> Self {
        Horde {
            leader,
            frequency: Frequency::new(),
            members: Vec::new(),
            spawn_tile: None,
            machine_id: 0,
            flags: HordeFlags::empty(),
            tags: Vec::new(),
        }
    }

    pub fn frequency(&self, level: u32) -> u32 {
        // if (typeof horde.frequency == 'number') return horde.frequency;
        // if (Array.isArray(horde.frequency)) {
        //   const delta = Math.max(0, depth - horde.levelRange[0]);
        //   return Math.max(0, horde.frequency[0] + delta * horde.frequency[1]);
        // }
        // return 1;

        self.frequency.get_weight(level)
    }
}

impl Debug for Horde {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = f.debug_struct("Horde");
        s.field("leader", &self.leader.id);
        s.field("frequency", &self.frequency);
        if self.members.len() > 0 {
            let members: Vec<(String, u32)> =
                self.members.iter().map(|m| (m.0.id.clone(), m.1)).collect();
            s.field("members", &members);
        }
        if let Some(tile) = self.spawn_tile.as_ref() {
            s.field("spawn_tile", tile);
        }
        if self.machine_id > 0 {
            s.field("machine_id", &self.machine_id);
        }
        if self.flags.is_empty() == false {
            s.field("flags", &self.flags);
        }
        if self.tags.len() > 0 {
            s.field("tags", &self.tags);
        }
        s.finish()
    }
}

#[derive(Component)]
pub struct HordeRef(Arc<Horde>);

impl HordeRef {
    pub fn new(horde: Arc<Horde>) -> Self {
        HordeRef(horde)
    }

    pub fn is(&self, horde: &Arc<Horde>) -> bool {
        Arc::ptr_eq(&self.0, horde)
    }
}

impl Deref for HordeRef {
    type Target = Horde;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
