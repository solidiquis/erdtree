use crate::{
    render::Engine,
    tree::{count::FileCount, Tree},
};
use std::fmt::{self, Display};

pub struct Flat;

impl Display for Engine<Flat> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctx = self.context();
        let tree = self.tree();
        let arena = tree.arena();
        let root_id = tree.root_id();
        let max_depth = ctx.level();
        let mut file_count_data = vec![];

        let descendants = root_id.descendants(arena);

        for node_id in descendants {
            let node = arena[node_id].get();

            if node.depth() > max_depth {
                continue;
            }

            node.flat_display(f, ctx)?;
            file_count_data.push(Tree::compute_file_count(node_id, arena));
        }

        if !file_count_data.is_empty() {
            write!(f, "\n{}", FileCount::from(file_count_data))?;
        }

        Ok(())
    }
}
