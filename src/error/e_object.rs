use std::fmt::Display;

use super::{ErrorIn, ErrorType};

/// An error that can happen when working with a commit
#[derive(Debug)]
pub enum ObjectError {
    UnknownType(String),
}

impl Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownType(ty) => write!(f, "Unknown object type {:?}", ty),
        }
    }
}

impl From<ObjectError> for ErrorType {
    fn from(value: ObjectError) -> Self {
        ErrorType::Object(value)
    }
}
impl ErrorIn for ObjectError {}
