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
    PathCanonicalization(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirNotFound(e) => write!(f, "{e}"),
            Self::ExpectedParent => write!(f, "File expected to have parent"),
            Self::InvalidGlobPatterns(e) => write!(f, "Invalid glob patterns: {e}"),
            Self::MissingRoot => write!(f, "Failed to compute root node"),
            Self::PathCanonicalization(e) => write!(f, "{e}"),
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
        Self::PathCanonicalization(value)
    }
}
