use crate::cli;
use std::{
    convert::From,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

/// Errors that may occur during filesystem traversal.
#[derive(Debug)]
pub enum Error {
    CliError(cli::Error),
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

impl From<cli::Error> for Error {
    fn from(e: cli::Error) -> Self {
        Self::CliError(e)
    }
}
