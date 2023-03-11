use std::{env, fs, path::PathBuf};

pub const CONFIG_ENV_VAR: &str = "ERDTREE_CONFIG_PATH";
pub const CONFIG_NAME: &str = ".erdtreerc";

pub fn read_config_to_string() -> Option<String> {
    env::var_os(CONFIG_ENV_VAR)
        .map(PathBuf::from)
        .map(fs::read_to_string)
        .map(Result::ok)
        .flatten()
        .or_else(|| {
            env::var_os("HOME")
                .map(PathBuf::from)
                .map(|p| p.join(CONFIG_NAME))
                .map(fs::read_to_string)
                .map(Result::ok)
                .flatten()
        })
}
