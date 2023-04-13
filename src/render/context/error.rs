use clap::Error as ClapError;
use ignore::Error as IgnoreError;
use regex::Error as RegexError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ArgParse(#[source] ClapError),

    #[error("A configuration file was found but failed to parse: {0}")]
    Config(#[source] ClapError),

    #[error("{0}")]
    IgnoreError(#[from] IgnoreError),

    #[error("{0}")]
    InvalidRegularExpression(#[from] RegexError),

    #[error("Regular expressions search is disabled due to use of '--glob' or '--iglob'")]
    RegexDisabled,

    #[error("Missing '--pattern' argument")]
    PatternNotProvided,
}
