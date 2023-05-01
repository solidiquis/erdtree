use clap::ValueEnum;

/// Enum to determine how directories should be ordered relative to regular files in output.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum Order {
    /// Directories are ordered as if they were regular nodes.
    #[default]
    None,

    /// Sort directories above files
    First,

    /// Sort directories below files
    Last
}
