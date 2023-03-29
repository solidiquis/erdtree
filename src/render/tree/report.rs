use super::{node::Node, Tree};
use crate::render::disk_usage::{
    file_size::{FileSize, HumanReadableComponents},
    unit::PrefixKind,
};
use std::{
    convert::AsRef,
    ffi::OsStr,
    fmt::{self, Display},
    path::Path,
};

pub struct Report<'a> {
    tree: &'a Tree,
}

impl<'a> Report<'a> {
    pub const fn new(tree: &'a Tree) -> Self {
        Self { tree }
    }
}

impl Display for Report<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tree = self.tree.inner();
        let root = self.tree.root();
        let ctx = self.tree.context();
        let max_depth = ctx.level().unwrap_or(usize::MAX);
        let dir = ctx.dir();
        let prefix_kind = ctx.prefix;

        let du_info = |node: &Node| {
            if ctx.human {
                let HumanReadableComponents { size, unit } = node.file_size().map_or_else(
                    HumanReadableComponents::default,
                    FileSize::human_readable_components,
                );

                (size, unit)
            } else {
                let size = node
                    .file_size()
                    .map_or_else(|| String::from("0"), |fs| format!("{}", fs.bytes));

                let unit = String::from("B");

                (size, unit)
            }
        };

        let root_node = tree[root].get();

        let total_du_width = root_node
            .file_size()
            .map_or_else(|| String::from("0"), |fs| format!("{}", fs.bytes))
            .len();

        let (total_du, root_unit) = du_info(root_node);

        let width_du_col = if ctx.human {
            total_du_width
                + root_unit.len()
                + if matches!(prefix_kind, PrefixKind::Bin) {
                    3
                } else {
                    1
                }
        } else {
            total_du_width + 2
        };

        let root_du_info = format!("{total_du} {root_unit}");
        let root_iden = root_node.file_type_identifier().unwrap_or("-");

        let root_name = <OsStr as AsRef<Path>>::as_ref(root_node.file_name()).display();

        writeln!(
            f,
            "{root_iden}   {root_du_info:>width_du_col$}   {root_name}"
        )?;

        let base_path = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());

        for node_id in root.descendants(tree).skip(1) {
            let node = tree[node_id].get();

            if node.depth > max_depth {
                continue;
            }

            let (du, unit) = du_info(node);
            let du_info = format!("{du} {unit}");
            let ft_iden = node.file_type_identifier().unwrap_or("-");

            let file = if ctx.file_name {
                <OsStr as AsRef<Path>>::as_ref(node.file_name()).display()
            } else {
                let full_path = node.path();

                full_path
                    .strip_prefix(&base_path)
                    .unwrap_or(full_path)
                    .display()
            };

            writeln!(f, "{ft_iden}   {du_info:>width_du_col$}   {file}")?;
        }

        Ok(())
    }
}
