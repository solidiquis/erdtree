use crate::render::context;
use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

/// Errors that may occur during filesystem traversal.
#[derive(Debug)]
pub enum Error {
    CliError(context::Error),
    ExpectedParent,
    MissingRoot,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::CliError(e) => write!(f, "{e}"),
            Error::ExpectedParent => write!(f, "File expected to have parent"),
            Error::MissingRoot => write!(f, "Failed to compute root node"),
        }
    }
}

impl StdError for Error {}

impl From<context::Error> for Error {
    fn from(e: context::Error) -> Self {
        Self::CliError(e)
    }
}
