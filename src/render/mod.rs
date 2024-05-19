use crate::{
    error::prelude::*,
    file,
    user::{args::Layout, Context},
};
use indextree::{NodeEdge, NodeId};
use std::io::{self, Write};

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

pub struct Renderer<'a> {
    ctx: &'a Context,
    file_tree: &'a file::Tree,
}

impl<'a> Renderer<'a> {
    pub fn new(ctx: &'a Context, file_tree: &'a file::Tree) -> Self {
        Self { ctx, file_tree }
    }

    pub fn render(self) -> Result<()> {
        let out = match self.ctx.layout {
            Layout::Tree => self.tree(),
            Layout::InvertedTree => self.inverted_tree(),
            Layout::Flat => self.flat(),
        }?;

        writeln!(io::stdout(), "{out}").into_report(ErrorCategory::Warning)?;

        Ok(())
    }

    fn inverted_tree(&self) -> Result<String> {
        let arena = self.file_tree.arena();
        let root = self.file_tree.root_id();
        let max_depth = self.ctx.level();

        let mut buf = String::new();

        let is_first_sibling = |node_id: NodeId, depth: usize| {
            (depth > 0)
                .then(|| node_id.following_siblings(arena).nth(1).is_none())
                .unwrap_or(false)
        };

        let mut inherited_prefix_components = vec![""];

        let mut formatter = row::formatter(&mut buf, self.ctx)?;

        let mut reverse_traverse = root.reverse_traverse(arena);
        reverse_traverse.next();

        for node_edge in reverse_traverse {
            let (node, node_id, depth) = match node_edge {
                NodeEdge::Start(node_id) => {
                    let node = arena[node_id].get();
                    let depth = node.depth();

                    if node.is_dir() {
                        inherited_prefix_components.pop();
                    }

                    if depth > max_depth {
                        continue;
                    }

                    (node, node_id, depth)
                }
                NodeEdge::End(node_id) => {
                    let node = arena[node_id].get();
                    let depth = node.depth();

                    if node.is_dir() {
                        if is_first_sibling(node_id, depth) {
                            inherited_prefix_components.push(SEP);
                        } else {
                            inherited_prefix_components.push(VLINE);
                        }
                    }
                    continue;
                }
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

            let _ = formatter(node, prefix);
        }

        drop(formatter);

        Ok(buf)
    }

    pub fn tree(&self) -> Result<String> {
        let arena = self.file_tree.arena();
        let root = self.file_tree.root_id();
        let max_depth = self.ctx.level();

        let mut buf = String::new();

        let is_last_sibling = |node_id: NodeId, depth: usize| {
            (depth > 0)
                .then(|| node_id.following_siblings(arena).nth(1).is_none())
                .unwrap_or(false)
        };

        let mut inherited_prefix_components = vec![""];

        let mut formatter = row::formatter(&mut buf, self.ctx)?;

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

                    if node.is_dir() && depth < max_depth {
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

            let _ = formatter(node, prefix);

            if node.is_dir() && depth < max_depth {
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

    fn flat(&self) -> Result<String> {
        let arena = self.file_tree.arena();
        let root = self.file_tree.root_id();
        let max_depth = self.ctx.level();
        let mut buf = String::new();

        let mut formatter = row::formatter(&mut buf, self.ctx)?;

        for node_edge in root.traverse(arena) {
            let node_id = match node_edge {
                NodeEdge::Start(_) => continue,
                NodeEdge::End(id) if id.is_removed(arena) => continue,
                NodeEdge::End(id) => id,
            };

            let node = arena[node_id].get();

            if node.depth() > max_depth {
                continue;
            }

            formatter(node, "".to_string()).into_report(ErrorCategory::Warning)?;
        }
        drop(formatter);

        Ok(buf)
    }
}
