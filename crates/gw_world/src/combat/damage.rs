use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum DamageKind {
    BASH,
    SLASH,
    PIERCE,

    MAGIC,
    HEAL,
    POISON,
}

#[derive(Debug, Clone, Copy)]
pub struct DamageInfo {
    pub kind: DamageKind,
    pub amount: i32,
}

impl DamageInfo {
    pub fn new(kind: DamageKind, amount: i32) -> Self {
        DamageInfo { kind, amount }
    }
}

impl Display for DamageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[red][{}]", self.amount)
    }
}
