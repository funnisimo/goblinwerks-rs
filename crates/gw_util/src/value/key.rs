use crate::point::Point;

use super::DataConvertError;
use super::Value;
use legion::Entity;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Index(usize),
    Integer(i64),
    String(String),
    Point(i32, i32),

    Entity(Entity),
}

impl Key {
    pub fn type_name(&self) -> &'static str {
        match self {
            Key::Entity(_) => "Entity",
            Key::Index(_) => "usize",
            Key::Integer(_) => "int",
            Key::String(_) => "String",
            Key::Point(_, _) => "Point",
        }
    }

    pub fn is_entity(&self) -> bool {
        match self {
            Key::Entity(_) => true,
            _ => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            Key::Index(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Key::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_point(&self) -> bool {
        match self {
            Key::Point(_, _) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Key::String(_) => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Key::Index(v) => match (*v).try_into() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Key::Integer(v) => match (*v).try_into() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Key::String(v) => match v.parse::<i64>() {
                Err(_) => None,
                Ok(v) => Some(v),
            },

            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Key::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_point(&self) -> Option<Point> {
        match self {
            Key::Point(x, y) => Some(Point::new(*x, *y)),
            _ => None,
        }
    }

    pub fn as_entity(&self) -> Option<&Entity> {
        match self {
            Key::Entity(e) => Some(e),
            _ => None,
        }
    }

    pub fn to_entity(self) -> Option<Entity> {
        match self {
            Key::Entity(e) => Some(e),
            _ => None,
        }
    }
}

impl TryInto<usize> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v),
            Key::Integer(v) => Ok(v as usize),
            Key::String(v) => match v.parse::<usize>() {
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
            Key::Integer(v) => match v.try_into() {
                Ok(v) => Ok(v),
                Err(_) => Err(DataConvertError::WrongType),
            },
            Key::String(v) => match v.parse::<u64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<u64> for Key {
    fn eq(&self, other: &u64) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v == *other as i64,
        }
    }
}

// i64
impl TryInto<i64> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i64, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i64),
            Key::Integer(v) => Ok(v as i64),
            Key::String(v) => match v.parse::<i64>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<i64> for Key {
    fn eq(&self, other: &i64) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as i64 == *other,
        }
    }
}

// u32
impl TryInto<u32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u32),
            Key::Integer(v) => Ok(v as u32),
            Key::String(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<u32> for Key {
    fn eq(&self, other: &u32) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as u32 == *other,
        }
    }
}

// i32
impl TryInto<i32> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i32),
            Key::Integer(v) => Ok(v as i32),
            Key::String(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<i32> for Key {
    fn eq(&self, other: &i32) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as i32 == *other,
        }
    }
}

// u16
impl TryInto<u16> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u16, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u16),
            Key::Integer(v) => Ok(v as u16),
            Key::String(v) => match v.parse::<u16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<u16> for Key {
    fn eq(&self, other: &u16) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as u16 == *other,
        }
    }
}

// i16
impl TryInto<i16> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i16, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i16),
            Key::Integer(v) => Ok(v as i16),
            Key::String(v) => match v.parse::<i16>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<i16> for Key {
    fn eq(&self, other: &i16) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as i16 == *other,
        }
    }
}

// u8
impl TryInto<u8> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u8, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as u8),
            Key::Integer(v) => Ok(v as u8),
            Key::String(v) => match v.parse::<u8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<u8> for Key {
    fn eq(&self, other: &u8) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as u8 == *other,
        }
    }
}

// i8

impl TryInto<i8> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i8, DataConvertError> {
        match self {
            Key::Index(v) => Ok(v as i8),
            Key::Integer(v) => Ok(v as i8),
            Key::String(v) => match v.parse::<i8>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },

            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<i8> for Key {
    fn eq(&self, other: &i8) -> bool {
        match self.as_int() {
            None => false,
            Some(v) => v as i8 == *other,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Index(v) => write!(f, "{}", v),
            Key::Integer(v) => write!(f, "{}", v),
            Key::String(v) => write!(f, "{}", v),
            Key::Point(x, y) => write!(f, "({},{})", x, y),

            Key::Entity(entity) => {
                write!(f, "{:?}", entity)
            }
        }
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        match self {
            Key::String(v) => v == other,
            _ => false,
        }
    }
}

impl TryInto<Entity> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Entity, DataConvertError> {
        match self {
            Key::Entity(v) => Ok(v),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<Entity> for Key {
    fn eq(&self, other: &Entity) -> bool {
        match self {
            Key::Entity(e) => e == other,
            _ => false,
        }
    }
}

impl TryInto<Point> for Key {
    type Error = DataConvertError;

    fn try_into(self) -> Result<Point, DataConvertError> {
        match self {
            Key::Point(x, y) => Ok(Point::new(x, y)),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl PartialEq<Point> for Key {
    fn eq(&self, other: &Point) -> bool {
        match self {
            Key::Point(x, y) => other.x == *x && other.y == *y,
            _ => false,
        }
    }
}

impl TryFrom<Value> for Key {
    type Error = DataConvertError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Index(v) => Ok(Key::Index(v)),
            Value::Integer(v) => Ok(Key::Integer(v)),
            Value::String(v) => Ok(Key::String(v)),
            Value::Point(x, y) => Ok(Key::Point(x, y)),

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
        Key::Integer(v as i64)
    }
}

// i64
impl From<i64> for Key {
    fn from(v: i64) -> Self {
        Key::Integer(v as i64)
    }
}

// u32
impl From<u32> for Key {
    fn from(v: u32) -> Self {
        Key::Integer(v as i64)
    }
}

// i32
impl From<i32> for Key {
    fn from(v: i32) -> Self {
        Key::Integer(v as i64)
    }
}

// u16
impl From<u16> for Key {
    fn from(v: u16) -> Self {
        Key::Integer(v as i64)
    }
}

// i16
impl From<i16> for Key {
    fn from(v: i16) -> Self {
        Key::Integer(v as i64)
    }
}

// u8
impl From<u8> for Key {
    fn from(v: u8) -> Self {
        Key::Integer(v as i64)
    }
}

// i8
impl From<i8> for Key {
    fn from(v: i8) -> Self {
        Key::Integer(v as i64)
    }
}

impl From<&str> for Key {
    fn from(v: &str) -> Self {
        Key::String(v.to_owned())
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Key::String(v)
    }
}

impl From<&String> for Key {
    fn from(v: &String) -> Self {
        Key::String(v.clone())
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

impl From<Point> for Key {
    fn from(value: Point) -> Self {
        Key::Point(value.x, value.y)
    }
}

impl From<&Point> for Key {
    fn from(value: &Point) -> Self {
        Key::Point(value.x, value.y)
    }
}
