use crate::render::{context::Context, tree::node::Node};

#[cfg(unix)]
pub fn compute(node: &Node, prefix: &str, ctx: &Context) -> String {
    use crate::render::{context::time::Stamp, styles};

    let size = presenters::size(node, ctx);
    let padded_icon = presenters::padded_icon(node, ctx);

    let file_name = node.symlink_target_file_name().map_or_else(
        || Node::stylize(node.file_name(), node.style),
        |target_name| {
            let link_name = node.file_name();
            Node::stylize_link_name(link_name, target_name, node.style)
        },
    );

    if ctx.long {
        let mode = node.mode().unwrap();

        let perms = if ctx.octal {
            Node::style_octal_permissions(&mode)
        } else if node.has_xattrs() {
            Node::style_sym_permissions(&mode, true)
        } else {
            Node::style_sym_permissions(&mode, false)
        };

        let datetime = match ctx.time {
            Stamp::Created => node.created(),
            Stamp::Accessed => node.accessed(),
            Stamp::Modified => node.modified(),
        };

        let ino = presenters::num(node.ino(), ctx.max_ino_width, styles::get_ino_style);
        let nlink = presenters::num(node.nlink(), ctx.max_nlink_width, styles::get_nlink_style);
        let blocks = presenters::num(node.blocks(), ctx.max_block_width, styles::get_block_style);
        let timestamp = presenters::datetime(datetime);

        format!(
            "{ino:<ino_padding$}{perms:<perms_padding$}{nlink} {blocks} {timestamp} {size} {prefix}{padded_icon}{file_name}",
            ino_padding = ino.len() + 1,
            perms_padding = perms.len() + 1,
        )
    } else {
        format!("{size} {prefix}{padded_icon}{file_name}")
    }
}

#[cfg(not(unix))]
pub fn compute(node: &Node, prefix: &str, ctx: &Context) -> String {
    let size = presenters::size(node, ctx);
    let padded_icon = presenters::padded_icon(node, ctx);

    let file_name = node.symlink_target_file_name().map_or_else(
        || Node::stylize(node.file_name(), node.style),
        |target_name| {
            let link_name = node.file_name();
            Node::stylize_link_name(link_name, target_name, node.style)
        },
    );

    format!("{size} {prefix}{padded_icon}{file_name}")
}

/// Helper functions to build each component of the output.
mod presenters {
    use crate::render::{context::Context, disk_usage::file_size::FileSize, tree::Node};

    #[cfg(unix)]
    use crate::render::styles::{self, error::Error, PLACEHOLDER};

    #[cfg(unix)]
    use std::time::SystemTime;

    #[cfg(unix)]
    type StyleGetter = fn() -> Result<&'static ansi_term::Style, Error<'static>>;

    /// Builds the disk usage portion of the output.
    #[inline]
    pub(super) fn size(node: &Node, ctx: &Context) -> String {
        node.file_size().map_or_else(
            || FileSize::placeholder(ctx),
            |size| size.format(ctx.max_du_width),
        )
    }

    /// Builds the icon portion of the output.
    #[inline]
    pub(super) fn padded_icon(node: &Node, ctx: &Context) -> String {
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
    pub(super) fn num(num: Option<u64>, max_width: usize, style_getter: StyleGetter) -> String {
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
    pub(super) fn datetime(datetime: Option<SystemTime>) -> String {
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
}
