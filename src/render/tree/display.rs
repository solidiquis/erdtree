use crate::render::{
    disk_usage::{
        file_size::{FileSize, HumanReadableComponents},
        units::PrefixKind,
    },
    styles,
    tree::{count::FileCount, node::Node, Tree},
};
use indextree::NodeId;
use std::{
    ffi::OsStr,
    fmt::{self, Display, Formatter},
    path::Path,
};

/// Empty trait used to constrain generic parameter `display_variant` of [Tree].
pub trait TreeVariant {}

/// For printing output with colored ANSI escapes.
pub struct Regular {}

/// For generating plain-text report of disk usage without ASCII tree.
pub struct Report {}

impl TreeVariant for Regular {}
impl TreeVariant for Report {}

impl Display for Tree<Regular> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ctx = self.context();

        let root = self.root;
        let inner = self.inner();
        let level = self.level();
        let show_count = ctx.count;
        let mut file_count_data = vec![];

        let mut descendants = root.descendants(inner).skip(1).peekable();

        let mut display_node = |node_id: NodeId, prefix: &str| -> fmt::Result {
            let node = inner[node_id].get();

            node.display(f, prefix, ctx)?;

            if show_count {
                let count = Self::compute_file_count(node_id, inner);
                file_count_data.push(count);
            }

            writeln!(f)
        };

        display_node(root, "")?;

        let mut prefix_components = vec![""];

        while let Some(current_node_id) = descendants.next() {
            let mut current_prefix_components = prefix_components.clone();

            let current_node = inner[current_node_id].get();

            let current_node_depth = current_node.depth();

            let mut siblings = current_node_id.following_siblings(inner).skip(1).peekable();

            let last_sibling = siblings.peek().is_none();

            let theme = if current_node.is_symlink() {
                styles::get_link_theme().unwrap()
            } else {
                styles::get_tree_theme().unwrap()
            };

            let prefix_part = if last_sibling {
                theme.get("uprt").unwrap()
            } else {
                theme.get("vtrt").unwrap()
            };

            current_prefix_components.push(prefix_part);

            let prefix = current_prefix_components.join("");

            if current_node_depth <= level {
                display_node(current_node_id, &prefix)?;
            } else {
                break;
            }

            if let Some(next_id) = descendants.peek() {
                let next_node = inner[*next_id].get();

                let next_node_depth = next_node.depth();

                if next_node_depth == current_node_depth + 1 {
                    if last_sibling {
                        prefix_components.push(styles::SEP);
                    } else {
                        let prefix = theme.get("vt").unwrap();
                        prefix_components.push(prefix);
                    }
                } else if next_node_depth < current_node_depth {
                    let depth_delta = current_node_depth - next_node_depth;

                    prefix_components.truncate(prefix_components.len() - depth_delta);
                }
            }
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}

impl Display for Tree<Report> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tree = self.inner();
        let root = self.root();
        let ctx = self.context();
        let max_depth = ctx.level().unwrap_or(usize::MAX);
        let dir = ctx.dir();
        let prefix_kind = ctx.prefix;
        let show_count = ctx.count;
        let mut file_count_data = vec![];

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

        if show_count {
            let count = Self::compute_file_count(root, tree);
            file_count_data.push(count);
        }

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

            if show_count {
                let count = Self::compute_file_count(node_id, tree);
                file_count_data.push(count);
            }

            if node.depth() > max_depth {
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

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}
