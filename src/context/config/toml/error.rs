use config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load .erdtree.toml")]
    LoadConfig,

    #[error("The configuration file is improperly formatted")]
    InvalidFormat(#[from] ConfigError),

    #[error("Alternate configuration '{0}' was not found in '.erdtree.toml'")]
    MissingAltConfig(String),

    #[error("'#{0}' is required to be a pointer-sized unsigned integer type")]
    InvalidInteger(String),

    #[error("'#{0}' has a type that is invalid")]
    InvalidArgument(String),
}
