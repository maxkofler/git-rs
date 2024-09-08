use super::{CommitError, ObjectError};

use super::{Error, Result};
use std::{fmt::Display, io, string::FromUtf8Error};

use super::ErrorIn;

/// All the possible types of errors that can occur within this library
#[derive(Debug)]
#[repr(u16)]
pub enum ErrorType {
    /// An IO error
    IO(io::Error),
    /// An error when parsing hex strings
    FromHex(hex::FromHexError),
    /// An error when parsing UTF-8 strings
    FromUtf8(FromUtf8Error),
    /// General parsing errors
    Parse(ParsingError),
    /// An error while working with a commit
    Commit(CommitError),
    /// An error while working with an object
    Object(ObjectError),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => e.fmt(f),
            Self::FromHex(e) => e.fmt(f),
            Self::FromUtf8(e) => e.fmt(f),
            Self::Parse(e) => e.fmt(f),
            Self::Commit(e) => e.fmt(f),
            Self::Object(e) => e.fmt(f),
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

impl From<FromUtf8Error> for ErrorType {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}
impl ErrorIn for FromUtf8Error {}

/// A error that can happen while parsing something
#[derive(Debug)]
pub enum ParsingError {
    /// Failed to split `String` at `char`
    Split(String, char),
    /// Failed to parse `String` to a KVLM
    KVLM(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Split(string, c) => write!(f, "Failed to split '{string}' at '{c}'"),
            Self::KVLM(string) => write!(f, "Failed to parse '{string}' to a KVLM"),
        }
    }
}

impl ParsingError {
    pub fn throw(self) -> Result<()> {
        Err(Error::new(ErrorType::Parse(self)))
    }
}
impl From<ParsingError> for ErrorType {
    fn from(value: ParsingError) -> Self {
        Self::Parse(value)
    }
}
impl ErrorIn for ParsingError {}
