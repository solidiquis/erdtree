use crate::{
    ansi::Escaped,
    render::{context::Context, tree::node::Node},
};
use std::{
    borrow::Cow,
    fmt::{self, Formatter},
};

/// Helpers to prepare each invidual section of the [Node]'s attributes to display.
mod presenters;

impl Node {
    /// Formats the [Node] for the tree view.
    #[cfg(unix)]
    pub(super) fn tree(
        &self,
        f: &mut Formatter,
        prefix: Option<&str>,
        ctx: &Context,
    ) -> fmt::Result {
        let size = presenters::format_size(self, ctx);
        let padded_icon = presenters::format_padded_icon(self, ctx);
        let file_name = presenters::file_name(self);

        let pre = prefix.unwrap_or("");

        let ln = if ctx.long {
            let presenters::LongAttrs {
                ino,
                perms,
                nlink,
                blocks,
                timestamp,
            } = presenters::format_long(self, ctx);

            format!(
                "{ino:<ino_padding$} {perms:<perms_padding$} {nlink} {blocks} {timestamp} {size} {pre}{padded_icon}{file_name}",
                ino_padding = ino.len(),
                perms_padding = perms.len(),
            )
        } else {
            format!("{size} {pre}{padded_icon}{file_name}")
        };

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&ln, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{ln}")
        }
    }

    /// Formats the [Node] for a plain report view.
    #[cfg(unix)]
    pub(super) fn flat(&self, f: &mut Formatter, ctx: &Context) -> fmt::Result {
        use std::{ffi::OsStr, path::Path};

        let size = presenters::format_size(self, ctx);

        let file = {
            let node_path = self.path();

            if self.depth() == 0 {
                node_path.file_name().map_or_else(
                    || Cow::from(node_path.display().to_string()),
                    OsStr::to_string_lossy,
                )
            } else {
                node_path
                    .strip_prefix(ctx.dir_canonical())
                    .map_or_else(|_| self.path().to_string_lossy(), Path::to_string_lossy)
            }
        };

        let ln = if ctx.long {
            let presenters::LongAttrs {
                ino,
                perms,
                nlink,
                blocks,
                timestamp,
            } = presenters::format_long(self, ctx);

            format!(
                "{ino:<ino_padding$} {perms:<perms_padding$} {nlink} {blocks} {timestamp} {size}   {file}",
                ino_padding = ino.len(),
                perms_padding = perms.len(),
            )
        } else {
            format!("{size}   {file}")
        };

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&ln, window_width);
            writeln!(f, "{out}")
        } else {
            writeln!(f, "{ln}")
        }
    }

    /// Formats the [Node] for a plain report view.
    #[cfg(not(unix))]
    pub(super) fn flat(&self, f: &mut Formatter, ctx: &Context) -> fmt::Result {
        let size = presenters::format_size(self, ctx);

        let file = {
            let path = self
                .path()
                .strip_prefix(ctx.dir_canonical())
                .unwrap_or_else(|_| self.path());

            Cow::from(path.display().to_string())
        };

        let ln = format!("{size}   {file}");

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as AnsiEscaped>::truncate(&ln, window_width);
            writeln!(f, "{out}")
        } else {
            writeln!(f, "{ln}")
        }
    }

    /// Formats the [Node] for the tree view.
    #[cfg(not(unix))]
    pub(super) fn tree(
        &self,
        f: &mut Formatter,
        prefix: Option<&str>,
        ctx: &Context,
    ) -> fmt::Result {
        let size = presenters::format_size(self, ctx);
        let padded_icon = presenters::format_padded_icon(self, ctx);
        let file_name = presenters::file_name(self);
        let pre = prefix.unwrap_or("");

        let ln = format!("{size} {pre}{padded_icon}{file_name}");

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as AnsiEscaped>::truncate(&ln, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{ln}")
        }
    }
}
