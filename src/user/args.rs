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

/// Different types of timestamps available in long-view.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TimeStamp {
    /// Time created (alias: ctime)
    #[value(alias("ctime"))]
    Create,

    /// Time last accessed (alias: atime)
    #[value(alias("atime"))]
    Access,

    /// Time last modified (alias: mtime)
    #[default]
    #[value(alias("mtime"))]
    Mod,
}

/// Different formatting options for timestamps
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TimeFormat {
    /// Timestamp formatted following the iso8601, with slight differences and the time-zone omitted
    Iso,

    /// Timestamp formatted following the exact iso8601 specifications
    IsoStrict,

    /// Timestamp only shows date without time in YYYY-MM-DD format
    Short,

    /// Timestamp is shown in DD MMM HH:MM format
    #[default]
    Default,
}

/// Which layout to use when rendering the tree.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Layout {
    /// Outputs the tree with the root node at the bottom of the output
    #[default]
    Regular,

    /// Outputs the tree with the root node at the top of the output
    Inverted,

    /// Outputs a flat layout using paths rather than an ASCII tree
    Flat,

    /// Outputs an inverted flat layout with the root at the top of the output
    Iflat,
}
