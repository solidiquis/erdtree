use super::config::toml::error::Error as TomlError;
use clap::{parser::MatchesError, Error as ClapError};
use ignore::Error as IgnoreError;
use regex::Error as RegexError;

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
    ConfigError(#[from] TomlError),

    #[error("{0}")]
    MatchError(#[from] MatchesError),

    #[error("'--config' was specified but a `.erdtree.toml` file could not be found")]
    NoToml,

    #[error("Please migrate from `erdtreerc` to `.erdtree.toml` to make use of `--config`")]
    Rc,
}
