use crate::{error::prelude::*, file, user::{args::Layout, Context}};
use indextree::{NodeEdge, NodeId};

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

/// Concerned with the presentation of a single [`crate::file::File`] which constitutes a single
/// row in the program output.
mod row;

pub fn output(file_tree: &file::Tree, ctx: &Context) -> Result<String> {
    match ctx.layout {
        Layout::Regular => tree(file_tree, ctx),
        Layout::Inverted => inverted_tree(file_tree, ctx),
        Layout::Flat => todo!(),
        Layout::Iflat => todo!(),
    }
}

fn tree(file_tree: &file::Tree, ctx: &Context) -> Result<String> {
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

    let mut formatter = row::formatter(&mut buf, ctx);

    let mut reverse_traverse = root.reverse_traverse(arena);
    reverse_traverse.next();

    for node_edge in reverse_traverse {
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

        let prefix = format!(
            "{}{}",
            inherited_prefix_components.join(""),
            (depth > 0)
                .then(|| {
                    is_first_sibling(node_id, depth)
                        .then_some(UL_CORNER)
                        .unwrap_or(ROTATED_T)
                })
                .unwrap_or("")
        );

        if let Err(e) = formatter(&node, prefix) {
            log::warn!("{e}");
        }
    }

    drop(formatter);

    Ok(buf)
}

pub fn inverted_tree(file_tree: &file::Tree, ctx: &Context) -> Result<String> {
    let arena = file_tree.arena();
    let root = file_tree.root_id();
    let max_depth = ctx.level();

    let mut buf = String::new();

    let is_last_sibling = |node_id: NodeId, depth: usize| {
        (depth > 0)
            .then(|| node_id.following_siblings(arena).skip(1).next().is_none())
            .unwrap_or(false)
    };

    let mut inherited_prefix_components = vec![""];

    let mut formatter = row::formatter(&mut buf, ctx);

    let mut traverse = root.traverse(arena);
    traverse.next();

    formatter(arena[root].get(), "".to_string())
        .into_report(ErrorCategory::Internal)
        .context(error_source!())?;

    for node_edge in traverse {
        let (node, node_id, depth) = match node_edge {
            NodeEdge::Start(node_id) => {
                let node = arena[node_id].get();
                let depth = node.depth();

                if depth > max_depth {
                    continue;
                }

                (node, node_id, depth)
            }
            NodeEdge::End(node_id) => {
                let node = arena[node_id].get();
                let depth = node.depth();

                if utils::node_is_dir(&node) && depth < max_depth {
                    inherited_prefix_components.pop();
                }
                continue;
            }
        };

        let prefix = format!(
            "{}{}",
            inherited_prefix_components.join(""),
            (depth > 0)
                .then(|| {
                    is_last_sibling(node_id, depth)
                        .then_some(BL_CORNER)
                        .unwrap_or(ROTATED_T)
                })
                .unwrap_or("")
        );

        if let Err(e) = formatter(&node, prefix) {
            log::warn!("{e}");
        }

        if utils::node_is_dir(&node) && depth < max_depth {
            if is_last_sibling(node_id, depth) {
                inherited_prefix_components.push(SEP);
            } else {
                inherited_prefix_components.push(VLINE);
            }
        }
    }

    drop(formatter);

    Ok(buf)
}

mod utils {
    use crate::file::File;

    pub fn node_is_dir(node: &File) -> bool {
        node.file_type().is_some_and(|ft| ft.is_dir())
    }
}
