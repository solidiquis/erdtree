use super::config::toml::error::Error as TomlError;
use clap::Error as ClapError;
use ignore::Error as IgnoreError;
use regex::Error as RegexError;
use std::convert::From;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ArgParse(ClapError),

    #[error("A configuration file was found but failed to parse: {0}")]
    Config(ClapError),

    #[error("No glob was provided")]
    EmptyGlob,

    #[error("{0}")]
    IgnoreError(#[from] IgnoreError),

    #[error("{0}")]
    InvalidRegularExpression(#[from] RegexError),

    #[error("Missing '--pattern' argument")]
    PatternNotProvided,

    #[error("{0}")]
    ConfigError(TomlError),
}

impl From<TomlError> for Error {
    fn from(value: TomlError) -> Self {
        Self::ConfigError(value)
    }
}
