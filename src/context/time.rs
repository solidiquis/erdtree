use clap::ValueEnum;

/// Different types of timestamps available in long-view.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Stamp {
    /// Timestamp showing when the file was created.
    Created,

    /// Timestamp showing when the file was last accessed.
    Accessed,

    /// Timestamp showing when the file was last modified.
    #[default]
    Modified,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Format {
    Iso,

    IsoStrict,

    Short,

    #[default]
    Default,
}
