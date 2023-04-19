// A Frequency is a set of values that indicate the weight of something at different levels
// Frequencies can be constant: e.g. 100,
// They can also have different values for different levels: e.g. 1: 100, 2: 200
// They can also have values for level ranges: e.g. 1-5: 100
// And can have values for all levels above one: e.g. 10+: 200
//

use std::{cmp::Ordering, num::ParseIntError};

use crate::{
    formula::{parse_string_to_formula, Formula, NoCustomFunction},
    value::{Key, Value},
};

fn calc_formula(formula: &Formula, level: u32) -> Option<u32> {
    let ref_fn = |var: String| -> Option<Value> {
        if var.to_uppercase() == "LEVEL" {
            return Some(level.into());
        }
        None
    };

    match formula.calculate(None::<NoCustomFunction>, Some(&ref_fn)) {
        Err(_) => None,
        Ok(val) => val.as_int().map(|v| v as u32),
    }
}

#[derive(Debug, Clone)]
pub enum FreqEntry {
    Level(u32, Formula),      // for a single level, use the value
    Range(u32, u32, Formula), // for a level range use this value
    Over(u32, Formula),       // Over this level use this value
    Const(Formula),           // Use this value
}

impl FreqEntry {
    pub fn get_weight(&self, level: u32) -> Option<u32> {
        match self {
            FreqEntry::Const(v) => return calc_formula(v, level),
            FreqEntry::Level(l, v) => {
                if *l == level {
                    return calc_formula(v, level);
                }
            }
            FreqEntry::Range(l, h, v) => {
                if *l <= level && *h >= level {
                    return calc_formula(v, level);
                }
            }
            FreqEntry::Over(h, v) => {
                if level >= *h {
                    return calc_formula(v, level);
                }
            }
        }
        None
    }

    fn base_level(&self) -> (u32, u32) {
        match self {
            FreqEntry::Const(_v) => (4, u32::MAX), // last
            FreqEntry::Level(l, _v) => (1, *l),
            FreqEntry::Range(l, _h, _v) => (2, *l),
            FreqEntry::Over(h, _v) => (3, *h),
        }
    }
}

fn sort_entries(a: &FreqEntry, b: &FreqEntry) -> Ordering {
    let a_l = a.base_level();
    let b_l = b.base_level();
    match a_l.0.cmp(&b_l.0) {
        Ordering::Equal => a_l.1.cmp(&b_l.1),
        x => x,
    }
}

#[derive(Debug, Clone, Default)]
pub struct Frequency {
    entries: Vec<FreqEntry>,
}

impl Frequency {
    pub fn new() -> Self {
        Frequency {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, entry: FreqEntry) {
        self.entries.push(entry);
        self.entries.sort_by(sort_entries);
    }

    pub fn get_weight(&self, level: u32) -> Option<u32> {
        for entry in self.entries.iter() {
            if let Some(v) = entry.get_weight(level) {
                return Some(v);
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum FrequencyParseError {
    InvalidType(Value),
    InvalidKey(Key),
    InvalidValue(Key, Value),
}

pub fn from_value(value: &Value) -> Result<Frequency, FrequencyParseError> {
    let mut freq = Frequency::new();
    if value.is_int() {
        freq.push(FreqEntry::Const(Formula::Value(value.clone())));
        return Ok(freq);
    } else if value.is_string() {
        // value is a const formula
        let formula = value.as_str().unwrap().trim();
        let formula = if formula.starts_with("=") {
            parse_string_to_formula(formula)
        } else {
            let formula = format!("={}", formula);
            parse_string_to_formula(&formula)
        };

        if formula.is_err() {
            return Err(FrequencyParseError::InvalidType(value.clone()));
        }
        freq.push(FreqEntry::Const(formula));
        return Ok(freq);
    } else if value.is_map() {
        let map = value.as_map().unwrap();
        for (key, value) in map.iter() {
            let formula = if value.is_int() {
                Formula::Value(value.clone())
            } else if value.is_string() {
                // value is a const formula
                let formula = value.as_str().unwrap().trim();
                let formula = if formula.starts_with("=") {
                    parse_string_to_formula(formula)
                } else {
                    let formula = format!("={}", formula);
                    parse_string_to_formula(&formula)
                };
                if formula.is_err() {
                    return Err(FrequencyParseError::InvalidType(value.clone()));
                }
                formula
            } else {
                return Err(FrequencyParseError::InvalidValue(
                    key.clone(),
                    value.clone(),
                ));
            };

            if key.is_int() {
                if value.is_int() {
                    freq.push(FreqEntry::Level(key.as_int().unwrap() as u32, formula));
                } else {
                    return Err(FrequencyParseError::InvalidValue(
                        key.clone(),
                        value.clone(),
                    ));
                }
            } else if key.is_string() {
                let key_str = key.as_str().unwrap().trim();
                // includes "-"
                if key_str.contains("-") {
                    let parts: Vec<Result<u32, ParseIntError>> =
                        key_str.split("-").map(|v| v.parse()).collect();
                    if parts.len() != 2 {
                        return Err(FrequencyParseError::InvalidValue(
                            key.clone(),
                            value.clone(),
                        ));
                    }
                    if parts.iter().any(|v| v.is_err()) {
                        return Err(FrequencyParseError::InvalidValue(
                            key.clone(),
                            value.clone(),
                        ));
                    }

                    freq.push(FreqEntry::Range(
                        *parts[0].as_ref().unwrap(),
                        *parts[1].as_ref().unwrap(),
                        formula,
                    ));
                } else if key_str.ends_with("+") {
                    // ends with "+"
                    let level = match key_str.strip_suffix("+").unwrap().parse() {
                        Err(_) => {
                            return Err(FrequencyParseError::InvalidValue(
                                key.clone(),
                                value.clone(),
                            ))
                        }
                        Ok(v) => v,
                    };

                    freq.push(FreqEntry::Over(level, formula));
                } else {
                    // other
                    match key_str.parse::<u32>() {
                        Ok(level) => {
                            freq.push(FreqEntry::Level(level, formula));
                        }
                        Err(_) => return Err(FrequencyParseError::InvalidKey(key.clone())),
                    }
                }
            } else {
                return Err(FrequencyParseError::InvalidKey(key.clone()));
            }
        }
        return Ok(freq);
    }
    Err(FrequencyParseError::InvalidType(value.clone()))
}

#[cfg(test)]
mod test {
    use crate::json;

    use super::*;

    #[test]
    fn constant() {
        let mut freq = Frequency::new();
        freq.push(FreqEntry::Const(Formula::Value(100.into())));

        assert_eq!(freq.get_weight(0), Some(100));
        assert_eq!(freq.get_weight(10), Some(100));
        assert_eq!(freq.get_weight(100), Some(100));
        assert_eq!(freq.get_weight(1000), Some(100));
    }

    #[test]
    fn level() {
        let mut freq = Frequency::new();
        freq.push(FreqEntry::Level(5, Formula::Value(100.into())));

        assert_eq!(freq.get_weight(0), None);
        assert_eq!(freq.get_weight(5), Some(100));
        assert_eq!(freq.get_weight(10), None);
    }

    #[test]
    fn range() {
        let mut freq = Frequency::new();
        freq.push(FreqEntry::Range(5, 10, Formula::Value(100.into())));

        assert_eq!(freq.get_weight(4), None);
        assert_eq!(freq.get_weight(5), Some(100));
        assert_eq!(freq.get_weight(7), Some(100));
        assert_eq!(freq.get_weight(10), Some(100));
        assert_eq!(freq.get_weight(11), None);
    }

    #[test]
    fn over() {
        let mut freq = Frequency::new();
        freq.push(FreqEntry::Over(5, Formula::Value(100.into())));

        assert_eq!(freq.get_weight(4), None);
        assert_eq!(freq.get_weight(5), Some(100));
        assert_eq!(freq.get_weight(7), Some(100));
        assert_eq!(freq.get_weight(10), Some(100));
        assert_eq!(freq.get_weight(11), Some(100));
    }

    #[test]
    fn kept_sorted() {
        let mut freq = Frequency::new();
        freq.push(FreqEntry::Range(6, 8, Formula::Value(20.into())));
        freq.push(FreqEntry::Over(5, Formula::Value(100.into())));
        freq.push(FreqEntry::Level(7, Formula::Value(10.into())));

        assert_eq!(freq.get_weight(4), None);
        assert_eq!(freq.get_weight(5), Some(100));
        assert_eq!(freq.get_weight(6), Some(20));
        assert_eq!(freq.get_weight(7), Some(10));
        assert_eq!(freq.get_weight(8), Some(20));
        assert_eq!(freq.get_weight(9), Some(100));
        assert_eq!(freq.get_weight(11), Some(100));
    }

    #[test]
    fn parse_const() {
        let value: Value = 100.into();

        let freq: Frequency = from_value(&value).unwrap();
        assert_eq!(freq.get_weight(0), Some(100));
        assert_eq!(freq.get_weight(10), Some(100));
        assert_eq!(freq.get_weight(100), Some(100));
        assert_eq!(freq.get_weight(1000), Some(100));

        // formula
        let value: Value = "=100".into();

        let freq: Frequency = from_value(&value).unwrap();
        assert_eq!(freq.get_weight(0), Some(100));
        assert_eq!(freq.get_weight(10), Some(100));
        assert_eq!(freq.get_weight(100), Some(100));
        assert_eq!(freq.get_weight(1000), Some(100));

        // formula with Level
        let value: Value = "=1 + level".into();

        let freq: Frequency = from_value(&value).unwrap();
        assert_eq!(freq.get_weight(0), Some(1));
        assert_eq!(freq.get_weight(10), Some(11));
        assert_eq!(freq.get_weight(100), Some(101));
        assert_eq!(freq.get_weight(1000), Some(1001));

        // formula with Level and float
        let value: Value = "=2.5 * level".into();

        let freq: Frequency = from_value(&value).unwrap();
        assert_eq!(freq.get_weight(0), Some(0));
        assert_eq!(freq.get_weight(1), Some(2));
        assert_eq!(freq.get_weight(2), Some(5));
        assert_eq!(freq.get_weight(10), Some(25));
    }

    #[test]
    fn parse_level_map() {
        let text = "{ 1: 10, 2: 20, 3: 30 }";
        let json = json::parse_string(text).unwrap();

        let freq: Frequency = from_value(&json).unwrap();
        assert_eq!(freq.get_weight(0), None);
        assert_eq!(freq.get_weight(1), Some(10));
        assert_eq!(freq.get_weight(2), Some(20));
        assert_eq!(freq.get_weight(3), Some(30));
        assert_eq!(freq.get_weight(4), None);

        // we will add the '=' automagically to formulas
        let text = r#"{ 1: "10", 2: "2 + level", 3: 30 }"#;
        let json = json::parse_string(text).unwrap();

        let freq: Frequency = from_value(&json).unwrap();
        assert_eq!(freq.get_weight(0), None);
        assert_eq!(freq.get_weight(1), Some(10));
        assert_eq!(freq.get_weight(2), Some(4));
        assert_eq!(freq.get_weight(3), Some(30));
        assert_eq!(freq.get_weight(4), None);
    }

    #[test]
    fn parse_map_range() {
        // we will add the '=' automagically to formulas
        let text = r#"{ "1-3": 10, "4-6": 20, "7+": 30 }"#;
        let json = json::parse_string(text).unwrap();

        let freq: Frequency = from_value(&json).unwrap();
        assert_eq!(freq.get_weight(0), None);
        assert_eq!(freq.get_weight(1), Some(10));
        assert_eq!(freq.get_weight(3), Some(10));
        assert_eq!(freq.get_weight(4), Some(20));
        assert_eq!(freq.get_weight(6), Some(20));
        assert_eq!(freq.get_weight(7), Some(30));
        assert_eq!(freq.get_weight(17), Some(30));
    }
}
