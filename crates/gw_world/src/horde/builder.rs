use super::{Horde, HordeFlags};
use crate::being::{self, BeingKind, BeingKinds};
use gw_util::frequency::{self, Frequency};
use gw_util::value::Value;
use std::sync::Arc;

pub struct HordeBuilder {
    pub(super) horde: Horde,
}

impl HordeBuilder {
    pub(super) fn new(leader: Arc<BeingKind>) -> Self {
        HordeBuilder {
            horde: Horde::new(leader),
        }
    }

    pub fn frequency(&mut self, frequency: Frequency) -> &mut Self {
        self.horde.frequency = frequency;
        self
    }

    pub fn add_member(&mut self, member: Arc<BeingKind>, count: u32) -> &mut Self {
        self.horde.members.push((member, count));
        self
    }

    pub fn spawn_tile(&mut self, id: &str) -> &mut Self {
        self.horde.spawn_tile = Some(id.to_string());
        self
    }

    pub fn machine_id(&mut self, id: u32) -> &mut Self {
        self.horde.machine_id = id;
        self
    }

    pub fn apply_flags(&mut self, flag_string: &str) -> &mut Self {
        self.horde.flags.apply(flag_string);
        self
    }

    pub fn apply_tags(&mut self, tag_string: &str) -> &mut Self {
        for tag in tag_string.split(&[',', '|']).map(|v| v.trim()) {
            if tag.starts_with("!") {
                let tag = tag.strip_prefix("!").unwrap().trim();
                if let Some(pos) = self.horde.tags.iter().position(|t| t == tag) {
                    self.horde.tags.remove(pos);
                }
            } else {
                self.horde.tags.push(tag.to_string());
            }
        }
        self
    }

    pub fn build(self) -> Horde {
        self.horde
    }
}

#[derive(Debug, Clone)]
pub enum BuilderError {
    UnknownField(String),
    BadField(String, Value),
    BadMember(Value),
}

pub fn set_field(
    builder: &mut HordeBuilder,
    field: &str,
    value: &Value,
    kinds: &BeingKinds,
) -> Result<(), BuilderError> {
    match field {
        "leader" => match value.as_str() {
            None => Err(BuilderError::BadField(field.to_string(), value.clone())),
            Some(id) => match kinds.get(id) {
                None => Err(BuilderError::BadField(field.to_string(), value.clone())),
                Some(kind) => {
                    builder.horde.leader = kind;
                    Ok(())
                }
            },
        },
        "frequency" => match frequency::from_value(value) {
            Err(_) => Err(BuilderError::BadField(field.to_string(), value.clone())),
            Ok(f) => {
                builder.horde.frequency = f;
                Ok(())
            }
        },
        "members" => match value.as_list() {
            None => Err(BuilderError::BadField(field.to_string(), value.clone())),
            Some(list) => {
                let mut members: Vec<(Arc<BeingKind>, u32)> = Vec::new();
                for entry in list.iter() {
                    match being::from_value(entry, kinds, "CUSTOM", None) {
                        Err(_) => return Err(BuilderError::BadMember(entry.clone())),
                        Ok(being) => {
                            members.push((being, 1));
                        }
                    }
                }

                builder.horde.members = members;
                Ok(())
            }
        },
        "counts" => {
            if value.is_int() {
                let count = value.as_int().unwrap() as u32;

                for entry in builder.horde.members.iter_mut() {
                    entry.1 = count;
                }
                Ok(())
            } else if value.is_list() {
                let list = value.as_list().unwrap();
                // TODO - Validate len vs members len?
                if !list.iter().all(|v| v.is_int()) {
                    return Err(BuilderError::BadField(field.to_string(), value.clone()));
                }
                let counts: Vec<u32> = list.iter().map(|v| v.as_int().unwrap() as u32).collect();
                for (entry, count) in builder.horde.members.iter_mut().zip(counts.iter()) {
                    entry.1 = *count;
                }
                Ok(())
            } else {
                Err(BuilderError::BadField(field.to_string(), value.clone()))
            }
        }
        "tile" | "spawn_tile" => {
            if value.is_bool() {
                if value.as_bool().unwrap() {
                    Err(BuilderError::BadField(field.to_string(), value.clone()))
                } else {
                    Ok(())
                }
            } else {
                match value.as_str() {
                    None => Err(BuilderError::BadField(field.to_string(), value.clone())),
                    Some(id) => {
                        builder.spawn_tile(id);
                        Ok(())
                    }
                }
            }
        }
        "machine" | "machine_id" => match value.as_int() {
            None => Err(BuilderError::BadField(field.to_string(), value.clone())),
            Some(machine) => {
                builder.machine_id(machine as u32);
                Ok(())
            }
        },
        "flags" | "horde_flags" => {
            if value.is_int() {
                let v = value.as_int().unwrap() as u32;
                builder
                    .horde
                    .flags
                    .insert(HordeFlags::from_bits_truncate(v));
                Ok(())
            } else {
                match value.as_str() {
                    None => Err(BuilderError::BadField(field.to_string(), value.clone())),
                    Some(tag_str) => {
                        builder.apply_flags(tag_str);
                        Ok(())
                    }
                }
            }
        }
        "tags" => match value.as_str() {
            None => Err(BuilderError::BadField(field.to_string(), value.clone())),
            Some(tag_str) => {
                builder.apply_tags(tag_str);
                Ok(())
            }
        },
        _ => Err(BuilderError::UnknownField(field.to_string())),
    }
}
