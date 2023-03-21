use crate::value::Value;
use jsonc_parser::parse_to_value;
use jsonc_parser::JsonValue;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn parse_file(path: &str) -> Result<Value, String> {
    let text = read_to_string(path).expect("Failed to open file");
    parse_string(&text)
}

// pub fn parse_toml_inline(_value: &str) -> Result<HashMap<String, String>, String> {
//     Err("Not implemented.".to_owned())
// }

pub fn parse_string(text: &str) -> Result<Value, String> {
    let root = parse_to_value(&text, &Default::default())
        .expect("Parse failed.")
        .unwrap();

    Ok(root.into())
}

impl<'a> Into<Value> for JsonValue<'a> {
    fn into(self) -> Value {
        match self {
            JsonValue::Array(arr) => Value::List(arr.into_iter().map(|jv| jv.into()).collect()),
            JsonValue::Boolean(val) => Value::Boolean(val),
            JsonValue::Null => Value::Empty,
            JsonValue::Number(val) => {
                if val.contains(".") {
                    Value::Float(val.parse().unwrap())
                } else {
                    println!("JsonNumber({})", val);
                    Value::Integer(val.parse().unwrap())
                }
            }
            JsonValue::Object(obj) => {
                let mut map = HashMap::new();

                for (key, val) in obj.into_iter() {
                    map.insert(key.into(), val.into());
                }

                Value::Map(map)
            }
            JsonValue::String(str) => Value::String(str.to_string()),
        }
    }
}
