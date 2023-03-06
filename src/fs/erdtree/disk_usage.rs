use crate::cli;
use ansi_term::Color;
use filesize::PathExt;
use std::{
    convert::From,
    fmt::{self, Display},
    fs::Metadata,
    ops::AddAssign,
    path::Path,
};

/// Determines between logical or physical size for display
#[derive(Clone, Debug)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    Physical,
}

/// Binary prefixes
#[derive(Debug)]
pub enum Prefix {
    Base,
    Kibi,
    Mebi,
    Gibi,
}

/// Represents either logical or physical size and handles presentation.
#[derive(Debug)]
pub struct FileSize {
    pub bytes: u64,
    #[allow(dead_code)]
    disk_usage: DiskUsage,
    scale: usize,
}

impl FileSize {
    /// Initializes a [FileSize].
    pub fn new(bytes: u64, disk_usage: DiskUsage, scale: usize) -> Self {
        Self { bytes, disk_usage, scale }
    }

    /// Computes the logical size of a file given its [Metadata].
    pub fn logical(md: &Metadata, scale: usize) -> Self {
        let bytes = md.len();
        Self::new(bytes, DiskUsage::Logical, scale)
    }

    /// Computes the physical size of a file given its [Path] and [Metadata].
    pub fn physical(path: &Path, md: &Metadata, scale: usize) -> Option<Self> {
        path.size_on_disk_fast(md)
            .ok()
            .map(|bytes| Self::new(bytes, DiskUsage::Physical, scale))
    }
}

impl AddAssign<&Self> for FileSize {
    fn add_assign(&mut self, rhs: &Self) {
        self.bytes += rhs.bytes;
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fbytes = self.bytes as f64;
        let log = fbytes.log(2.0);

        let output = if log < 10.0 {
            Color::Cyan.paint(format!("{} {}", self.bytes, Prefix::Base))
        } else if (10.0..20.0).contains(&log) {
            Color::Yellow.paint(format!("{:.scale$} {}", fbytes / 1024.0_f64, Prefix::Kibi, scale = self.scale))
        } else if (20.0..30.0).contains(&log) {
            Color::Green.paint(format!("{:.scale$} {}", fbytes / 1024.0_f64.powi(2), Prefix::Mebi, scale = self.scale))
        } else {
            Color::Red.paint(format!("{:.scale$} {}", fbytes / 1024.0_f64.powi(3), Prefix::Gibi, scale = self.scale))
        };

        write!(f, "{output}")
    }
}

impl From<&cli::DiskUsage> for DiskUsage {
    fn from(du: &cli::DiskUsage) -> Self {
        match du {
            cli::DiskUsage::Logical => Self::Logical,
            cli::DiskUsage::Physical => Self::Physical,
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Base => write!(f, "B"),
            Prefix::Kibi => write!(f, "KiB"),
            Prefix::Mebi => write!(f, "MiB"),
            Prefix::Gibi => write!(f, "GiB"),
        }
    }
}

