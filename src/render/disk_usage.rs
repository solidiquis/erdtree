use ansi_term::Color;
use clap::ValueEnum;
use filesize::PathExt;
use std::{
    fmt::{self, Display},
    fs::Metadata,
    ops::AddAssign,
    path::Path,
};

/// Determines between logical or physical size for display
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    Physical,
}

/// Determines whether to use SI prefixes or binary prefixes.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum PrefixKind {
    /// Displays disk usage using binary prefixes.
    Bin,

    /// Displays disk usage using SI prefixes.
    Si,
}

/// Binary prefixes.
#[derive(Debug)]
pub enum BinPrefix {
    Base,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
}

/// SI prefixes.
#[derive(Debug)]
pub enum SiPrefix {
    Base,
    Kilo,
    Mega,
    Giga,
    Tera,
}

/// Represents either logical or physical size and handles presentation.
#[derive(Debug)]
pub struct FileSize {
    pub bytes: u64,
    #[allow(dead_code)]
    disk_usage: DiskUsage,
    prefix_kind: PrefixKind,
    scale: usize,
}

impl FileSize {
    /// Initializes a [FileSize].
    pub fn new(bytes: u64, disk_usage: DiskUsage, prefix_kind: PrefixKind, scale: usize) -> Self {
        Self {
            bytes,
            disk_usage,
            prefix_kind,
            scale,
        }
    }

    /// Computes the logical size of a file given its [Metadata].
    pub fn logical(md: &Metadata, prefix_kind: PrefixKind, scale: usize) -> Self {
        let bytes = md.len();
        Self::new(bytes, DiskUsage::Logical, prefix_kind, scale)
    }

    /// Computes the physical size of a file given its [Path] and [Metadata].
    pub fn physical(
        path: &Path,
        md: &Metadata,
        prefix_kind: PrefixKind,
        scale: usize,
    ) -> Option<Self> {
        path.size_on_disk_fast(md)
            .ok()
            .map(|bytes| Self::new(bytes, DiskUsage::Physical, prefix_kind, scale))
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

        let output = match self.prefix_kind {
            PrefixKind::Bin => {
                let log = fbytes.log(2.0);

                if log < 10.0 {
                    Color::Cyan.paint(format!("{} {}", self.bytes, BinPrefix::Base))
                } else if (10.0..20.0).contains(&log) {
                    Color::Yellow.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 1024.0_f64,
                        BinPrefix::Kibi,
                        scale = self.scale
                    ))
                } else if (20.0..30.0).contains(&log) {
                    Color::Green.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 1024.0_f64.powi(2),
                        BinPrefix::Mebi,
                        scale = self.scale
                    ))
                } else if (30.0..40.0).contains(&log) {
                    Color::Red.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 1024.0_f64.powi(3),
                        BinPrefix::Gibi,
                        scale = self.scale
                    ))
                } else {
                    Color::Blue.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 1024.0_f64.powi(4),
                        BinPrefix::Tebi,
                        scale = self.scale
                    ))
                }
            }

            PrefixKind::Si => {
                let log = fbytes.log(10.0);

                if log < 3.0 {
                    Color::Cyan.paint(format!("{} {}", fbytes, SiPrefix::Base))
                } else if (3.0..6.0).contains(&log) {
                    Color::Yellow.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 10.0_f64.powi(3),
                        SiPrefix::Kilo,
                        scale = self.scale
                    ))
                } else if (6.0..9.0).contains(&log) {
                    Color::Green.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 10.0_f64.powi(6),
                        SiPrefix::Mega,
                        scale = self.scale
                    ))
                } else if (9.0..12.0).contains(&log) {
                    Color::Green.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 10.0_f64.powi(9),
                        SiPrefix::Giga,
                        scale = self.scale
                    ))
                } else {
                    Color::Red.paint(format!(
                        "{:.scale$} {}",
                        fbytes / 10.0_f64.powi(12),
                        SiPrefix::Tera,
                        scale = self.scale
                    ))
                }
            }
        };

        write!(f, "{output}")
    }
}

impl Display for BinPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "B"),
            Self::Kibi => write!(f, "KiB"),
            Self::Mebi => write!(f, "MiB"),
            Self::Gibi => write!(f, "GiB"),
            Self::Tebi => write!(f, "TiB"),
        }
    }
}

impl Display for SiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "B"),
            Self::Kilo => write!(f, "KB"),
            Self::Mega => write!(f, "MB"),
            Self::Giga => write!(f, "GB"),
            Self::Tera => write!(f, "TB"),
        }
    }
}
