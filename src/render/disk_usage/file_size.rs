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
    /// The amount of padding we need to reserve to the left of the disk usage.
    const LEFT_PADDING: usize = 5;

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

    /// Formats the [FileSize] in a human readable format.
    pub fn format_human_readable(&self, max_size_width: usize) -> String {
        let du_themes = get_du_theme().ok();

        let HumanReadableComponents { size, unit } = Self::human_readable_components(self);
        let color = du_themes.and_then(|th| th.get(unit.as_str()));

        let max_padded = max_size_width + Self::LEFT_PADDING;
        let current_padded = self.scale + Self::LEFT_PADDING;

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

    /// Formats [FileSize] for presentation.
    pub fn format(&self, max_size_width: usize) -> String {
        format!("{:<width$}B", self.bytes, width = max_size_width + 1)
    }

    /// Returns a placeholder or empty string.
    pub fn placeholder(ctx: &Context) -> String {
        if ctx.suppress_size || ctx.max_du_width == 0 {
            String::new()
        } else {
            let (placeholder, extra_padding) = get_placeholder_style().map_or_else(
                |_| (Cow::from(styles::PLACEHOLDER), 1),
                |style| {
                    let placeholder = Cow::from(style.paint(styles::PLACEHOLDER).to_string());
                    let padding = placeholder.len();
                    (placeholder, padding)
                },
            );

            format!(
                "{:>len$}",
                placeholder,
                len = Self::placeholder_padding(ctx) + extra_padding
            )
        }
    }

    /// Base amount of padding to use for the placeholder.
    #[inline]
    const fn placeholder_padding(ctx: &Context) -> usize {
        let unit_len = match ctx.unit {
            PrefixKind::Bin => 3,
            PrefixKind::Si => 2,
        };

        let max_pad = Self::LEFT_PADDING + ctx.max_du_width + unit_len;
        let padding = Self::LEFT_PADDING + unit_len + if ctx.flat { 0 } else { 2 };

        if padding > max_pad {
            max_pad
        } else {
            padding
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

impl AddAssign<&Self> for FileSize {
    fn add_assign(&mut self, rhs: &Self) {
        self.bytes += rhs.bytes;
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
