use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered an unknown file type.")]
    UnknownFileType,
}
