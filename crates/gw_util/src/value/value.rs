use crate::point::Point;

use super::DataConvertError;
use super::Key;
use legion::Entity;
use std::collections::HashMap;
use std::fmt::Display;

///////////////////////////////////////////////////

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Empty,
    Index(usize),
    Integer(u64),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<Key, Value>),
    Point(i32, i32),

    Entity(Entity),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Empty => "empty",
            Value::Boolean(_) => "bool",
            Value::Entity(_) => "Entity",
            Value::Float(_) => "float",
            Value::Index(_) => "usize",
            Value::List(_) => "vec",
            Value::Map(_) => "map",
            Value::Integer(_) => "int",
            Value::Point(_, _) => "Point",
            Value::String(_) => "String",
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Empty => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_entity(&self) -> bool {
        match self {
            Value::Entity(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            Value::Index(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false,
        }
    }

    pub fn is_map(&self) -> bool {
        match self {
            Value::Map(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_point(&self) -> bool {
        match self {
            Value::Point(_, _) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Map(map) => map.get(&key.into()),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&Value> {
        match self {
            Value::List(list) => list.get(index),
            _ => None,
        }
    }

    pub fn get_path(&self, path: &str) -> Option<&Value> {
        match self {
            Value::Map(map) => {
                let keys: Vec<&str> = path.splitn(2, '.').collect();
                match map.get(&keys[0].into()) {
                    None => None,
                    Some(value) => match keys.len() {
                        1 => Some(value),
                        _ => value.get_path(keys[1]),
                    },
                }
            }
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<Key, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn to_map(self) -> Option<HashMap<Key, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn to_list(self) -> Option<Vec<Value>> {
        match self {
            Value::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<u64> {
        match self {
            Value::Index(v) => match (*v).try_into() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Value::Integer(v) => match (*v).try_into() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Value::Float(v) => Some(v.trunc() as u64),
            Value::String(v) => match v.parse::<u64>() {
                Err(_) => None,
                Ok(v) => Some(v),
            },
            Value::Boolean(v) => match v {
                true => Some(1),
                false => Some(0),
            },

            // Value::Blank => Ok(0.0),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Index(v) => Some(*v as f64),
            Value::Integer(v) => Some(*v as f64),
            Value::Float(v) => Some(*v),
            Value::String(v) => match v.parse::<f64>() {
                Err(_) => None,
                Ok(v) => Some(v),
            },
            Value::Boolean(v) => match v {
                true => Some(1.0),
                false => Some(0.0),
            },

            // Value::Blank => Ok(0.0),
            _ => None,
        }
    }

    pub fn as_point(&self) -> Option<Point> {
        match self {
            Value::Point(x, y) => Some(Point::new(*x, *y)),
            _ => None,
        }
    }

    pub fn as_entity(&self) -> Option<&Entity> {
        match self {
            Value::Entity(e) => Some(e),
            _ => None,
        }
    }

    pub fn to_entity(self) -> Option<Entity> {
        match self {
            Value::Entity(e) => Some(e),
            _ => None,
        }
    }
}

impl TryInto<usize> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => match (v.trunc() as u64).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::String(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// impl TryInto<usize> for &Value {
//     type Error = DataConvertError;

//     fn try_into(self) -> Result<usize, DataConvertError> {
//         match self {
//             Value::Index(v) => match (*v).try_into() {
//                 Ok(v) => Ok(v),
//                 Err(_) => Err(DataConvertError::WrongType),
//             },
//             Value::Number(v) => match (*v).try_into() {
//                 Ok(v) => Ok(v),
//                 Err(_) => Err(DataConvertError::WrongType),
//             },
//             Value::Float(v) => match (v.trunc() as u64).try_into() {
//                 Ok(v) => Ok(v),
//                 Err(_) => Err(DataConvertError::WrongType),
//             },
//             Value::Text(v) => match v.parse::<usize>() {
//                 Err(_) => Err(DataConvertError::WrongType),
//                 Ok(v) => Ok(v),
//             },
//             Value::Boolean(v) => match v {
//                 true => Ok(1),
//                 false => Ok(0),
//             },

//             // Value::Blank => Ok(0.0),
//             _ => Err(DataConvertError::WrongType),
//         }
//     }
// }

// u64

impl TryInto<u64> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as u64),
            Value::String(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i64

impl TryInto<i64> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i64, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as i64),
            Value::String(v) => match v.parse::<i64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u32

impl TryInto<u32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as u32),
            Value::String(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i32

impl TryInto<i32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.floor() as i32),
            Value::String(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u16

impl TryInto<u16> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u16, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as u16),
            Value::String(v) => match v.parse::<u16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i16

impl TryInto<i16> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i16, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as i16),
            Value::String(v) => match v.parse::<i16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u8

impl TryInto<u8> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u8, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as u8),
            Value::String(v) => match v.parse::<u8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i8

impl TryInto<i8> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i8, DataConvertError> {
        match self {
            Value::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Value::Float(v) => Ok(v.trunc() as i8),
            Value::String(v) => match v.parse::<i8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// f64

impl TryInto<f64> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f64, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as f64),
            Value::Integer(v) => Ok(v as f64),
            Value::Float(v) => Ok(v),
            Value::String(v) => match v.parse::<f64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

// f32

impl TryInto<f32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as f32),
            Value::Integer(v) => Ok(v as f32),
            Value::Float(v) => Ok(v as f32),
            Value::String(v) => match v.parse::<f32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<bool> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<bool, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v != 0),
            Value::Integer(v) => Ok(v != 0),
            Value::Float(v) => Ok(v != 0.0),
            Value::String(v) => Ok(v.len() > 0),
            Value::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            // Value::Blank => Ok(false),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<Point> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Point, DataConvertError> {
        match self {
            Value::Point(x, y) => Ok(Point::new(x, y)),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<Entity> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Entity, DataConvertError> {
        match self {
            Value::Entity(e) => Ok(e),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<HashMap<Key, Value>> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<HashMap<Key, Value>, Self::Error> {
        match self {
            Value::Map(map) => Ok(map),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<Vec<Value>> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Vec<Value>, Self::Error> {
        match self {
            Value::List(val) => Ok(val),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "()"),
            Value::Index(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Boolean(v) => match v {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::List(data) => {
                write!(f, "[")?;
                for (i, item) in data.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Map(data) => {
                write!(f, "{{")?;
                for (key, val) in data.iter() {
                    write!(f, "{:?}={}, ", key, val)?;
                }
                write!(f, "}}")
            }
            Value::Point(x, y) => write!(f, "({},{})", x, y),

            Value::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
        }
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Value::Index(v)
    }
}

// u64
impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Integer(v as u64)
    }
}

// i64
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v as u64)
    }
}

// u32
impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Integer(v as u64)
    }
}

// i32
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v as u64)
    }
}

// u16
impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::Integer(v as u64)
    }
}

// i16
impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Integer(v as u64)
    }
}

// u8
impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value::Integer(v as u64)
    }
}

// i8
impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Integer(v as u64)
    }
}

// f64
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

// f32
impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v as f64)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_owned())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::String(v.clone())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Value::List(vec)
    }
}

impl From<HashMap<Key, Value>> for Value {
    fn from(data: HashMap<Key, Value>) -> Self {
        Value::Map(data)
    }
}

impl From<Point> for Value {
    fn from(value: Point) -> Self {
        Value::Point(value.x, value.y)
    }
}

impl From<Key> for Value {
    fn from(value: Key) -> Self {
        match value {
            Key::Index(v) => Value::Index(v),
            Key::Integer(v) => Value::Integer(v),
            Key::String(v) => Value::String(v),
            Key::Point(x, y) => Value::Point(x, y),

            Key::Entity(v) => Value::Entity(v),
        }
    }
}

impl From<Entity> for Value {
    fn from(value: Entity) -> Self {
        Value::Entity(value)
    }
}
