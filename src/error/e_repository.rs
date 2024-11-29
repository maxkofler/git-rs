use std::fmt::Display;

use super::{ErrorIn, ErrorType};

/// An error that can happen when working with a repository
#[derive(Debug)]
pub enum RepositoryError {
    /// A repository was not found
    NotFound,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "No repository found"),
        }
    }
}

impl From<RepositoryError> for ErrorType {
    fn from(value: RepositoryError) -> Self {
        ErrorType::Repository(value)
    }
}
impl ErrorIn for RepositoryError {}
