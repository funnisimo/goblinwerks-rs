use crate::formula::types;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use toml_edit::{Formatted, Item, Value};

pub type StringTable = HashMap<String, HashMap<String, String>>;

pub fn string_table_from_file(path: &str) -> Result<StringTable, String> {
    let file = match File::open(path) {
        Err(e) => return Err(e.to_string()),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(file);
    parse_reader(&mut reader)
}

// pub fn parse_toml_inline(_value: &str) -> Result<HashMap<String, String>, String> {
//     Err("Not implemented.".to_owned())
// }

pub fn parse_to_string_table(data: &str) -> Result<StringTable, String> {
    let mut bytes = data.as_bytes();
    parse_reader(&mut bytes)
}

pub fn parse_reader(reader: &mut dyn BufRead) -> Result<StringTable, String> {
    let mut out = HashMap::new();

    let mut current: Option<&mut HashMap<String, String>> = None;

    let mut line_count = 0;
    for line_text in reader.lines() {
        line_count = line_count + 1;
        match line_text {
            Err(e) => return Err(e.to_string()),
            Ok(line_text) => {
                let line = line_text.trim();
                if line.len() == 0 || line.starts_with("#") {
                    continue;
                }
                if line.starts_with("[") {
                    let name = match line.chars().position(|ch| ch == ']') {
                        None => {
                            return Err(format!("{} - expected closing ']' : {}", line_count, line))
                        }
                        Some(pos) => line[1..pos].to_owned(),
                    };

                    out.insert(name.clone(), HashMap::new());
                    current = out.get_mut(&name);
                } else if current.is_none() {
                    return Err(format!(
                        "{} - Unexpected data, not in a group: {}",
                        line_count, line
                    ));
                } else {
                    let line_parts: Vec<&str> = line.splitn(2, "=").map(|p| p.trim()).collect();
                    if line_parts.len() != 2 {
                        return Err(format!(
                            "{} - expected 'field=value', found: {}",
                            line_count, line
                        ));
                    }
                    let key = line_parts[0];
                    let mut value = line_parts[1];
                    if value.starts_with("\"\"\"") {
                        // multi-line basic string
                    } else if value.starts_with("\"") {
                        // basic string
                        let a = value.chars();
                        let b = value.chars().skip(1);
                        value = match a.zip(b).position(|(a, b)| b == '"' && a != '\\') {
                            None => {
                                return Err(format!(
                                    "{} - Unclosed basic string : {}",
                                    line_count, line
                                ))
                            }
                            Some(idx) => &value[1..idx + 1],
                        };

                        if let Some(hash) = current.as_mut() {
                            hash.insert(key.to_owned(), value.to_owned());
                        }
                    } else if value.starts_with("'''") {
                        // multi-line literal string
                    } else if value.starts_with("'") {
                        // literal string
                        let a = value.chars();
                        let b = value.chars().skip(1);
                        value = match a.zip(b).position(|(a, b)| b == '\'' && a != '\\') {
                            None => {
                                return Err(format!(
                                    "{} - Unclosed literal string : {}",
                                    line_count, line
                                ))
                            }
                            Some(idx) => &value[1..idx + 1],
                        };

                        if let Some(hash) = current.as_mut() {
                            hash.insert(key.to_owned(), value.to_owned());
                        }
                    } else {
                        // generally any single value is ok now
                        // strip off any comments
                        value = match value.chars().skip(1).position(|ch| ch == '#') {
                            None => value,
                            Some(pos) => &value[..pos + 1].trim(),
                        };

                        if let Some(hash) = current.as_mut() {
                            hash.insert(key.to_owned(), value.to_owned());
                        }
                    }
                }
            }
        }
    }

    Ok(out)
}

pub fn num_value<V>(val: V) -> types::Value
where
    V: Into<f32>,
{
    types::Value::Number(val.into())
}

#[derive(Debug)]
pub enum GetErr {
    NotFound,
    NotValue,
    WrongType,
    ParseFail(String),
}

impl GetErr {
    pub fn as_str(&self) -> &str {
        match self {
            GetErr::NotFound => "Not found",
            GetErr::NotValue => "Not a value type",
            GetErr::WrongType => "Not expected datatype",
            GetErr::ParseFail(e) => e.as_str(),
        }
    }
}

pub fn get_str<'a>(val: Option<&'a Item>) -> Result<&'a Formatted<String>, GetErr> {
    match get_str_opt(val) {
        Err(e) => Err(e),
        Ok(None) => Err(GetErr::NotFound),
        Ok(v) => Ok(v.unwrap()),
    }
}

pub fn get_str_opt<'a>(val: Option<&'a Item>) -> Result<Option<&'a Formatted<String>>, GetErr> {
    match val {
        None => Ok(None),
        Some(item) => match item {
            Item::Value(value) => match value {
                Value::String(val) => Ok(Some(val)),
                _ => Err(GetErr::WrongType),
            },
            _ => Err(GetErr::NotValue),
        },
    }
}

pub fn get_num<'a>(val: Option<&'a Item>) -> Result<f32, GetErr> {
    match get_num_opt(val) {
        Err(e) => Err(e),
        Ok(None) => Err(GetErr::NotFound),
        Ok(v) => Ok(v.unwrap()),
    }
}

pub fn get_num_opt<'a>(val: Option<&'a Item>) -> Result<Option<f32>, GetErr> {
    match val {
        None => Ok(None),
        Some(item) => match item {
            Item::Value(value) => match value {
                Value::String(val) => match val.value().parse::<f32>() {
                    Err(e) => Err(GetErr::ParseFail(e.to_string())),
                    Ok(v) => Ok(Some(v)),
                },
                Value::Integer(value) => Ok(Some(*value.value() as f32)),
                Value::Float(value) => Ok(Some(*value.value() as f32)),
                _ => Err(GetErr::WrongType),
            },
            _ => Err(GetErr::NotValue),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_table() {
        let mut table = r#"
        # comment
        [group]
        # comment
        a = 1
        b = 2 # comment
        c = "3" # comment with "quotes"

        [another]
        a = 43
        b = "taco bell"
        c = -23
        "#
        .as_bytes();

        let toml = parse_reader(&mut table).expect("Failed to parse");

        println!("table = {:?}", toml);

        assert!(toml.contains_key(&"group".to_owned()));
        match toml.get(&"group".to_owned()) {
            None => panic!("Expected 'group'"),
            Some(hash) => {
                assert_eq!(hash.get(&"a".to_owned()).unwrap(), "1");
                assert_eq!(hash.get(&"b".to_owned()).unwrap(), "2");
                assert_eq!(hash.get(&"c".to_owned()).unwrap(), "3");
            }
        }

        assert!(toml.contains_key(&"another".to_owned()));
        match toml.get(&"another".to_owned()) {
            None => panic!("Expected 'another'"),
            Some(hash) => {
                assert_eq!(hash.get(&"a".to_owned()).unwrap(), "43");
                assert_eq!(hash.get(&"b".to_owned()).unwrap(), "taco bell");
                assert_eq!(hash.get(&"c".to_owned()).unwrap(), "-23");
            }
        }
    }
}
