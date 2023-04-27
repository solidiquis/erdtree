use clap::ValueEnum;

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Type {
    /// Sort entries by file name
    Name,

    /// Sort entries by size smallest to largest, top to bottom
    #[default]
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    SizeRev,
}
