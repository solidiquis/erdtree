const ERDTREE_CONFIG_TOML: &str = ".erdtree.toml";
const ERDTREE_TOML_PATH: &str = "ERDTREE_TOML_PATH";

const ERDTREE_CONFIG_NAME: &str = ".erdtreerc";
const ERDTREE_CONFIG_PATH: &str = "ERDTREE_CONFIG_PATH";

const ERDTREE_DIR: &str = "erdtree";

#[cfg(unix)]
const CONFIG_DIR: &str = ".config";

#[cfg(unix)]
const HOME: &str = "HOME";

#[cfg(unix)]
const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

/// Concerned with loading `.erdtree.toml`.
pub mod toml;

/// Concerned with parsing the result of [`toml::load`] into args.
pub mod parse;

#[cfg(test)]
mod test;
