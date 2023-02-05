use clap::ValueEnum;

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Order {
    /// Sort entries by file name
    Filename,
    
    /// Sort entries by size
    Size,

    /// No sorting
    None
}
