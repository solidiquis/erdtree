use crate::render::{
    styles,
    tree::{count::FileCount, Tree},
};
use indextree::NodeId;
use std::fmt::{self, Display, Formatter};

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
        let level = ctx.level();
        let mut file_count_data = vec![];

        let mut descendants = root.descendants(inner).skip(1).peekable();

        let mut display_node = |node_id: NodeId, prefix: &str| -> fmt::Result {
            let node = inner[node_id].get();

            node.tree_display(f, prefix, ctx)?;
            file_count_data.push(Self::compute_file_count(node_id, inner));

            writeln!(f)
        };

        display_node(root, "")?;

        let mut prefix_components = vec![""];

        while let Some(current_node_id) = descendants.next() {
            let mut current_prefix_components = prefix_components.clone();

            let current_node = inner[current_node_id].get();

            let current_depth = current_node.depth();

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

            if current_depth <= level {
                display_node(current_node_id, &prefix)?;
            }

            if let Some(next_id) = descendants.peek() {
                let next_node = inner[*next_id].get();

                let next_depth = next_node.depth();

                if next_depth == current_depth + 1 {
                    if last_sibling {
                        prefix_components.push(styles::SEP);
                    } else {
                        let prefix = theme.get("vt").unwrap();
                        prefix_components.push(prefix);
                    }
                } else if next_depth < current_depth {
                    let depth_delta = current_depth - next_depth;

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
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        let descendants = root.descendants(tree);

        for node_id in descendants {
            let node = tree[node_id].get();

            if node.depth() > max_depth {
                continue;
            }

            node.report_display(f, ctx)?;
            file_count_data.push(Self::compute_file_count(node_id, tree));
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}
