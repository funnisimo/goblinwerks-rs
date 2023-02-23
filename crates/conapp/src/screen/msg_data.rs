#[cfg(feature = "ecs")]
use bevy_ecs::prelude::Entity;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub enum DataConvertError {
    WrongType,
    Negative,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Index(usize),
    Number(i32),
    Text(String),
    U64(u64),
    #[cfg(feature = "ecs")]
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

            #[cfg(feature = "ecs")]
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

            #[cfg(feature = "ecs")]
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

            #[cfg(feature = "ecs")]
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

            #[cfg(feature = "ecs")]
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

            #[cfg(feature = "ecs")]
            Key::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
        }
    }
}

#[cfg(feature = "ecs")]
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

impl TryFrom<MsgData> for Key {
    type Error = DataConvertError;

    fn try_from(value: MsgData) -> Result<Self, Self::Error> {
        match value {
            MsgData::Index(v) => Ok(Key::Index(v)),
            MsgData::Number(v) => Ok(Key::Number(v)),
            MsgData::Text(v) => Ok(Key::Text(v)),
            #[cfg(feature = "ecs")]
            MsgData::Entity(v) => Ok(Key::Entity(v)),
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

#[cfg(feature = "ecs")]
impl From<Entity> for Key {
    fn from(v: Entity) -> Self {
        Key::Entity(v)
    }
}

#[cfg(feature = "ecs")]
impl From<&Entity> for Key {
    fn from(v: &Entity) -> Self {
        Key::Entity(v.clone())
    }
}

///////////////////////////////////////////////////

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MsgData {
    Index(usize),
    Number(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    List(Vec<MsgData>),
    Map(HashMap<Key, MsgData>),
    Error,
    U64(u64),
    #[cfg(feature = "ecs")]
    Entity(Entity),
}

impl TryInto<usize> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v),
            MsgData::Number(v) => Ok(v as usize),
            MsgData::Float(v) => Ok(v.floor() as usize),
            MsgData::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            MsgData::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as i32),
            MsgData::Number(v) => Ok(v),
            MsgData::Float(v) => Ok(v.floor() as i32),
            MsgData::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            MsgData::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as u32),
            MsgData::Number(v) => match v >= 0 {
                true => Ok(v as u32),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Float(v) => match v >= 0.0 {
                true => Ok(v.floor() as u32),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            MsgData::U64(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u64> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u64, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as u64),
            MsgData::Number(v) => match v >= 0 {
                true => Ok(v as u64),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Float(v) => match v >= 0.0 {
                true => Ok(v.floor() as u64),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Text(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            MsgData::U64(v) => Ok(v),

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<f32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as f32),
            MsgData::Number(v) => Ok(v as f32),
            MsgData::Float(v) => Ok(v),
            MsgData::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            MsgData::U64(v) => Ok(v as f32),

            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<bool> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<bool, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v != 0),
            MsgData::Number(v) => Ok(v != 0),
            MsgData::Float(v) => Ok(v != 0.0),
            MsgData::Text(v) => Ok(v.len() > 0),
            MsgData::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            MsgData::U64(v) => Ok(v != 0),
            // Value::Blank => Ok(false),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

#[cfg(feature = "ecs")]
impl TryInto<Entity> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Entity, DataConvertError> {
        match self {
            MsgData::Entity(e) => Ok(e),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl Display for MsgData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgData::Index(v) => write!(f, "{}", v),
            MsgData::Number(v) => write!(f, "{}", v),
            MsgData::Float(v) => write!(f, "{}", v),
            MsgData::Text(v) => write!(f, "{}", v),
            MsgData::Boolean(v) => match v {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            MsgData::List(data) => {
                write!(f, "[")?;
                for (i, item) in data.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            MsgData::Map(data) => {
                write!(f, "{{")?;
                for (key, val) in data.iter() {
                    write!(f, "{:?}={}, ", key, val)?;
                }
                write!(f, "}}")
            }
            MsgData::U64(v) => write!(f, "{}", v),
            #[cfg(feature = "ecs")]
            MsgData::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
            MsgData::Error => write!(f, "!ERROR!"),
        }
    }
}

impl From<usize> for MsgData {
    fn from(v: usize) -> Self {
        MsgData::Index(v)
    }
}

impl From<i32> for MsgData {
    fn from(v: i32) -> Self {
        MsgData::Number(v)
    }
}

impl From<u64> for MsgData {
    fn from(v: u64) -> Self {
        MsgData::U64(v)
    }
}

impl From<f32> for MsgData {
    fn from(v: f32) -> Self {
        MsgData::Float(v)
    }
}

impl From<&str> for MsgData {
    fn from(v: &str) -> Self {
        MsgData::Text(v.to_owned())
    }
}

impl From<String> for MsgData {
    fn from(v: String) -> Self {
        MsgData::Text(v)
    }
}

impl From<&String> for MsgData {
    fn from(v: &String) -> Self {
        MsgData::Text(v.clone())
    }
}

impl From<bool> for MsgData {
    fn from(v: bool) -> Self {
        MsgData::Boolean(v)
    }
}

impl From<Vec<MsgData>> for MsgData {
    fn from(vec: Vec<MsgData>) -> Self {
        MsgData::List(vec)
    }
}

impl From<HashMap<Key, MsgData>> for MsgData {
    fn from(data: HashMap<Key, MsgData>) -> Self {
        MsgData::Map(data)
    }
}

impl From<Key> for MsgData {
    fn from(value: Key) -> Self {
        match value {
            Key::Index(v) => MsgData::Index(v),
            Key::Number(v) => MsgData::Number(v),
            Key::Text(v) => MsgData::Text(v),
            Key::U64(v) => MsgData::Number(v as i32),
            #[cfg(feature = "ecs")]
            Key::Entity(v) => MsgData::Entity(v),
        }
    }
}

#[cfg(feature = "ecs")]
impl From<Entity> for MsgData {
    fn from(value: Entity) -> Self {
        MsgData::Entity(value)
    }
}
