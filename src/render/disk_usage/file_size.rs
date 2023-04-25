use super::units::{BinPrefix, PrefixKind, SiPrefix, UnitPrefix};
use crate::{
    render::styles::{self, get_du_theme, get_placeholder_style},
    utils, Context,
};
use ansi_term::Style;
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
    human_readable: bool,
    unpadded_display: Option<String>,

    // Precomputed style to use
    style: Option<&'static Style>,

    // Does this file size use `B` without a prefix?
    uses_base_unit: Option<()>,

    // How many columns are required for the size (without prefix).
    pub size_columns: usize,
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
        human_readable: bool,
        prefix_kind: PrefixKind,
    ) -> Self {
        Self {
            bytes,
            disk_usage,
            human_readable,
            prefix_kind,
            unpadded_display: None,
            style: None,
            uses_base_unit: None,
            size_columns: 0,
        }
    }

    /// Computes the logical size of a file given its [Metadata].
    pub fn logical(md: &Metadata, prefix_kind: PrefixKind, human_readable: bool) -> Self {
        let bytes = md.len();
        Self::new(bytes, DiskUsage::Logical, human_readable, prefix_kind)
    }

    /// Computes the physical size of a file given its [Path] and [Metadata].
    pub fn physical(
        path: &Path,
        md: &Metadata,
        prefix_kind: PrefixKind,
        human_readable: bool,
    ) -> Option<Self> {
        path.size_on_disk_fast(md)
            .ok()
            .map(|bytes| Self::new(bytes, DiskUsage::Physical, human_readable, prefix_kind))
    }

    pub fn unpadded_display(&self) -> Option<&str> {
        self.unpadded_display.as_deref()
    }

    /// Precompute the raw (unpadded) display and sets the number of columns the size (without
    /// the prefix) will occupy. Also sets the [Style] to use in advance to style the size output.
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn precompute_unpadded_display(&mut self) {
        let fbytes = self.bytes as f64;

        match self.prefix_kind {
            PrefixKind::Si => {
                let unit = SiPrefix::from(fbytes);
                let base_value = unit.base_value();

                if !self.human_readable {
                    self.unpadded_display = Some(format!("{} B", self.bytes));
                    self.size_columns = utils::num_integral(self.bytes);
                } else if matches!(unit, SiPrefix::Base) {
                    self.unpadded_display = Some(format!("{} {unit}", self.bytes));
                    self.size_columns = utils::num_integral(self.bytes);
                    self.uses_base_unit = Some(());
                } else {
                    let size = fbytes / (base_value as f64);
                    self.unpadded_display = Some(format!("{size:.2} {unit}"));
                    self.size_columns = utils::num_integral((size * 100.0).floor() as u64) + 1;
                }

                if let Ok(theme) = get_du_theme() {
                    let style = theme.get(format!("{unit}").as_str());
                    self.style = style;
                }
            }
            PrefixKind::Bin => {
                let unit = BinPrefix::from(fbytes);
                let base_value = unit.base_value();

                if !self.human_readable {
                    self.unpadded_display = Some(format!("{} B", self.bytes));
                    self.size_columns = utils::num_integral(self.bytes);
                } else if matches!(unit, BinPrefix::Base) {
                    self.unpadded_display = Some(format!("{} {unit}", self.bytes));
                    self.size_columns = utils::num_integral(self.bytes);
                    self.uses_base_unit = Some(());
                } else {
                    let size = fbytes / (base_value as f64);
                    self.unpadded_display = Some(format!("{size:.2} {unit}"));
                    self.size_columns = utils::num_integral((size * 100.0).floor() as u64) + 1;
                }

                if let Ok(theme) = get_du_theme() {
                    let style = theme.get(format!("{unit}").as_str());
                    self.style = style;
                }
            }
        }
    }

    /// Formats [FileSize] for presentation.
    pub fn format(&self, max_size_width: usize) -> String {
        let out = if self.human_readable {
            let mut precomputed = self.unpadded_display().unwrap().split(' ');
            let size = precomputed.next().unwrap();
            let unit = precomputed.next().unwrap();
            let unit_padding = match self.prefix_kind {
                PrefixKind::Si => 2,
                PrefixKind::Bin => 3,
            };

            if self.uses_base_unit.is_some() {
                format!("{:>max_size_width$} {unit:>unit_padding$}", self.bytes)
            } else {
                format!("{size:>max_size_width$} {unit:>unit_padding$}")
            }
        } else {
            format!("{:<max_size_width$} B", self.bytes)
        };

        if let Some(style) = self.style {
            style.paint(out).to_string()
        } else {
            out
        }
    }

    /// Returns a placeholder or empty string.
    pub fn placeholder(ctx: &Context) -> String {
        if ctx.suppress_size || ctx.max_size_width == 0 {
            return String::new();
        }

        let placeholder = get_placeholder_style().map_or_else(
            |_| Cow::from(styles::PLACEHOLDER),
            |style| Cow::from(style.paint(styles::PLACEHOLDER).to_string()),
        );

        let placeholder_padding = placeholder.len()
            + ctx.max_size_width
            + match ctx.unit {
                PrefixKind::Si if ctx.human => 2,
                PrefixKind::Bin if ctx.human => 3,
                PrefixKind::Si => 0,
                PrefixKind::Bin => 1,
            };

        format!("{placeholder:>placeholder_padding$}")
    }
}

impl AddAssign<&Self> for FileSize {
    fn add_assign(&mut self, rhs: &Self) {
        self.bytes += rhs.bytes;
    }
}
