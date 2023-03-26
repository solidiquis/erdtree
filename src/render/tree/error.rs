use ignore::Error as IgnoreError;
use std::io::Error as IoError;

/// Errors that may occur while traversing or construction of [`Tree`].
///
/// [`Tree`]: super::Tree
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DirNotFound(String),

    #[error("File expected to have parent")]
    ExpectedParent,

    #[error("Invalid glob patterns: {0}")]
    InvalidGlobPatterns(#[from] IgnoreError),

    #[error("Failed to compute root node")]
    MissingRoot,

    #[error("{0}")]
    PathCanonicalization(#[from] IoError),
}
