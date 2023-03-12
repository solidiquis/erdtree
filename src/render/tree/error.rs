use ignore::Error as IgnoreError;
use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    io::Error as IoError,
};

/// Errors that may occur while traversing or construction of [`Tree`].
///
/// [`Tree`]: super::Tree
#[derive(Debug)]
pub enum Error {
    DirNotFound(String),
    ExpectedParent,
    InvalidGlobPatterns(IgnoreError),
    MissingRoot,
    PathCanonicalizationError(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::DirNotFound(e) => write!(f, "{e}"),
            Error::ExpectedParent => write!(f, "File expected to have parent"),
            Error::InvalidGlobPatterns(e) => write!(f, "Invalid glob patterns: {e}"),
            Error::MissingRoot => write!(f, "Failed to compute root node"),
            Error::PathCanonicalizationError(e) => write!(f, "{e}"),
        }
    }
}

impl StdError for Error {}

impl From<ignore::Error> for Error {
    fn from(value: ignore::Error) -> Self {
        Self::InvalidGlobPatterns(value)
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Self::PathCanonicalizationError(value)
    }
}
