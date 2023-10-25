use clap::ValueEnum;

/// The disk usage metric to report.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum Metric {
    /// Physical disk usage in bytes
    #[default]
    Physical,

    /// Apparent disk usage in bytes
    Logical,

    /// Total words in a file
    Word,

    /// Total lines in a file
    Line,

    /// Total amount of blocks allocated to store a file on disk
    #[cfg(unix)]
    Blocks,
}

/// Whether to report byte size using SI or binary prefixes or no prefix.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum BytePresentation {
    /// Reports the total amount of bytes
    #[default]
    Raw,

    /// Reports byte size in binary units e.g. KiB
    Binary,

    /// Reports byte size in SI units e.g. KB
    StandardInternational,
}
