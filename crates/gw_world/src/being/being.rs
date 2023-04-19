use super::AIFlags;
use super::BeingKindFlags;
use super::MoveFlags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Being {
    pub id: String,

    pub move_flags: MoveFlags,
    pub ai_flags: AIFlags,
    pub kind_flags: BeingKindFlags,
    pub act_time: u32,

    pub xp: u32,
    pub name: Option<String>,
    pub talk: Option<String>,
    pub flavor: Option<String>,
    pub description: Option<String>,
}

impl Being {
    pub fn new(id: String) -> Self {
        Being {
            id,

            act_time: 100,
            move_flags: MoveFlags::empty(),
            ai_flags: AIFlags::empty(),
            kind_flags: BeingKindFlags::empty(),

            xp: 0,
            name: None,
            talk: None,
            flavor: None,
            description: None,
        }
    }

    pub fn name(&self) -> &String {
        match self.name {
            None => match self.flavor {
                None => &self.id,
                Some(ref flavor) => flavor,
            },
            Some(ref name) => name,
        }
    }
}
