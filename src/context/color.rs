use clap::ValueEnum;

/// Enum to determine how the output should be colorized.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum Coloring {
    /// Print plainly without ANSI escapes
    None,

    /// Attempt to colorize output
    #[default]
    Auto,

    /// Turn on colorization always
    Forced,
}
