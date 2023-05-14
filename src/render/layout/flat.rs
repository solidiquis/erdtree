use crate::{
    render::{
        grid::{self, Row},
        Engine, Flat,
    },
    tree::{count::FileCount, Tree},
};
use indextree::NodeEdge;
use std::fmt::{self, Display};

impl Display for Engine<Flat> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctx = self.context();
        let tree = self.tree();
        let arena = tree.arena();
        let root_id = tree.root_id();
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        for edge in root_id.reverse_traverse(arena) {
            let node_id = match edge {
                NodeEdge::Start(id) => id,
                NodeEdge::End(_) => continue,
            };
            let node = arena[node_id].get();

            if node.depth() > max_depth {
                continue;
            }

            let row = Row::<grid::Flat>::new(node, ctx, None);

            writeln!(f, "{row}")?;

            file_count_data.push(Tree::compute_file_count(node_id, arena));
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}
