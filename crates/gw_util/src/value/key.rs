use super::DataConvertError;
use super::Value;
use legion::Entity;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Index(usize),
    Number(u64),
    Text(String),

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

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u64

impl TryInto<u64> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            Key::Index(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Key::Number(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Key::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i64
impl TryInto<i64> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i64, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i64),
            Key::Number(v) => Ok(v as i64),
            Key::Text(v) => match v.parse::<i64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u32
impl TryInto<u32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u32),
            Key::Number(v) => Ok(v as u32),
            Key::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i32
impl TryInto<i32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i32),
            Key::Number(v) => Ok(v as i32),
            Key::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u16
impl TryInto<u16> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u16, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u16),
            Key::Number(v) => Ok(v as u16),
            Key::Text(v) => match v.parse::<u16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i16
impl TryInto<i16> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i16, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i16),
            Key::Number(v) => Ok(v as i16),
            Key::Text(v) => match v.parse::<i16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// u8
impl TryInto<u8> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u8, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u8),
            Key::Number(v) => Ok(v as u8),
            Key::Text(v) => match v.parse::<u8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

// i8

impl TryInto<i8> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i8, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i8),
            Key::Number(v) => Ok(v as i8),
            Key::Text(v) => match v.parse::<i8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

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

// u64
impl From<u64> for Key {
    fn from(v: u64) -> Self {
        Key::Number(v as u64)
    }
}

// i64
impl From<i64> for Key {
    fn from(v: i64) -> Self {
        Key::Number(v as u64)
    }
}

// u32
impl From<u32> for Key {
    fn from(v: u32) -> Self {
        Key::Number(v as u64)
    }
}

// i32
impl From<i32> for Key {
    fn from(v: i32) -> Self {
        Key::Number(v as u64)
    }
}

// u16
impl From<u16> for Key {
    fn from(v: u16) -> Self {
        Key::Number(v as u64)
    }
}

// i16
impl From<i16> for Key {
    fn from(v: i16) -> Self {
        Key::Number(v as u64)
    }
}

// u8
impl From<u8> for Key {
    fn from(v: u8) -> Self {
        Key::Number(v as u64)
    }
}

// i8
impl From<i8> for Key {
    fn from(v: i8) -> Self {
        Key::Number(v as u64)
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
