use super::unit::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};
use crate::{render::styles::get_du_theme, Context};
use clap::ValueEnum;
use filesize::PathExt;
use std::{fs::Metadata, ops::AddAssign, path::Path};

/// Represents either logical or physical size and handles presentation.
#[derive(Clone, Debug)]
pub struct FileSize {
    pub bytes: u64,
    #[allow(dead_code)]
    disk_usage: DiskUsage,
    prefix_kind: PrefixKind,
    scale: usize,
}

/// Disk usage information in human readable format
pub struct HumanReadableComponents {
    pub size: String,
    pub unit: String,
}

/// Determines between logical or physical size for display
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum DiskUsage {
    /// How many bytes does a file contain
    #[default]
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    Physical,
}

impl FileSize {
    /// Initializes a [FileSize].
    pub const fn new(
        bytes: u64,
        disk_usage: DiskUsage,
        prefix_kind: PrefixKind,
        scale: usize,
    ) -> Self {
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
        let du_themes = get_du_theme();

        let HumanReadableComponents { size, unit } = Self::human_readable_components(self);
        let color = du_themes.get(unit.as_str()).unwrap();

        if align {
            match self.prefix_kind {
                PrefixKind::Bin => color
                    .paint(format!("{size:>len$} {unit:>3}", len = self.scale + 4))
                    .to_string(),
                PrefixKind::Si => color
                    .paint(format!("{size:>len$} {unit:>2}", len = self.scale + 4))
                    .to_string(),
            }
        } else {
            color.paint(format!("{size} {unit}")).to_string()
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

    /// Returns a tuple of the human readable size and prefix.
    pub fn human_readable_components(&self) -> HumanReadableComponents {
        let fbytes = self.bytes as f64;
        let scale = self.scale;

        let (size, unit) = match self.prefix_kind {
            PrefixKind::Bin => {
                let unit = BinPrefix::from(fbytes);
                let base_value = unit.base_value();

                if matches!(unit, BinPrefix::Base) {
                    (format!("{}", self.bytes), format!("{unit}"))
                } else {
                    // Checks if the `scale` provided results in a value that implies fractional bytes.
                    if self.bytes <= 10_u64.pow(scale as u32) {
                        (format!("{}", self.bytes), format!("{}", BinPrefix::Base))
                    } else {
                        (
                            format!("{:.scale$}", fbytes / (base_value as f64)),
                            format!("{unit}"),
                        )
                    }
                }
            }

            PrefixKind::Si => {
                let unit = SiPrefix::from(fbytes);
                let base_value = unit.base_value();

                if matches!(unit, SiPrefix::Base) {
                    (format!("{}", self.bytes), format!("{unit}"))
                } else {
                    // Checks if the `scale` provided results in a value that implies fractional bytes.
                    if 10_u64.pow(scale as u32) >= base_value {
                        (format!("{}", self.bytes), format!("{}", SiPrefix::Base))
                    } else {
                        (
                            format!("{:.scale$}", fbytes / (base_value as f64)),
                            format!("{unit}"),
                        )
                    }
                }
            }
        };

        HumanReadableComponents { size, unit }
    }
}

impl AddAssign<u64> for FileSize {
    fn add_assign(&mut self, rhs: u64) {
        self.bytes += rhs;
    }
}

impl Default for HumanReadableComponents {
    fn default() -> Self {
        Self {
            size: String::from("0"),
            unit: String::from("B"),
        }
    }
}
