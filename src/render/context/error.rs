use clap::Error as ClapError;
use ignore::Error as IgnoreError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ArgParse(#[source] ClapError),

    #[error("A configuration file was found but failed to parse: {0}")]
    Config(#[source] ClapError),

    #[error("{0}")]
    IgnoreError(#[from] IgnoreError),

    #[error("Missing '--pattern' argument")]
    PatternNotProvided,
}
