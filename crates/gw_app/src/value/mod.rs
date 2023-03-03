mod key;
pub use key::*;

mod value;
pub use value::*;

#[derive(Debug)]
pub enum DataConvertError {
    WrongType,
    Negative,
}
