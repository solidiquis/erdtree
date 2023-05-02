use crate::{
    render::{Engine, theme},
    styles,
    tree::{count::FileCount, node::Node, Tree},
};
use indextree::{NodeEdge, NodeId};
use std::fmt::{self, Display};

pub struct Regular;

impl Display for Engine<Regular> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctx = self.context();
        let tree = self.tree();
        let root_id = tree.root_id();
        let arena = tree.arena();
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        let mut display_node = |node_id: NodeId, node: &Node, prefix: &str| -> fmt::Result {
            node.tree_display(f, prefix, ctx)?;
            file_count_data.push(Tree::compute_file_count(node_id, arena));
            writeln!(f)
        };

        let mut get_theme = if ctx.follow {
            theme::link_theme_getter()
        } else {
            theme::regular_theme_getter()
        };

        let mut base_prefix_components = vec![""];

        let mut tree_edges = root_id.reverse_traverse(arena).skip(1).peekable();

        while let Some(node_edge) = tree_edges.next() {
            let current_node_id = match node_edge {
                NodeEdge::Start(id) => id,

                NodeEdge::End(id) => {
                    let current_node = arena[id].get();

                    if !current_node.is_dir() || id.children(arena).count() == 0 {
                        continue;
                    }

                    let theme = get_theme(current_node);

                    let topmost_sibling = id.following_siblings(arena).nth(1).is_none();

                    if topmost_sibling {
                        base_prefix_components.push(styles::SEP);
                    } else {
                        base_prefix_components.push(theme.get("vt").unwrap());
                    }

                    continue;
                }
            };

            let current_node = arena[current_node_id].get();

            let node_depth = current_node.depth();

            let topmost_sibling = current_node_id.following_siblings(arena).nth(1).is_none();

            let theme = get_theme(current_node);

            if node_depth <= max_depth {
                if node_depth == 0 {
                    display_node(current_node_id, current_node, "")?;
                } else {
                    let prefix_part = if topmost_sibling {
                        theme.get("drt").unwrap()
                    } else {
                        theme.get("vtrt").unwrap()
                    };

                    let mut current_prefix_components = base_prefix_components.clone();

                    current_prefix_components.push(prefix_part);

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
