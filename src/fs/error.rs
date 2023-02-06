use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

/// Errors that may occur during filesystem traversal.
#[derive(Debug)]
pub enum Error {
    ExpectedParent,
    MissingRoot,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ExpectedParent => write!(f, "File expected to have parent"),
            Error::MissingRoot => write!(f, "Failed to compute root node"),
        }
    }
}

impl StdError for Error {}
