use crate::{error::prelude::*, file::File, user::Context};
use ahash::{HashMap, HashSet};
use ignore::Walk;
use indextree::{Arena, NodeId, Traverse};
use std::{
    convert::TryFrom,
    fs,
    ops::Deref,
    path::PathBuf,
};

/// Utilities for parallel traversal and associated event publishing/consuming.
mod parallel;

/// Concerned with initializing [`Walk`] and [`WalkParallel`] from the user [`Context`].
mod walker;

/// Representation of the file-tree that is traversed starting from the root directory whose index
/// in the underlying `arena` is `root_id`.
pub struct FileTree {
    root_id: NodeId,
    arena: Arena<File>,
}

/// Errors associated with [`FileTree`].
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    #[error("Failed to query the root directory")]
    RootDirMissing,

    #[error("Expected ancestor node to exist in arena")]
    ParentNode,

    #[error("Failed to compute directory size")]
    MissingDirSize,
}

impl FileTree {
    pub fn new(root_id: NodeId, arena: Arena<File>) -> Self {
        Self { root_id, arena }
    }

    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    /// Initializes a [`FileTree`] completely on one thread.
    pub fn init(ctx: &Context) -> Result<Self> {
        let mut walker = Walk::try_from(ctx)?;

        let root_entry = walker
            .next()
            .ok_or(TreeError::RootDirMissing)
            .into_report(ErrorCategory::Internal)?;

        let root_node = root_entry
            .into_report(ErrorCategory::Internal)
            .and_then(|data| File::init(data, ctx).into_report(ErrorCategory::Internal))?;

        let mut arena = Arena::new();
        let root_node_id = arena.new_node(root_node);
        let mut current_dir_id = root_node_id;

        let mut dirsize_map = HashMap::default();
        let mut dir_stack = vec![];

        dirsize_map.insert(root_node_id, 0);

        // To prevent two or more files with the same underlying inode from
        // counted more than once which would lead to inaccurate disk usage.
        let mut inode_set = HashSet::default();

        for dent in walker {
            let node = match dent
                .into_report(ErrorCategory::Warning)
                .and_then(|data| File::init(data, ctx).into_report(ErrorCategory::Warning))
            {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{e}");
                    continue;
                },
            };

            let size = match node.inode() {
                Ok(inode) => inode_set
                    .insert(inode)
                    .then_some(node.size().value())
                    .unwrap_or(0),
                Err(e) => {
                    log::error!("{e}");
                    node.size().value()
                },
            };

            // Check if new node is directory before we transfer ownership to `arena`.
            let is_dir = node.file_type().is_some_and(|ft| ft.is_dir());

            let new_node_id = arena.new_node(node);

            current_dir_id.append(new_node_id, &mut arena);

            let parent_dir_id = new_node_id
                .ancestors(&arena)
                .nth(1) // skip self
                .ok_or(TreeError::ParentNode)
                .into_report(ErrorCategory::Internal)
                .context(error_source!())?;

            if let Some(parent_size) = dirsize_map.get_mut(&parent_dir_id) {
                *parent_size += size;
            } else {
                dirsize_map.insert(parent_dir_id, size);
            }

            if is_dir {
                dir_stack.push(new_node_id);
                current_dir_id = new_node_id;
            }
        }

        while let Some(node_id) = dir_stack.pop() {
            let node_size = dirsize_map
                .remove(&node_id)
                .ok_or(TreeError::MissingDirSize)
                .into_report(ErrorCategory::Internal)
                .context(error_source!())?;

            let parent_size = node_id
                .ancestors(&arena)
                .nth(1) // skip self
                .and_then(|parent_dir_id| dirsize_map.get_mut(&parent_dir_id))
                .ok_or(TreeError::ParentNode)
                .into_report(ErrorCategory::Internal)
                .context(error_source!())?;

            *parent_size += node_size;
        }

        Ok(Self::new(root_node_id, arena))
    }

    /// Like [`FileTree::init`] but leverages parallelism for disk-reads and [`File`] initialization.
    pub fn init_parallel(ctx: &Context) -> Result<Self> {
        let mut arena = Arena::new();
        let mut branches = HashMap::<PathBuf, Vec<NodeId>>::default();

        parallel::run(ctx, |file| {
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
            .ok_or(TreeError::RootDirMissing)
            .into_report(ErrorCategory::Internal)?;

        let mut dfs_queue = vec![root_id];
        let mut inode_set = HashSet::default();

        'outer: while let Some(node_id) = dfs_queue.last() {
            let current_id = *node_id;

            let current_node_path = arena[current_id].get().path();

            let Some(children) = branches.get_mut(current_node_path) else {
                dfs_queue.pop();
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
                        }
                    };
                    (size, is_dir, inode)
                };

                if child_is_dir {
                    dfs_queue.push(child_node_id);
                    continue 'outer;
                }

                if inode_set.insert(child_inode) {
                    *arena[current_id].get_mut().size_mut() += child_size;
                }
            }

            dfs_queue.pop();

            if let Some(parent_id) = current_id.ancestors(&arena).skip(1).nth(0) {
                let current_dir_size = {
                    arena[current_id].get().size().value()
                };
                *arena[parent_id].get_mut().size_mut() += current_dir_size;
            }
        }
        
        Ok(Self { root_id, arena })
    }

    pub fn traverse(&self) -> Traverse<'_, File> {
        self.root_id.traverse(&self.arena)
    }
}

impl Deref for FileTree {
    type Target = Arena<File>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}
