use crate::{error::prelude::*, file::File, user::Context};
use ahash::{HashMap, HashSet};
use indextree::{Arena, NodeId};
use std::{fs, ops::Deref, path::PathBuf};

/// Concerned with how to display user-presentable tree output.
pub mod display;
use display::column;

/// Parallel disk reading
mod traverse;

/// Representation of the file-tree that is traversed starting from the root directory whose index
/// in the underlying `arena` is `root_id`.
pub struct FileTree {
    root_id: NodeId,
    arena: Arena<File>,
    column_widths: column::Widths,
}

/// Errors associated with [`FileTree`].
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    #[error("Failed to extrapolate the root directory")]
    RootDir,
}

impl FileTree {
    /// Like [`FileTree::init`] but leverages parallelism for disk-reads and [`File`] initialization.
    pub fn init(ctx: &Context) -> Result<Self> {
        let mut arena = Arena::new();
        let mut branches = HashMap::<PathBuf, Vec<NodeId>>::default();
        let mut column_widths = column::Widths::default();

        traverse::run(ctx, |file| {
            column_widths.update(&file, ctx);

            let node_id = arena.new_node(file);
            let file = arena[node_id].get();
            let file_path = file.path();

            if let Some(parent) = file_path.parent() {
                if let Some(nodes) = branches.get_mut(parent) {
                    nodes.push(node_id);
                } else {
                    branches.insert(parent.to_path_buf(), vec![node_id]);
                }
            } else {
                let presumable_system_root = fs::canonicalize(file_path)
                    .into_report(ErrorCategory::Internal)
                    .context("Failed to canonicalize presumable root directory")?;

                branches.insert(presumable_system_root, vec![]);
            }
            Ok(())
        })?;

        let root_path = ctx.dir_canonical()?;

        let root_id = root_path
            .parent()
            .and_then(|p| branches.get(p))
            .and_then(|b| (b.len() == 1).then(|| b[0]))
            .ok_or(TreeError::RootDir)
            .into_report(ErrorCategory::Internal)
            .context(error_source!())?;

        let mut dfs_stack = vec![root_id];
        let mut inode_set = HashSet::default();

        'outer: while let Some(node_id) = dfs_stack.last() {
            let current_id = *node_id;

            let current_node_path = arena[current_id].get().path();

            let Some(children) = branches.get_mut(current_node_path) else {
                dfs_stack.pop();
                continue;
            };

            while let Some(child_node_id) = children.pop() {
                current_id.append(child_node_id, &mut arena);

                let (child_size, child_is_dir, child_inode) = {
                    let child_node = arena[child_node_id].get();
                    let is_dir = child_node.file_type().is_some_and(|f| f.is_dir());
                    let size = child_node.size().value();
                    let inode = match child_node.inode() {
                        Ok(value) => value,
                        Err(err) => {
                            log::warn!(
                                "Failed to query inode of {} which may affect disk usage report: {}",
                                child_node.path().display(),
                                err
                            );
                            continue;
                        },
                    };
                    (size, is_dir, inode)
                };

                if child_is_dir {
                    dfs_stack.push(child_node_id);
                    continue 'outer;
                }

                if inode_set.insert(child_inode) {
                    *arena[current_id].get_mut().size_mut() += child_size;
                }

                *arena[current_id].get_mut().size_mut() += inode_set
                    .insert(child_inode)
                    .then_some(child_size)
                    .unwrap_or(0);
            }

            dfs_stack.pop();

            if let Some(parent_id) = current_id.ancestors(&arena).skip(1).nth(0) {
                let current_dir_size = { arena[current_id].get().size().value() };
                *arena[parent_id].get_mut().size_mut() += current_dir_size;
            }
        }

        Ok(Self {
            root_id,
            arena,
            column_widths,
        })
    }

    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    pub fn arena(&self) -> &Arena<File> {
        &self.arena
    }

    pub fn node_is_dir(&self, id: NodeId) -> bool {
        self[id].get().file_type().is_some_and(|ft| ft.is_dir())
    }
}

impl Deref for FileTree {
    type Target = Arena<File>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}
