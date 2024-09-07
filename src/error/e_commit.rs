use std::fmt::Display;

use super::{ErrorIn, ErrorType};

/// An error that can happen when working with a commit
#[derive(Debug)]
pub enum CommitError {
    /// The (parsed) commit is missing an attached tree
    MissingTree,
    /// The (parsed) commit is missing an author
    MissingAuthor,
    /// THe (parsed) commit is missing a committer
    MissingCommitter,
}

impl Display for CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTree => write!(f, "The commit is missing a tree"),
            Self::MissingAuthor => write!(f, "The commit is missing an author"),
            Self::MissingCommitter => write!(f, "The commit is missing a committer"),
        }
    }
}

impl From<CommitError> for ErrorType {
    fn from(value: CommitError) -> Self {
        ErrorType::Commit(value)
    }
}
impl ErrorIn for CommitError {}
