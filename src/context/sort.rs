use clap::ValueEnum;

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Type {
    /// Sort entries by file name in lexicographical order.
    Name,
    /// Sort entries by file name in reversed lexicographical order.
    Rname,

    /// Sort entries by size smallest to largest, top to bottom
    #[default]
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
