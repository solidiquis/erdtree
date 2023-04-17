use crate::render::{context::Context, tree::node::Node};

pub fn compute_with_color(node: &Node, prefix: &str, ctx: &Context) -> String {
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

        let sym = if node.has_xattrs() {
            Node::style_sym_permissions(&format!("{}@", &mode))
        } else {
            Node::style_sym_permissions(&format!("{} ", &mode))
        };

        let oct = Node::style_octal_permissions(&mode);

        format!(
            "{oct} {sym:mode_len$}{size} {prefix}{padded_icon}{file_name}",
            mode_len = sym.len() + 1,
        )
    } else {
        format!("{size} {prefix}{padded_icon}{file_name}")
    }
}

pub fn compute(node: &Node, prefix: &str, ctx: &Context) -> String {
    let size = presenters::size(node, ctx);
    let padded_icon = presenters::padded_icon(node, ctx);

    let file_name = node.symlink_target_file_name().map_or_else(
        || node.file_name_lossy(),
        |target_name| {
            let link_name = node.file_name();
            Node::stylize_link_name(link_name, target_name, None)
        },
    );

    if ctx.long {
        let mode = node.mode().unwrap();

        let sym = if node.has_xattrs() {
            format!("{mode}@")
        } else {
            format!("{mode} ")
        };

        let oct = format!("{mode:04o}");

        format!(
            "{oct} {sym:mode_len$}{size} {prefix}{padded_icon}{file_name}",
            mode_len = sym.len() + 1,
        )
    } else {
        format!("{size} {prefix}{padded_icon}{file_name}")
    }
}

/// Helper functions to build each component of the output.
mod presenters {
    use crate::render::{context::Context, disk_usage::file_size::FileSize, tree::Node};

    #[inline]
    /// Builds the disk usage portion of the output.
    pub(super) fn size(node: &Node, ctx: &Context) -> String {
        node.file_size().map_or_else(
            || FileSize::empty_string(ctx),
            |size| size.format(ctx.max_du_width),
        )
    }

    #[inline]
    /// Builds the icon portion of the output.
    pub(super) fn padded_icon(node: &Node, ctx: &Context) -> String {
        if ctx.icons {
            let icon = node.compute_icon(ctx.no_color());
            let padding = icon.len() - 1;
            format!("{icon:<padding$}")
        } else {
            String::new()
        }
    }
}
