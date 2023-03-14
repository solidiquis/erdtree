use std::{
    env, fs,
    path::{Path, PathBuf},
};

const ERDTREE_CONFIG_NAME: &str = ".erdtreerc";
const ERDTREE_CONFIG_PATH: &str = "ERDTREE_CONFIG_PATH";
const ERDTREE_DIR: &str = "erdtree";
const CONFIG_DIR: &str = ".config";
const HOME: &str = "HOME";
const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

/// Reads the config file into a `String` if there is one. When `None` is provided then the config
/// is looked for in the following locations in order:
///
/// - `$ERDTREE_CONFIG_PATH`
/// - `$XDG_CONFIG_HOME/erdtree/.erdtreerc`
/// - `$XDG_CONFIG_HOME/.erdtreerc`
/// - `$HOME/.config/erdtree/.erdtreerc`
/// - `$HOME/.erdtreerc`
pub fn read_config_to_string<T: AsRef<Path>>(path: Option<T>) -> Option<String> {
    if let Some(p) = path {
        return fs::read_to_string(p).ok().map(prepend_arg_prefix);
    }

    config_from_config_path()
        .or_else(config_from_xdg_path)
        .or_else(config_from_home)
        .map(prepend_arg_prefix)
}

/// Parses the config `str`, removing comments and preparing it as a format understood by
/// [`get_matches_from`].
///
/// [`get_matches_from`]: clap::builder::Command::get_matches_from
pub fn parse_config<'a>(config: &'a str) -> Vec<&'a str> {
    config
        .lines()
        .filter(|line| {
            line.trim_start()
                .chars()
                .nth(0)
                .map(|ch| ch != '#')
                .unwrap_or(true)
        })
        .map(str::split_ascii_whitespace)
        .flatten()
        .collect::<Vec<&'a str>>()
}

/// Try to read in config from `ERDTREE_CONFIG_PATH`.
fn config_from_config_path() -> Option<String> {
    env::var_os(ERDTREE_CONFIG_PATH)
        .map(PathBuf::from)
        .map(fs::read_to_string)
        .map(Result::ok)
        .flatten()
}

/// Try to read in config from either one of the following locations:
/// - `$HOME/.config/erdtree/.erdtreerc`
/// - `$HOME/.erdtreerc`
fn config_from_home() -> Option<String> {
    let home = env::var_os(HOME).map(PathBuf::from)?;

    let config_path = home
        .join(CONFIG_DIR)
        .join(ERDTREE_DIR)
        .join(ERDTREE_CONFIG_NAME);

    fs::read_to_string(config_path).ok().or_else(|| {
        let config_path = home.join(ERDTREE_CONFIG_NAME);
        fs::read_to_string(config_path).ok()
    })
}

/// Try to read in config from either one of the following locations:
/// - `$XDG_CONFIG_HOME/erdtree/.erdtreerc`
/// - `$XDG_CONFIG_HOME/.erdtreerc`
fn config_from_xdg_path() -> Option<String> {
    let xdg_config = env::var_os(XDG_CONFIG_HOME).map(PathBuf::from)?;

    let config_path = xdg_config.join(ERDTREE_DIR).join(ERDTREE_CONFIG_NAME);

    fs::read_to_string(config_path).ok().or_else(|| {
        let config_path = xdg_config.join(ERDTREE_CONFIG_NAME);
        fs::read_to_string(config_path).ok()
    })
}

/// Prepends "--\n" to the config string which is required for proper parsing by
/// [`get_matches_from`].
///
/// [`get_matches_from`]: clap::builder::Command::get_matches_from
fn prepend_arg_prefix(config: String) -> String {
    format!("--\n{config}")
}
