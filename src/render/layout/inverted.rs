use crate::{
    render::{
        grid::{self, Row},
        theme, Engine, Inverted,
    },
    styles,
    tree::{count::FileCount, node::Node, Tree},
};
use indextree::NodeId;
use std::fmt::{self, Display};

impl Display for Engine<Inverted> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctx = self.context();
        let tree = self.tree();

        let root_id = tree.root_id();
        let arena = tree.arena();
        let level = ctx.level();
        let mut file_count_data = vec![];

        let mut descendants = root_id.descendants(arena).skip(1).peekable();

        let mut display_node = |node_id: NodeId, node: &Node, prefix: &str| -> fmt::Result {
            let row = Row::<grid::Tree>::new(node, ctx, Some(prefix));
            file_count_data.push(Tree::compute_file_count(node_id, arena));
            writeln!(f, "{row}")
        };

        display_node(root_id, arena[root_id].get(), "")?;

        let mut get_theme = if ctx.follow {
            theme::link_theme_getter()
        } else {
            theme::regular_theme_getter()
        };

        let mut base_prefix_components = vec![""];

        while let Some(current_node_id) = descendants.next() {
            let current_node = arena[current_node_id].get();

            let current_depth = current_node.depth();

            let mut siblings = current_node_id.following_siblings(arena).skip(1).peekable();

            let last_sibling = siblings.peek().is_none();

            let theme = get_theme(current_node);

            if current_depth <= level {
                let prefix_part = if last_sibling {
                    theme.get("uprt").unwrap()
                } else {
                    theme.get("vtrt").unwrap()
                };

                let mut current_prefix_components = base_prefix_components.clone();

                current_prefix_components.push(prefix_part);

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
