use config::{Config, File};
use std::{env, ffi::OsStr, path::PathBuf};

/// Reads in `.erdtree.toml` file.
#[cfg(windows)]
pub fn load() -> Option<Config> {
    windows::load_toml()
}

#[cfg(not(any(windows, unix)))]
pub fn load() -> Option<Config> {
    None
}

/// Reads in `.erdtree.toml` file.
#[cfg(unix)]
pub fn load() -> Option<Config> {
    unix::load_toml()
}

/// Attempts to load in `.erdtree.toml` from `$ERDTREE_TOML_PATH`.
fn toml_from_env() -> Option<Config> {
    let path = match env::var_os(super::ERDTREE_TOML_PATH).map(PathBuf::from) {
        Some(config_path) => config_path,
        None => return None,
    };

    let file = path
        .file_stem()
        .and_then(OsStr::to_str)
        .map(File::with_name)?;

    Config::builder().add_source(file).build().ok()
}

/// Concerned with how to load `.erdtree.toml` on Unix systems.
#[cfg(unix)]
mod unix {
    use super::super::{
        CONFIG_DIR, ERDTREE_CONFIG_FILE, ERDTREE_CONFIG_TOML, ERDTREE_DIR, HOME, XDG_CONFIG_HOME,
    };
    use config::{Config, File};
    use std::{env, ffi::OsStr, path::PathBuf};

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$XDG_CONFIG_HOME/erdtree/.erdtree.toml`
    /// - `$XDG_CONFIG_HOME/.erdtree.toml`
    /// - `$HOME/.config/erdtree/.erdtree.toml`
    /// - `$HOME/.erdtree.toml`
    pub fn load_toml() -> Option<Config> {
        super::toml_from_env()
            .or_else(toml_from_xdg_path)
            .or_else(toml_from_home)
    }

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$XDG_CONFIG_HOME/erdtree/.erdtree.toml`
    /// - `$XDG_CONFIG_HOME/.erdtree.toml`
    fn toml_from_xdg_path() -> Option<Config> {
        let path = match env::var_os(XDG_CONFIG_HOME).map(PathBuf::from) {
            Some(dir_path) => dir_path,
            None => return None,
        };

        let file = path
            .join(ERDTREE_DIR)
            .join(ERDTREE_CONFIG_TOML)
            .file_stem()
            .and_then(OsStr::to_str)
            .map(File::with_name)?;

        if let Ok(config) = Config::builder().add_source(file).build() {
            return Some(config);
        }

        let file = path
            .join(ERDTREE_CONFIG_TOML)
            .file_stem()
            .and_then(OsStr::to_str)
            .map(File::with_name)?;

        Config::builder().add_source(file).build().ok()
    }

    /// Looks for `.erdtree.toml` in the following locations in order:
    ///
    /// - `$HOME/.config/erdtree/.erdtree.toml`
    /// - `$HOME/.erdtree.toml`
    fn toml_from_home() -> Option<Config> {
        let home = match env::var_os(HOME).map(PathBuf::from) {
            Some(path) => path,
            None => return None, // Why don't you have `HOME` set? Weirdo.
        };

        let source = home
            .join(CONFIG_DIR)
            .join(ERDTREE_DIR)
            .join(ERDTREE_CONFIG_FILE)
            .to_str()
            .map(File::with_name)?;

        if let Ok(config) = Config::builder().add_source(source).build() {
            return Some(config);
        }

        let file = home
            .join(ERDTREE_CONFIG_TOML)
            .file_stem()
            .and_then(OsStr::to_str)
            .map(File::with_name)?;

        Config::builder().add_source(file).build().ok()
    }
}

/// Concerned with how to load `.erdtree.toml` on Windows.
#[cfg(windows)]
mod windows {
    use super::super::{ERDTREE_CONFIG_FILE, ERDTREE_DIR};
    use config::{Config, File};

    /// Try to read in config from the following location:
    /// - `%APPDATA%\erdtree\.erdtree.toml`
    pub fn load_toml() -> Option<Config> {
        super::toml_from_env().or_else(toml_from_appdata)
    }

    /// Try to read in config from the following location:
    /// - `%APPDATA%\erdtree\.erdtree.toml`
    fn toml_from_appdata() -> Option<Config> {
        let file = dirs::config_dir().and_then(|config_dir| {
            config_dir
                .join(ERDTREE_DIR)
                .join(ERDTREE_CONFIG_FILE)
                .to_str()
                .map(File::with_name)
        })?;

        Config::builder().add_source(file).build().ok()
    }
}
