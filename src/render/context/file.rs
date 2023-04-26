use clap::ValueEnum;

/// File-types found in both Unix and Windows.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Type {
    /// A regular file.
    #[default]
    File,

    /// A directory.
    Dir,

    /// A symlink.
    Link,
}
