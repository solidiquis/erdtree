use crate::render::{context::Context, disk_usage::file_size::FileSize, tree::Node};
use std::borrow::Cow;

#[cfg(unix)]
use crate::render::{
    context::time::Stamp,
    styles::{self, error::Error, PLACEHOLDER},
};

#[cfg(unix)]
use std::time::SystemTime;

#[cfg(unix)]
type StyleGetter = fn() -> Result<&'static ansi_term::Style, Error<'static>>;

/// Attributes for the long view to be displayed.
#[cfg(unix)]
pub(super) struct LongAttrs {
    pub ino: String,
    pub perms: String,
    pub nlink: String,
    pub blocks: String,
    pub timestamp: String,
}

/// Formats the parameters for the long view.
#[cfg(unix)]
#[inline]
pub(super) fn format_long(node: &Node, ctx: &Context) -> LongAttrs {
    let mode = node.mode().unwrap();

    let perms = if ctx.octal {
        Node::style_octal_permissions(&mode)
    } else if node.has_xattrs() {
        Node::style_sym_permissions(&mode, true)
    } else {
        Node::style_sym_permissions(&mode, false)
    };

    let datetime = match ctx.time() {
        Stamp::Created => node.created(),
        Stamp::Accessed => node.accessed(),
        Stamp::Modified => node.modified(),
    };

    let ino = format_num(node.ino(), ctx.max_ino_width, styles::get_ino_style);
    let nlink = format_num(node.nlink(), ctx.max_nlink_width, styles::get_nlink_style);
    let blocks = format_num(node.blocks(), ctx.max_block_width, styles::get_block_style);
    let timestamp = format_datetime(datetime);

    LongAttrs {
        ino,
        perms,
        nlink,
        blocks,
        timestamp,
    }
}

/// Builds the disk usage portion of the output.
#[inline]
pub(super) fn format_size(node: &Node, ctx: &Context) -> String {
    node.file_size().map_or_else(
        || FileSize::placeholder(ctx),
        |size| size.format(ctx.max_du_width),
    )
}

/// Builds the icon portion of the output.
#[inline]
pub(super) fn format_padded_icon(node: &Node, ctx: &Context) -> String {
    if ctx.icons {
        let icon = node.compute_icon(ctx.no_color());
        let padding = icon.len() - 1;
        format!("{icon:<padding$}")
    } else {
        String::new()
    }
}

/// Builds a numeric portion of the output.
#[cfg(unix)]
#[inline]
pub(super) fn format_num(num: Option<u64>, max_width: usize, style_getter: StyleGetter) -> String {
    let out = num
        .map(|num| format!("{num:>max_width$}"))
        .unwrap_or(format!("{PLACEHOLDER:>max_width$}"));

    if let Ok(style) = style_getter() {
        style.paint(out).to_string()
    } else {
        out
    }
}

#[cfg(unix)]
#[inline]
pub(super) fn format_datetime(datetime: Option<SystemTime>) -> String {
    use chrono::{offset::Local, DateTime};

    let out = datetime.map(DateTime::<Local>::from).map_or_else(
        || format!("{PLACEHOLDER:>12}"),
        |dt| format!("{:>12}", dt.format("%d %h %H:%M %g")),
    );

    if let Ok(style) = styles::get_datetime_style() {
        style.paint(out).to_string()
    } else {
        out
    }
}

#[inline]
pub(super) fn file_name(node: &Node) -> Cow<str> {
    node.symlink_target_file_name().map_or_else(
        || Node::stylize(node.file_name(), node.style),
        |target_name| {
            let link_name = node.file_name();
            Node::stylize_link_name(link_name, target_name, node.style)
        },
    )
}
