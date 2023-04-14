use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct StatValue {
    value: i32,
    max: i32,
    min: i32,
}

impl StatValue {
    pub fn new(val: i32) -> Self {
        StatValue {
            value: val,
            max: val.max(0),
            min: val.min(0),
        }
    }

    pub fn set_max(&mut self, max: i32) {
        self.max = max;
    }

    pub fn set_min(&mut self, min: i32) {
        self.min = min;
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn reset(&mut self, value_max: i32) {
        self.value = value_max;
        self.max = value_max;
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn update(&mut self, delta: i32) -> i32 {
        self.value = self.value.saturating_add(delta).clamp(self.min, self.max);
        self.value
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Stat {
    HEALTH,
    MAGIC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    data: HashMap<Stat, StatValue>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, stat: Stat) -> Option<i32> {
        match self.data.get(&stat) {
            None => None,
            Some(val) => Some(val.value()),
        }
    }

    pub fn update(&mut self, stat: Stat, delta: i32) -> i32 {
        match self.data.get_mut(&stat) {
            None => {
                self.data.insert(stat, StatValue::new(delta));
                delta
            }
            Some(val) => val.update(delta),
        }
    }

    pub fn set(&mut self, stat: Stat, value: i32) {
        match self.data.get_mut(&stat) {
            None => {
                self.data.insert(stat, StatValue::new(value));
            }
            Some(val) => val.set_value(value),
        }
    }
}
