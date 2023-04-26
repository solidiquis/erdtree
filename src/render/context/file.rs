use clap::ValueEnum;

/// File-types found in both Unix and Windows.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
#[allow(clippy::module_name_repetitions)]
pub enum FileType {
    /// A regular file.
    #[default]
    File,

    /// A directory.
    Dir,

    /// A symlink.
    Link,
}
