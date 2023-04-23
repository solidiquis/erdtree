use crate::render::{
    styles,
    tree::{count::FileCount, node::Node, Tree},
};
use indextree::{NodeEdge, NodeId};
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

/// Empty trait used to constrain generic parameter `display_variant` of [Tree].
pub trait TreeVariant {}

/// For printing output with colored ANSI escapes.
pub struct Regular {}

/// Prints the invered tree with colored ANSI escapes.
pub struct Inverted {}

/// For generating plain-text report of disk usage without ASCII tree.
pub struct Flat {}

impl TreeVariant for Regular {}
impl TreeVariant for Flat {}
impl TreeVariant for Inverted {}

impl Display for Tree<Inverted> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ctx = self.context();

        let root_id = self.root_id;
        let arena = self.arena();
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        let mut display_node = |node_id: NodeId, node: &Node, prefix: &str| -> fmt::Result {
            node.tree_display(f, prefix, ctx)?;
            file_count_data.push(Self::compute_file_count(node_id, arena));

            writeln!(f)
        };

        let get_theme = |node: &Node| if node.is_symlink() {
            styles::get_link_theme().unwrap()
        } else {
            styles::get_tree_theme().unwrap()
        };

        let mut base_prefix_components = vec![""];

        let mut tree_edges = root_id.reverse_traverse(arena).peekable();

        while let Some(node_edge) = tree_edges.next() {
            let current_node_id = match node_edge {
                NodeEdge::Start(id) => id,
                NodeEdge::End(id) => {
                    let current_node = arena[id].get();

                    if !current_node.is_dir() || id.children(arena).count() == 0 {
                        continue;
                    }

                    let theme = get_theme(&current_node);

                    let topmost_sibling = id.following_siblings(arena).skip(1).next().is_none();

                    if current_node.depth() > 0 {
                        if topmost_sibling {
                            base_prefix_components.push(styles::SEP)
                        } else {
                            base_prefix_components.push(theme.get("vt").unwrap())
                        }
                    }

                    continue;
                },
            };

            let current_node = arena[current_node_id].get();

            let node_depth = current_node.depth();

            let mut current_prefix_components = Cow::from(&base_prefix_components);

            let topmost_sibling = current_node_id.following_siblings(arena).skip(1).next().is_none();

            let theme = get_theme(&current_node);

            if node_depth <= max_depth {
                if node_depth == 0 {
                    display_node(current_node_id, current_node, "")?;
                } else {
                    let prefix_part = if topmost_sibling {
                        theme.get("drt").unwrap()
                    } else {
                        theme.get("vtrt").unwrap()
                    };
                    current_prefix_components.to_mut().push(prefix_part);

                    let prefix = current_prefix_components.join("");

                    display_node(current_node_id, current_node, &prefix)?;
                }
            }

            if let Some(NodeEdge::Start(next_id)) = tree_edges.peek() {
                let next_node = arena[*next_id].get();

                if next_node.depth() < node_depth {
                    base_prefix_components.pop();
                }
            }
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}

impl Display for Tree<Regular> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ctx = self.context();

        let root_id = self.root_id;
        let arena = self.arena();
        let level = ctx.level();
        let mut file_count_data = vec![];

        let mut descendants = root_id.descendants(arena).skip(1).peekable();

        let mut display_node = |node_id: NodeId, node: &Node, prefix: &str| -> fmt::Result {
            node.tree_display(f, prefix, ctx)?;
            file_count_data.push(Self::compute_file_count(node_id, arena));
            writeln!(f)
        };

        display_node(root_id, arena[root_id].get(), "")?;

        let mut base_prefix_components = vec![""];

        while let Some(current_node_id) = descendants.next() {
            let mut current_prefix_components = Cow::from(&base_prefix_components);

            let current_node = arena[current_node_id].get();

            let current_depth = current_node.depth();

            let mut siblings = current_node_id.following_siblings(arena).skip(1).peekable();

            let last_sibling = siblings.peek().is_none();

            let theme = if current_node.is_symlink() {
                styles::get_link_theme().unwrap()
            } else {
                styles::get_tree_theme().unwrap()
            };

            if current_depth <= level {
                let prefix_part = if last_sibling {
                    theme.get("uprt").unwrap()
                } else {
                    theme.get("vtrt").unwrap()
                };

                current_prefix_components.to_mut().push(prefix_part);

                let prefix = current_prefix_components.join("");

                display_node(current_node_id, current_node, &prefix)?;
            }

            if let Some(next_id) = descendants.peek() {
                let next_node = arena[*next_id].get();

                let next_depth = next_node.depth();

                if next_depth == current_depth + 1 {
                    if last_sibling {
                        base_prefix_components.push(styles::SEP);
                    } else {
                        let prefix = theme.get("vt").unwrap();
                        base_prefix_components.push(prefix);
                    }
                } else if next_depth < current_depth {
                    let depth_delta = current_depth - next_depth;

                    base_prefix_components.truncate(base_prefix_components.len() - depth_delta);
                }
            }
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}

impl Display for Tree<Flat> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tree = self.arena();
        let root_id = self.root_id();
        let ctx = self.context();
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        let descendants = root_id.descendants(tree);

        for node_id in descendants {
            let node = tree[node_id].get();

            if node.depth() > max_depth {
                continue;
            }

            node.flat_display(f, ctx)?;
            file_count_data.push(Self::compute_file_count(node_id, tree));
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}
