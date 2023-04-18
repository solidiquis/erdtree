use super::units::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};
use crate::{
    render::styles::{self, get_du_theme, get_placeholder_style},
    Context,
};
use clap::ValueEnum;
use filesize::PathExt;
use std::{borrow::Cow, fs::Metadata, ops::AddAssign, path::Path};

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
    Logical,

    /// How much actual space on disk, taking into account sparse files and compression.
    #[default]
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
    pub fn format(&self, max_size_width: usize) -> String {
        let du_themes = get_du_theme().ok();

        let HumanReadableComponents { size, unit } = Self::human_readable_components(self);
        let color = du_themes.and_then(|th| th.get(unit.as_str()));

        let max_padded = max_size_width + 5;
        let current_padded = self.scale + 5;

        let padded_total_width = if current_padded > max_padded {
            max_padded
        } else {
            current_padded
        };

        #[allow(clippy::option_if_let_else)]
        match color {
            Some(col) => match self.prefix_kind {
                PrefixKind::Bin => col
                    .paint(format!("{size:>padded_total_width$} {unit:>3}"))
                    .to_string(),
                PrefixKind::Si => col
                    .paint(format!("{size:>padded_total_width$} {unit:>2}"))
                    .to_string(),
            },

            None => match self.prefix_kind {
                PrefixKind::Bin => format!("{size:>padded_total_width$} {unit:>3}"),
                PrefixKind::Si => format!("{size:>padded_total_width$} {unit:>2}"),
            },
        }
    }

    /// Returns spaces times the length of a file size, formatted with the given options
    /// " " * len(123.45 KiB)
    pub fn empty_string(ctx: &Context) -> String {
        if ctx.suppress_size {
            String::new()
        } else {
            let (placeholder, extra_padding) = get_placeholder_style().map_or_else(
                |_| (Cow::from(styles::PLACEHOLDER), 0),
                |style| {
                    let placeholder = Cow::from(style.paint(styles::PLACEHOLDER).to_string());
                    let padding = placeholder.len();
                    (placeholder, padding)
                },
            );

            format!(
                "{:>len$}",
                placeholder,
                len = Self::empty_string_len(ctx) + extra_padding
            )
        }
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
            + match ctx.unit {
                PrefixKind::Bin => 3,
                PrefixKind::Si => 2,
            }
    }

    /// Returns a tuple of the human readable size and prefix.
    pub fn human_readable_components(&self) -> HumanReadableComponents {
        let fbytes = self.bytes as f64;
        let scale = self.scale;
        let power = u32::try_from(scale).expect("Provided scale caused an overflow");

        let (size, unit) = match self.prefix_kind {
            PrefixKind::Bin => {
                let unit = BinPrefix::from(fbytes);
                let base_value = unit.base_value();

                if matches!(unit, BinPrefix::Base) {
                    (format!("{}", self.bytes), format!("{unit}"))
                } else {
                    // Checks if the `scale` provided results in a value that implies fractional bytes.
                    if self.bytes <= 10_u64.checked_pow(power).unwrap_or(u64::MAX) {
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
                    if 10_u64.checked_pow(power).unwrap_or(u64::MAX) >= base_value {
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
