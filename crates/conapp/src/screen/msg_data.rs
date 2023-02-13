#[cfg(feature = "ecs")]
use bevy_ecs::prelude::Entity;
use std::fmt::Display;

#[derive(Debug)]
pub enum DataConvertError {
    WrongType,
    Negative,
}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MsgData {
    Index(usize),
    Number(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    List(Vec<MsgData>),
    Error,
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
            #[cfg(feature = "ecs")]
            MsgData::Entity(entity) => {
                write!(f, "{:?}", entity)
            }

            // Value::Blank => Ok("".to_owned()),
            MsgData::Error => write!(f, "!ERROR!"),
        }
    }
}

impl From<i32> for MsgData {
    fn from(v: i32) -> Self {
        MsgData::Number(v)
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

#[cfg(feature = "ecs")]
impl From<Entity> for MsgData {
    fn from(value: Entity) -> Self {
        MsgData::Entity(value)
    }
}
