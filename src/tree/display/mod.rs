use super::FileTree;
use crate::{error::prelude::*, user::Context};
use indextree::{NodeEdge, NodeId};
use std::fmt::Write;

/// Concerned with properties of individual columns in the output.
pub mod column;

/// Used as general placeholder for an empty field.
pub const PLACEHOLDER: &str = "-";

/// Used for padding between tree branches.
pub const SEP: &str = "   ";

/// The `│` box drawing character.
pub const VLINE: &str = "\u{2502}  ";

/// The `┌─` box drawing character.
pub const UL_CORNER: &str = "\u{250C}\u{2500} ";

/// The `└─` box drawing characters.
pub const BL_CORNER: &str = "\u{2514}\u{2500} ";

/// The `├─` box drawing characters.
pub const ROTATED_T: &str = "\u{251C}\u{2500} ";

pub fn tree(file_tree: &FileTree, ctx: &Context) -> Result<String> {
    let arena = file_tree.arena();
    let root = file_tree.root_id();
    let max_depth = ctx.level();
    let mut buf = String::new();

    let is_first_sibling = |node_id: NodeId, depth: usize| {
        (depth > 0)
            .then(|| node_id.following_siblings(arena).skip(1).next().is_none())
            .unwrap_or(false)
    };

    let mut inherited_prefix_components = vec![""];

    for node_edge in root.reverse_traverse(arena) {
        let (node, node_id, depth) = match node_edge {
            NodeEdge::Start(node_id) => {
                let node = arena[node_id].get();
                let depth = node.depth();

                if utils::node_is_dir(&node) {
                    inherited_prefix_components.pop();
                }

                if depth > max_depth {
                    continue;
                }

                (node, node_id, depth)
            },
            NodeEdge::End(node_id) => {
                let node = arena[node_id].get();
                let depth = node.depth();

                if depth == 0 {
                    continue;
                }

                if utils::node_is_dir(&node) {
                    if is_first_sibling(node_id, depth) {
                        inherited_prefix_components.push(SEP);
                    } else {
                        inherited_prefix_components.push(VLINE);
                    }
                }
                continue;
            },
        };

        let name = node.file_name().to_string_lossy();
        let inherited_prefix = inherited_prefix_components.join("");

        let prefix = (depth > 0)
            .then(|| {
                is_first_sibling(node_id, depth)
                    .then_some(UL_CORNER)
                    .unwrap_or(ROTATED_T)
            })
            .unwrap_or("");

        writeln!(buf, "{inherited_prefix}{prefix}{name}")
            .into_report(ErrorCategory::Internal)
            .context(error_source!())?;
    }

    Ok(buf)
}

mod utils {
    use crate::file::File;

    pub fn node_is_dir(node: &File) -> bool {
        node.file_type().is_some_and(|ft| ft.is_dir())
    }
}
