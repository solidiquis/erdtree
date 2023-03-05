use crate::cli;
use std::convert::From;

/// Determines between logical or physical size for display
#[derive(Debug)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    Physical,
}

impl From<&cli::DiskUsage> for DiskUsage {
    fn from(du: &cli::DiskUsage) -> Self {
        match du {
            cli::DiskUsage::Logical => Self::Logical,
            cli::DiskUsage::Physical => Self::Physical,
        }
    }
}
