use super::DataConvertError;
use super::Key;
use crate::ecs::Entity;
use std::collections::HashMap;
use std::fmt::Display;

///////////////////////////////////////////////////

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Index(usize),
    Number(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<Key, Value>),
    Error,
    U64(u64),

    Entity(Entity),
}

impl TryInto<usize> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v),
            Value::Number(v) => Ok(v as usize),
            Value::Float(v) => Ok(v.floor() as usize),
            Value::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<usize> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v),
            Value::Number(v) => Ok(*v as usize),
            Value::Float(v) => Ok((*v).floor() as usize),
            Value::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as i32),
            Value::Number(v) => Ok(v),
            Value::Float(v) => Ok(v.floor() as i32),
            Value::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v as i32),
            Value::Number(v) => Ok(*v),
            Value::Float(v) => Ok((*v).floor() as i32),
            Value::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as u32),
            Value::Number(v) => match v >= 0 {
                true => Ok(v as u32),
                false => Err(DataConvertError::Negative),
            },
            Value::Float(v) => match v >= 0.0 {
                true => Ok(v.floor() as u32),
                false => Err(DataConvertError::Negative),
            },
            Value::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v as u32),
            Value::Number(v) => match *v >= 0 {
                true => Ok(*v as u32),
                false => Err(DataConvertError::Negative),
            },
            Value::Float(v) => match *v >= 0.0 {
                true => Ok(v.floor() as u32),
                false => Err(DataConvertError::Negative),
            },
            Value::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match *v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u64> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as u64),
            Value::Number(v) => match v >= 0 {
                true => Ok(v as u64),
                false => Err(DataConvertError::Negative),
            },
            Value::Float(v) => match v >= 0.0 {
                true => Ok(v.floor() as u64),
                false => Err(DataConvertError::Negative),
            },
            Value::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => Ok(v),

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u64> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v as u64),
            Value::Number(v) => match *v >= 0 {
                true => Ok(*v as u64),
                false => Err(DataConvertError::Negative),
            },
            Value::Float(v) => match *v >= 0.0 {
                true => Ok(v.floor() as u64),
                false => Err(DataConvertError::Negative),
            },
            Value::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            Value::U64(v) => Ok(*v),

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<f32> for Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(v as f32),
            Value::Number(v) => Ok(v as f32),
            Value::Float(v) => Ok(v),
            Value::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            Value::U64(v) => Ok(v as f32),

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<f32> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f32, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v as f32),
            Value::Number(v) => Ok(*v as f32),
            Value::Float(v) => Ok(*v),
            Value::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            Value::U64(v) => Ok(*v as f32),

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
            Value::Number(v) => Ok(v != 0),
            Value::Float(v) => Ok(v != 0.0),
            Value::Text(v) => Ok(v.len() > 0),
            Value::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            Value::U64(v) => Ok(v != 0),
            // Value::Blank => Ok(false),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<bool> for &Value {
    type Error = DataConvertError;

    fn try_into(self) -> Result<bool, DataConvertError> {
        match self {
            Value::Index(v) => Ok(*v != 0),
            Value::Number(v) => Ok(*v != 0),
            Value::Float(v) => Ok(*v != 0.0),
            Value::Text(v) => Ok(v.len() > 0),
            Value::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            Value::U64(v) => Ok(*v != 0),
            // Value::Blank => Ok(false),
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Index(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Text(v) => write!(f, "{}", v),
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
            Value::U64(v) => write!(f, "{}", v),

            Value::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
            Value::Error => write!(f, "!ERROR!"),
        }
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Value::Index(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Number(v)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::U64(v as u64)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::U64(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_owned())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::Text(v.clone())
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

impl From<Key> for Value {
    fn from(value: Key) -> Self {
        match value {
            Key::Index(v) => Value::Index(v),
            Key::Number(v) => Value::Number(v),
            Key::Text(v) => Value::Text(v),
            Key::U64(v) => Value::Number(v as i32),

            Key::Entity(v) => Value::Entity(v),
        }
    }
}

impl From<Entity> for Value {
    fn from(value: Entity) -> Self {
        Value::Entity(value)
    }
}
