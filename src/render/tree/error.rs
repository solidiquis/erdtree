use super::styles::error::Error as StyleError;
use crate::{
    fs::permissions::error::Error as PermissionsError, render::context::error::Error as CtxError,
};
use ignore::Error as IgnoreError;
use std::io::Error as IoError;

/// Errors that may occur while traversing or construction of [`Tree`].
///
/// [`Tree`]: super::Tree
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Context(#[from] CtxError),

    #[error("{0}")]
    DirNotFound(String),

    #[error("File expected to have parent")]
    ExpectedParent,

    #[error("Invalid glob patterns: {0}")]
    InvalidGlobPatterns(#[from] IgnoreError),

    #[error("Failed to compute root node.")]
    MissingRoot,

    #[error("No entries to show with given arguments.")]
    NoMatches,

    #[error("{0}")]
    PathCanonicalization(#[from] IoError),

    #[error("{0}")]
    Persmissions(#[from] PermissionsError),

    #[error("{0}")]
    UninitializedTheme(#[from] StyleError<'static>),
}
