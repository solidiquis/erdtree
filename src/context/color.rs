use clap::ValueEnum;
use once_cell::sync::OnceCell;
use std::{env, ffi::OsString};

pub static NO_COLOR: OnceCell<Option<OsString>> = OnceCell::new();

/// Reads in the `NO_COLOR` environment variable to determine whether or not to display color in
/// the output.
pub fn no_color_env() {
    let _ = NO_COLOR.set(env::var_os("NO_COLOR"));
}

/// Enum to determine how the output should be colorized.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum Coloring {
    /// Print plainly without ANSI escapes
    None,

    /// Attempt to colorize output
    #[default]
    Auto,

    /// Turn on colorization always
    Force,
}
