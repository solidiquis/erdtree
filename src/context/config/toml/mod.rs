use config::{Config, File, Value, ValueKind};
use error::Error;
use std::{env, ffi::OsString};

/// Errors associated with loading and parsing the toml config file.
pub mod error;

/// Testing related to `.erdtree.toml`.
pub mod test;

/// Represents an instruction on how to handle a single key-value pair, which makes up a single
/// command-line argument, when constructing the arguments vector.
enum ArgInstructions {
    /// Used for bool arguments such as `--icons`. When `icons = true` is set in `.erdtree.toml`,
    /// we only want `--icons` to be pushed into the ultimate arguments vector.
    PushKeyOnly,

    /// Used for arguments such as `--threads 10`.
    PushKeyValue { parsed_value: OsString },

    /// If a bool field is set to false in `.erdtree.toml` (e.g. `icons = false`) then we want to
    /// completely omit the key-value pair from the arguments that we ultimately use.
    Pass,
}

/// Takes in a `Config` that is generated from [`load`] returning a `Vec<OsString>` which
/// represents command-line arguments from `.erdtree.toml`. If a `named_table` is provided then
/// the top-level table in `.erdtree.toml` is ignored and the configurations specified in the
/// `named_table` will be used instead.
pub fn parse(config: Config, named_table: Option<&str>) -> Result<Vec<OsString>, Error> {
    let mut args_map = config.cache.into_table()?;

    if let Some(table) = named_table {
        let new_conf = args_map
            .get(table)
            .and_then(|conf| conf.clone().into_table().ok())
            .ok_or_else(|| Error::MissingAltConfig(table.to_owned()))?;

        args_map = new_conf;
    } else {
        args_map.retain(|_k, v| !matches!(v.kind, ValueKind::Table(_)));
    }

    let mut parsed_args = vec![OsString::from("--")];

    let process_key = |s| OsString::from(format!("--{s}").replace('_', "-"));

    for (k, v) in &args_map {
        match parse_argument(k, v)? {
            ArgInstructions::PushKeyValue { parsed_value } => {
                let fmt_key = process_key(k);
                parsed_args.push(fmt_key);
                parsed_args.push(parsed_value);
            }

            ArgInstructions::PushKeyOnly => {
                let fmt_key = process_key(k);
                parsed_args.push(fmt_key);
            }

            ArgInstructions::Pass => continue,
        }
    }

    Ok(parsed_args)
}

/// Reads in `.erdtree.toml` file.
pub fn load() -> Result<Config, Error> {
    #[cfg(windows)]
    return windows::load_toml().ok_or(Error::LoadConfig);

    #[cfg(unix)]
    unix::load_toml().ok_or(Error::LoadConfig)
}

/// Attempts to load in `.erdtree.toml` from `$ERDTREE_TOML_PATH`. Will return `None` for whatever
/// reason.
fn toml_from_env() -> Option<Config> {
    let config = env::var_os(super::ERDTREE_TOML_PATH)
        .map(OsString::into_string)
        .and_then(Result::ok)?;

    let file = config.strip_suffix(".toml").map(File::with_name)?;

    Config::builder().add_source(file).build().ok()
}

/// Simple utility used to extract the underlying value from the [`Value`] enum that we get when
/// loading in the values from `.erdtree.toml`, returning instructions on how the argument should
/// be processed into the ultimate arguments vector.
fn parse_argument(keyword: &str, arg: &Value) -> Result<ArgInstructions, Error> {
    macro_rules! try_parse_num {
        ($n:expr) => {
            usize::try_from($n)
                .map_err(|_e| Error::InvalidInteger(keyword.to_owned()))
                .map(|num| {
                    let parsed = OsString::from(format!("{num}"));
                    ArgInstructions::PushKeyValue {
                        parsed_value: parsed,
                    }
                })
        };
    }

    match &arg.kind {
        ValueKind::Boolean(val) => {
            if *val {
                Ok(ArgInstructions::PushKeyOnly)
            } else {
                Ok(ArgInstructions::Pass)
            }
        }
        ValueKind::String(val) => Ok(ArgInstructions::PushKeyValue {
            parsed_value: OsString::from(val),
        }),
        ValueKind::I64(val) => try_parse_num!(*val),
        ValueKind::I128(val) => try_parse_num!(*val),
        ValueKind::U64(val) => try_parse_num!(*val),
        ValueKind::U128(val) => try_parse_num!(*val),
        _ => Err(Error::InvalidArgument(keyword.to_owned())),
    }
}

/// Concerned with how to load `.erdtree.toml` on Unix systems.
#[cfg(unix)]
mod unix {
    use super::super::{CONFIG_DIR, ERDTREE_CONFIG_TOML, ERDTREE_DIR, HOME, XDG_CONFIG_HOME};
    use config::{Config, File};
    use std::{env, path::PathBuf};

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$ERDTREE_TOML_PATH`
    /// - `$XDG_CONFIG_HOME/erdtree/.erdtree.toml`
    /// - `$XDG_CONFIG_HOME/.erdtree.toml`
    /// - `$HOME/.config/erdtree/.erdtree.toml`
    /// - `$HOME/.erdtree.toml`
    pub(super) fn load_toml() -> Option<Config> {
        super::toml_from_env()
            .or_else(toml_from_xdg_path)
            .or_else(toml_from_home)
    }

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$XDG_CONFIG_HOME/erdtree/.erdtree.toml`
    /// - `$XDG_CONFIG_HOME/.erdtree.toml`
    fn toml_from_xdg_path() -> Option<Config> {
        let config = env::var_os(XDG_CONFIG_HOME).map(PathBuf::from)?;

        let mut file = config
            .join(ERDTREE_DIR)
            .join(ERDTREE_CONFIG_TOML)
            .to_str()
            .and_then(|s| s.strip_suffix(".toml"))
            .map(File::with_name);

        if file.is_none() {
            file = config
                .join(ERDTREE_CONFIG_TOML)
                .to_str()
                .and_then(|s| s.strip_suffix(".toml"))
                .map(File::with_name);
        }

        Config::builder().add_source(file?).build().ok()
    }

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$HOME/.config/erdtree/.erdtree.toml`
    /// - `$HOME/.erdtree.toml`
    fn toml_from_home() -> Option<Config> {
        let home = env::var_os(HOME).map(PathBuf::from)?;

        let mut file = home
            .join(CONFIG_DIR)
            .join(ERDTREE_DIR)
            .join(ERDTREE_CONFIG_TOML)
            .to_str()
            .and_then(|s| s.strip_suffix(".toml"))
            .map(File::with_name);

        if file.is_none() {
            file = home
                .join(ERDTREE_CONFIG_TOML)
                .to_str()
                .and_then(|s| s.strip_suffix(".toml"))
                .map(File::with_name);
        }

        Config::builder().add_source(file?).build().ok()
    }
}

/// Concerned with how to load `.erdtree.toml` on Windows.
#[cfg(windows)]
mod windows {
    use super::super::{ERDTREE_CONFIG_TOML, ERDTREE_DIR};
    use config::{Config, File};

    /// Try to read in config from the following location:
    /// - `%APPDATA%\erdtree\.erdtree.toml`
    pub(super) fn load_toml() -> Option<Config> {
        super::toml_from_env().or_else(toml_from_appdata)
    }

    /// Try to read in config from the following location:
    /// - `%APPDATA%\erdtree\.erdtree.toml`
    fn toml_from_appdata() -> Option<Config> {
        let app_data = dirs::config_dir()?;

        let file = app_data
            .join(ERDTREE_DIR)
            .join(ERDTREE_CONFIG_TOML)
            .to_str()
            .and_then(|s| s.strip_prefix(".toml"))
            .map(File::with_name)?;

        Config::builder().add_source(file).build().ok()
    }
}
