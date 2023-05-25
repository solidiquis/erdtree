use clap::ValueEnum;

/// Different types of timestamps available in long-view.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Stamp {
    /// Timestamp showing when the file was created.
    Create,

    /// Timestamp showing when the file was last accessed.
    Access,

    /// Timestamp showing when the file was last modified.
    #[default]
    Mod,
}

/// Different formatting options for timestamps
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Format {
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
