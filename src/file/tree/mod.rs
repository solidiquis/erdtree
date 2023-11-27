use crate::{
    error::prelude::*,
    file::File,
    user::{column, Context},
};
use ahash::{HashMap, HashSet};
use indextree::{Arena, NodeId};
use std::{ops::Deref, path::PathBuf};

/// Parallel disk reading
mod traverse;

/// Representation of the file-tree that is traversed starting from the root directory whose index
/// in the underlying `arena` is `root_id`.
pub struct Tree {
    root_id: NodeId,
    arena: Arena<File>,
}

/// Errors associated with [`Tree`].
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    #[error("Failed to extrapolate the root directory")]
    RootDir,
}

impl Tree {
    /// Like [`Tree::init`] but leverages parallelism for disk-reads and [`File`] initialization.
    pub fn init(ctx: &Context) -> Result<(Self, column::Metadata)> {
        let mut arena = Arena::new();
        let mut branches = HashMap::<PathBuf, Vec<NodeId>>::default();
        let mut col_md = column::Metadata::default();
        let mut maybe_root_id = None;

        traverse::run(ctx, |file| {
            #[cfg(unix)]
            col_md.update_unix_attrs_widths(&file, ctx);

            let node_id = arena.new_node(file);
            let file = arena[node_id].get();
            let file_path = file.path();

            maybe_root_id = (file.depth() == 0).then_some(node_id).or(maybe_root_id);

            if let Some(parent) = file_path.parent() {
                if let Some(nodes) = branches.get_mut(parent) {
                    nodes.push(node_id);
                } else {
                    branches.insert(parent.to_path_buf(), vec![node_id]);
                }
            }
            Ok(())
        })?;

        let root_id = maybe_root_id
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
                        Ok(value) => {
                            #[cfg(unix)]
                            col_md.update_inode_attr_widths(&value);
                            value
                        },
                        Err(err) => {
                            log::warn!(
                                "Failed to query inode of {} which may affect disk usage report: {err}",
                                child_node.path().display(),
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

            if let Some(parent_id) = current_id.ancestors(&arena).nth(1) {
                let current_dir_size = { arena[current_id].get().size().value() };
                *arena[parent_id].get_mut().size_mut() += current_dir_size;
            }
        }

        col_md.update_size_width(arena[root_id].get(), ctx);

        let tree = Self { root_id, arena };

        Ok((tree, col_md))
    }

    pub fn prune(&mut self) {
        let to_prune = self
            .root_id
            .descendants(&self.arena)
            .filter(|n| {
                self.arena[*n]
                    .get()
                    .file_type()
                    .is_some_and(|ft| ft.is_dir() && n.children(&self.arena).count() == 0)
            })
            .collect::<Vec<_>>();

        to_prune
            .into_iter()
            .for_each(|n| n.remove_subtree(&mut self.arena));
    }

    pub fn init_without_disk_usage() -> Self {
        todo!()
    }

    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    pub fn arena(&self) -> &Arena<File> {
        &self.arena
    }
}

impl Deref for Tree {
    type Target = Arena<File>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}
