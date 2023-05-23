use crate::{
    context::Context,
    disk_usage::{
        file_size::{byte, DiskUsage, FileSize},
        units::PrefixKind,
    },
    render::theme,
    styles::{self, PLACEHOLDER},
    tree::node::Node,
};
use chrono::{DateTime, Local};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt::{self, Display},
    path::Path,
};

/// Constitutes a single cell in a given row of the output. The `kind` field denotes what type of
/// data actually goes into the cell once rendered. Each `kind` which is of type [Kind] has its own
/// rules for rendering. Cell's do not have to be of a consistent width.
pub struct Cell<'a> {
    ctx: &'a Context,
    node: &'a Node,
    kind: Kind<'a>,
}

/// The type of data that a [Cell] should render.
pub enum Kind<'a> {
    FileName {
        prefix: Option<&'a str>,
    },
    FilePath,
    FileSize,
    #[cfg(unix)]
    Datetime,
    #[cfg(unix)]
    Ino,
    #[cfg(unix)]
    Nlink,
    #[cfg(unix)]
    Blocks,
    #[cfg(unix)]
    Permissions,
}

impl<'a> Cell<'a> {
    /// Initializes a new [Cell].
    pub const fn new(node: &'a Node, ctx: &'a Context, kind: Kind<'a>) -> Self {
        Self { ctx, node, kind }
    }

    /// Rules on how to render a file-name with icons and a prefix if applicable. The order in
    /// which items are rendered are: prefix-icon-name.
    #[inline]
    fn fmt_name(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        match self.kind {
            Kind::FileName { prefix } => {
                let pre = prefix.unwrap_or("");
                let name = theme::stylize_file_name(node);

                if !ctx.icons {
                    return write!(f, "{pre}{name}");
                }

                let icon = node.compute_icon(ctx.no_color());

                write!(f, "{pre}{icon} {name}")
            }

            _ => unreachable!(),
        }
    }

    /// Rules on how to render a file's path
    #[inline]
    fn fmt_path(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let path = if node.depth() == 0 {
            let file_name = node.file_name();
            <OsStr as AsRef<Path>>::as_ref(file_name).display()
        } else {
            node.path()
                .strip_prefix(ctx.dir_canonical())
                .unwrap_or_else(|_| node.path())
                .display()
        };

        if !ctx.icons {
            return write!(f, "{path}");
        }

        let icon = node.compute_icon(ctx.no_color());

        write!(f, "{icon} {path}")
    }

    /// Rules on how to render the file size.
    #[inline]
    fn fmt_file_size(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let Some(file_size) = node.file_size() else {
            return Self::fmt_size_placeholder(f, ctx)
        };

        match file_size {
            FileSize::Byte(metric) => Self::fmt_bytes(f, metric, ctx),
            FileSize::Line(metric) => Self::fmt_unitless_disk_usage(f, metric, ctx),
            FileSize::Word(metric) => Self::fmt_unitless_disk_usage(f, metric, ctx),

            #[cfg(unix)]
            FileSize::Block(metric) => Self::fmt_unitless_disk_usage(f, metric, ctx),
        }
    }

    /// Rules on how to format block for rendering
    #[cfg(unix)]
    #[inline]
    fn fmt_blocks(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let max_width = ctx.max_block_width;

        let out = node
            .blocks()
            .map(|num| format!("{num:>max_width$}"))
            .unwrap_or(format!("{PLACEHOLDER:>max_width$}"));

        let formatted_blocks = if let Ok(style) = styles::get_block_style() {
            style.paint(out).to_string()
        } else {
            out
        };

        write!(f, "{formatted_blocks}")
    }

    /// Rules on how to format nlink for rendering.
    #[cfg(unix)]
    #[inline]
    fn fmt_nlink(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let max_width = ctx.max_nlink_width;

        let out = node
            .nlink()
            .map(|num| format!("{num:>max_width$}"))
            .unwrap_or(format!("{PLACEHOLDER:>max_width$}"));

        let formatted_nlink = if let Ok(style) = styles::get_nlink_style() {
            style.paint(out).to_string()
        } else {
            out
        };

        write!(f, "{formatted_nlink}")
    }

    /// Rules on how to format ino for rendering.
    #[cfg(unix)]
    #[inline]
    fn fmt_ino(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let max_width = ctx.max_ino_width;

        let out = node
            .ino()
            .map(|num| format!("{num:>max_width$}"))
            .unwrap_or(format!("{PLACEHOLDER:>max_width$}"));

        let formatted_ino = if let Ok(style) = styles::get_ino_style() {
            style.paint(out).to_string()
        } else {
            out
        };

        write!(f, "{formatted_ino}")
    }

    /// Rules on how to format datetime for rendering.
    #[cfg(unix)]
    #[inline]
    fn fmt_datetime(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let datetime = match ctx.time() {
            time::Stamp::Created => node.created(),
            time::Stamp::Accessed => node.accessed(),
            time::Stamp::Modified => node.modified(),
        };

        let out = datetime.map(DateTime::<Local>::from).map_or_else(
            || format!("{PLACEHOLDER:>12}"),
            |dt| format!("{:>12}", self.fmt_timestamp(dt)),
        );

        let formatted_datetime = if let Ok(style) = styles::get_datetime_style() {
            style.paint(out).to_string()
        } else {
            out
        };

        write!(f, "{formatted_datetime}")
    }

	/// Rules on how to format timestamp
    #[cfg(unix)]
    #[inline]
    fn fmt_timestamp(&self, dt: DateTime<Local>) -> String {
        let time_format = self.ctx.time_format();
        let delayed_format = match time_format {
            time::Format::Default => dt.format("%d %h %H:%M %g"),
            time::Format::Iso => dt.format("%Y-%m-%d %H:%M:%S"),
            time::Format::IsoStrict => dt.format("%Y-%m-%dT%H:%M:%S%Z"),
            time::Format::Short => dt.format("%Y-%m-%d"),
        };

        format!("{:>12}", delayed_format)
    }

    /// Rules on how to format permissions for rendering
    #[cfg(unix)]
    #[inline]
    fn fmt_permissions(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let formatted_perms = if ctx.octal {
            theme::style_oct_permissions(node)
        } else {
            theme::style_sym_permissions(node)
        };

        write!(f, "{formatted_perms}")
    }

    #[inline]
    fn fmt_size_placeholder(f: &mut fmt::Formatter<'_>, ctx: &Context) -> fmt::Result {
        if ctx.suppress_size || ctx.max_size_width == 0 {
            return write!(f, "");
        }

        let placeholder = styles::get_placeholder_style().map_or_else(
            |_| Cow::from(styles::PLACEHOLDER),
            |style| Cow::from(style.paint(styles::PLACEHOLDER).to_string()),
        );

        let mut placeholder_padding = placeholder.len() + ctx.max_size_width - 1;

        placeholder_padding += match ctx.disk_usage {
            DiskUsage::Logical | DiskUsage::Physical => match ctx.unit {
                PrefixKind::Si if ctx.human => 2,
                PrefixKind::Bin if ctx.human => 3,
                PrefixKind::Si => 0,
                PrefixKind::Bin => 1,
            },
            _ => 0,
        };

        write!(f, "{placeholder:>placeholder_padding$}")
    }

    #[inline]
    fn fmt_bytes(f: &mut fmt::Formatter<'_>, metric: &byte::Metric, ctx: &Context) -> fmt::Result {
        let max_size_width = ctx.max_size_width;
        let max_unit_width = ctx.max_size_unit_width;
        let out = format!("{metric}");
        let [size, unit]: [&str; 2] = out.split(' ').collect::<Vec<&str>>().try_into().unwrap();

        if ctx.no_color() {
            return write!(f, "{size:>max_size_width$} {unit:>max_unit_width$}");
        }

        let color = styles::get_du_theme().unwrap().get(unit).unwrap();

        let out = color.paint(format!("{size:>max_size_width$} {unit:>max_unit_width$}"));

        write!(f, "{out}")
    }

    #[inline]
    fn fmt_unitless_disk_usage<M: Display>(
        f: &mut fmt::Formatter<'_>,
        metric: &M,
        ctx: &Context,
    ) -> fmt::Result {
        let max_size_width = ctx.max_size_width;

        if ctx.no_color() {
            return write!(f, "{metric:>max_size_width$}");
        }
        let color = styles::get_du_theme().unwrap().get("B").unwrap();

        write!(f, "{}", color.paint(format!("{metric:>max_size_width$}")))
    }
}

impl Display for Cell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::FileName { prefix: _prefix } => self.fmt_name(f),
            Kind::FilePath => self.fmt_path(f),
            Kind::FileSize => self.fmt_file_size(f),

            #[cfg(unix)]
            Kind::Ino => self.fmt_ino(f),

            #[cfg(unix)]
            Kind::Nlink => self.fmt_nlink(f),

            #[cfg(unix)]
            Kind::Blocks => self.fmt_blocks(f),

            #[cfg(unix)]
            Kind::Datetime => self.fmt_datetime(f),

            #[cfg(unix)]
            Kind::Permissions => self.fmt_permissions(f),
        }
    }
}
