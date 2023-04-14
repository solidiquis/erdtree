use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown file type.")]
    UnknownFileType,

    #[error("Failed to compute file mode.")]
    UnknownMode,
}
