use super::DataConvertError;
use super::Value;
use crate::ecs::Entity;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Index(usize),
    Number(i32),
    Text(String),
    U64(u64),

    Entity(Entity),
}

impl TryInto<usize> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v),
            Key::Number(v) => Ok(v as usize),
            Key::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<usize> for &Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Key::Index(v) => Ok(*v),
            Key::Number(v) => Ok(*v as usize),
            Key::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i32),
            Key::Number(v) => Ok(v),
            Key::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for &Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(*v as i32),
            Key::Number(v) => Ok(*v),
            Key::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u32),
            Key::Number(v) => match v >= 0 {
                true => Ok(v as u32),
                false => Err(DataConvertError::Negative),
            },
            Key::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for &Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(*v as u32),
            Key::Number(v) => match *v >= 0 {
                true => Ok(*v as u32),
                false => Err(DataConvertError::Negative),
            },
            Key::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => match (*v).try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u64> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u64),
            Key::Number(v) => match v >= 0 {
                true => Ok(v as u64),
                false => Err(DataConvertError::Negative),
            },
            Key::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => Ok(v),

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u64> for &Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Key::Index(v) => Ok(*v as u64),
            Key::Number(v) => match *v >= 0 {
                true => Ok(*v as u64),
                false => Err(DataConvertError::Negative),
            },
            Key::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            Key::U64(v) => Ok(*v),

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Index(v) => write!(f, "{}", v),
            Key::Number(v) => write!(f, "{}", v),
            Key::Text(v) => write!(f, "{}", v),
            Key::U64(v) => write!(f, "{}", v),

            Key::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
        }
    }
}

impl TryInto<Entity> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Entity, DataConvertError> {
        match self {
            Key::Entity(v) => Ok(v),
            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryFrom<Value> for Key {
    type Error = DataConvertError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Index(v) => Ok(Key::Index(v)),
            Value::Number(v) => Ok(Key::Number(v)),
            Value::Text(v) => Ok(Key::Text(v)),

            Value::Entity(v) => Ok(Key::Entity(v)),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl From<usize> for Key {
    fn from(v: usize) -> Self {
        Key::Index(v)
    }
}

impl From<i32> for Key {
    fn from(v: i32) -> Self {
        Key::Number(v)
    }
}

impl From<u32> for Key {
    fn from(v: u32) -> Self {
        Key::U64(v as u64)
    }
}

impl From<u64> for Key {
    fn from(v: u64) -> Self {
        Key::U64(v)
    }
}

impl From<&str> for Key {
    fn from(v: &str) -> Self {
        Key::Text(v.to_owned())
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Key::Text(v)
    }
}

impl From<&String> for Key {
    fn from(v: &String) -> Self {
        Key::Text(v.clone())
    }
}

impl From<Entity> for Key {
    fn from(v: Entity) -> Self {
        Key::Entity(v)
    }
}

impl From<&Entity> for Key {
    fn from(v: &Entity) -> Self {
        Key::Entity(v.clone())
    }
}
