use ansi_term::Color;
use clap::ValueEnum;
use filesize::PathExt;
use std::{
    fmt::{self, Display},
    fs::Metadata,
    ops::AddAssign,
    path::Path,
};

use crate::Context;

/// Determines between logical or physical size for display
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    #[default]
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    Physical,
}

/// Determines whether to use SI prefixes or binary prefixes.
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum PrefixKind {
    /// Displays disk usage using binary prefixes.
    #[default]
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
#[derive(Clone, Debug)]
pub struct FileSize {
    pub bytes: u64,
    #[allow(dead_code)]
    disk_usage: DiskUsage,
    prefix_kind: PrefixKind,
    scale: usize,
}

impl FileSize {
    /// Initializes a [FileSize].
    pub const fn new(bytes: u64, disk_usage: DiskUsage, prefix_kind: PrefixKind, scale: usize) -> Self {
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

impl AddAssign<u64> for FileSize {
    fn add_assign(&mut self, rhs: u64) {
        self.bytes += rhs;
    }
}

impl FileSize {
    /// Transforms the `FileSize` into a string.
    /// `Display` / `ToString` traits not used in order to have control over alignment.
    ///
    /// `align` false makes strings such as
    /// `123.45 KiB`
    /// `1.23 MiB`
    /// `12 B`
    ///
    /// `align` true makes strings such as
    /// `123.45 KiB`
    /// `  1.23 MiB`
    /// `    12   B`
    pub fn format(&self, align: bool) -> String {
        let fbytes = self.bytes as f64;
        let scale = self.scale;
        let (color, bytes, base) = match self.prefix_kind {
            PrefixKind::Bin => {
                let log = fbytes.log2();
                if log < 10. {
                    (
                        Color::Cyan,
                        format!("{}", self.bytes),
                        format!("{}", BinPrefix::Base),
                    )
                } else if log < 20. {
                    (
                        Color::Yellow,
                        format!("{:.scale$}", fbytes / 1024.0_f64.powi(1),),
                        format!("{}", BinPrefix::Kibi),
                    )
                } else if log < 30. {
                    (
                        Color::Green,
                        format!("{:.scale$}", fbytes / 1024.0_f64.powi(2),),
                        format!("{}", BinPrefix::Mebi),
                    )
                } else if log < 40. {
                    (
                        Color::Red,
                        format!("{:.scale$}", fbytes / 1024.0_f64.powi(3),),
                        format!("{}", BinPrefix::Gibi),
                    )
                } else {
                    (
                        Color::Blue,
                        format!("{:.scale$}", fbytes / 1024.0_f64.powi(4),),
                        format!("{}", BinPrefix::Tebi),
                    )
                }
            }
            PrefixKind::Si => {
                let log = fbytes.log10();
                if log < 3. {
                    (
                        Color::Cyan,
                        format!("{}", self.bytes),
                        format!("{}", SiPrefix::Base),
                    )
                } else if log < 6. {
                    (
                        Color::Yellow,
                        format!("{:.scale$}", fbytes / 10.0_f64.powi(3),),
                        format!("{}", SiPrefix::Kilo),
                    )
                } else if log < 9. {
                    (
                        Color::Green,
                        format!("{:.scale$}", fbytes / 10.0_f64.powi(6),),
                        format!("{}", SiPrefix::Mega),
                    )
                } else if log < 12. {
                    (
                        Color::Red,
                        format!("{:.scale$}", fbytes / 10.0_f64.powi(9),),
                        format!("{}", SiPrefix::Giga),
                    )
                } else {
                    (
                        Color::Blue,
                        format!("{:.scale$}", fbytes / 10.0_f64.powi(12),),
                        format!("{}", SiPrefix::Tera),
                    )
                }
            }
        };
        if align {
            match self.prefix_kind {
                PrefixKind::Bin => color
                    .paint(format!("{bytes:>len$} {base:>3}", len = self.scale + 4))
                    .to_string(),
                PrefixKind::Si => color
                    .paint(format!("{bytes:>len$} {base:>2}", len = self.scale + 4))
                    .to_string(),
            }
        } else {
            color.paint(format!("{bytes} {base}")).to_string()
        }
    }

    /// Returns spaces times the length of a file size, formatted with the given options
    /// " " * len(123.45 KiB)
    pub fn empty_string(ctx: &Context) -> String {
        format!("{:len$}", "", len = Self::empty_string_len(ctx))
    }

    const fn empty_string_len(ctx: &Context) -> usize {
        // 3 places before the decimal
        // 1 for the decimal
        // ctx.scale after the decimal
        // 1 space before unit
        // 2/3 spaces per unit, depending
        3 + 1
            + ctx.scale
            + 1
            + match ctx.prefix {
                PrefixKind::Bin => 3,
                PrefixKind::Si => 2,
            }
    }
}

impl Display for BinPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => f.pad("B"),
            Self::Kibi => f.pad("KiB"),
            Self::Mebi => f.pad("MiB"),
            Self::Gibi => f.pad("GiB"),
            Self::Tebi => f.pad("TiB"),
        }
    }
}

impl Display for SiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => f.pad("B"),
            Self::Kilo => f.pad("KB"),
            Self::Mega => f.pad("MB"),
            Self::Giga => f.pad("GB"),
            Self::Tera => f.pad("TB"),
        }
    }
}
