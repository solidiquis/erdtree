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
    Block,
}

/// Whether to report byte size using SI or binary prefixes or no prefix.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum BytePresentation {
    /// Reports the total amount of bytes
    #[default]
    Raw,

    /// Reports byte size in binary units e.g. KiB
    #[value(alias("binary"))]
    Bin,

    /// Reports byte size in SI units e.g. KB
    #[value(alias("standard-international"))]
    Si,
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

/// Which layout to use when rendering the file-tree
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Layout {
    /// Outputs the tree with the root node at the bottom of the output
    #[default]
    InvertedTree,

    /// Outputs the tree with the root node at the top of the output
    Tree,

    /// Outputs a flat layout using paths rather than an ASCII tree
    Flat,
}

/// Which field to sort directory entries by
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Sort {
    /// No ordering.
    #[default]
    None,

    /// Sort entries by file name in lexicographical order
    Name,
    /// Sort entries by file name in reversed lexicographical order
    Rname,

    /// Sort entries by size smallest to largest, top to bottom
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    Rsize,

    /// Sort entries by newer to older Accessing Date
    #[value(alias("atime"))]
    Access,

    /// Sort entries by older to newer Accessing Date
    #[value(alias("ratime"))]
    Raccess,

    /// Sort entries by newer to older Creation Date
    #[value(alias("ctime"))]
    Create,

    /// Sort entries by older to newer Creation Date
    #[value(alias("rctime"))]
    Rcreate,

    /// Sort entries by newer to older Alteration Date
    #[value(alias("mtime"))]
    Mod,

    /// Sort entries by older to newer Alteration Date
    #[value(alias("rmtime"))]
    Rmod,
}

/// Whether to sort directory entries relative either to their siblings or all directory entries
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SortType {
    /// Sort directory entries relative to their siblings. Only applies when using 'tree' or
    /// 'inverted-tree' layout.
    #[default]
    Tree,
    /// Sort directory entries relative to all directory entries. Only applies when using 'flat'
    /// layout.
    Flat,
}

/// How directories should be ordered relative to regular files.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq, Default)]
pub enum DirOrder {
    /// No particular ordering for directories relative to other files
    #[default]
    None,

    /// Sort directories above files
    First,

    /// Sort directories below files
    Last,
}

/// File-types found in both Unix and Windows.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum FileType {
    /// A regular file
    #[default]
    File,

    /// A directory
    Dir,

    /// A symlink
    Symlink,

    /// A FIFO
    #[cfg(unix)]
    Pipe,

    /// A socket
    #[cfg(unix)]
    Socket,

    /// A character device
    #[cfg(unix)]
    Char,

    /// A block device
    #[cfg(unix)]
    Block,
}