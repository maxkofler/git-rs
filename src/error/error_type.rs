use std::{fmt::Display, io};

use super::ErrorIn;

/// All the possible types of errors that can occur within this library
#[derive(Debug)]
#[repr(u16)]
pub enum ErrorType {
    /// An IO error
    IO(io::Error),
    /// An error when parsing hex strings
    FromHex(hex::FromHexError),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => e.fmt(f),
            Self::FromHex(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
impl ErrorIn for io::Error {}

impl From<hex::FromHexError> for ErrorType {
    fn from(value: hex::FromHexError) -> Self {
        Self::FromHex(value)
    }
}
impl ErrorIn for hex::FromHexError {}
