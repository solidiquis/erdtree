use super::order;
use crate::{
    error::prelude::*,
    file::File,
    progress,
    user::{
        args::{Layout, Sort, SortType},
        column, Context,
    },
};
use ahash::{HashMap, HashSet};
use indextree::{Arena, NodeId};
use std::{ops::Deref, path::PathBuf};

/// Concerned with pruning and filtering via file-type, globbing, and regular expressions.
mod filter;

/// Parallel disk reading.
mod traverse;

/// Representation of the file-tree that is traversed starting from the root directory whose index
/// in the underlying `arena` is `root_id`.
pub struct Tree {
    root_id: NodeId,
    arena: Arena<File>,
}

/// Intermediate components that will be used to construct the final [`Tree`] data structure.
pub struct TransitionState {
    arena: Arena<File>,
    branches: HashMap<PathBuf, Vec<NodeId>>,
    column_metadata: column::Metadata,
    root_id: NodeId,
}

/// Errors associated with [`Tree`].
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    #[error("Failed to extrapolate the root directory")]
    RootDir,
}

impl Tree {
    /// Like [`Tree::init`] but leverages parallelism for disk-reads and [`File`] initialization.
    pub fn init(ctx: &Context) -> Result<(Self, super::Accumulator, column::Metadata)> {
        let (
            TransitionState {
                mut arena,
                mut branches,
                mut column_metadata,
                root_id,
            },
            accumulator,
        ) = Self::load(ctx)?;

        let mut dir_stack = vec![root_id];
        let mut inode_set = HashSet::default();

        // Keeps track of which directory entry we're at for each directory while doing depth first
        // traversal. The key is the NodeID of a directory with the value being an index into a
        // slice of the directory's children.
        let mut dirent_cursor_map = HashMap::<NodeId, usize>::default();

        // Map of dynamically computed directory sizes.
        let mut dirsize_map = HashMap::<NodeId, u64>::default();

        'outer: while let Some(node_id) = dir_stack.last() {
            let current_dir = *node_id;

            let current_node_path = arena[current_dir].get().path();

            let Some(dirents) = branches.get_mut(current_node_path) else {
                dir_stack.pop();
                continue;
            };

            let current_dirsize = dirsize_map.entry(current_dir).or_insert(0);
            let dirent_cursor = dirent_cursor_map.entry(current_dir).or_insert(0);

            for dirent_node_id in &dirents[*dirent_cursor..] {
                *dirent_cursor += 1;
                let dirent_node_id = *dirent_node_id;
                let dirent_node = arena[dirent_node_id].get();

                if dirent_node.is_dir() {
                    dir_stack.push(dirent_node_id);
                    continue 'outer;
                }

                if let Ok(inode) = dirent_node.inode() {
                    #[cfg(unix)]
                    column_metadata.update_inode_attr_widths(&inode);

                    *current_dirsize += inode_set
                        .insert(inode)
                        .then(|| dirent_node.size().value())
                        .unwrap_or(0);
                }
            }

            dir_stack.pop();

            // To play nicely with aliasing rules around mutable refs
            let current_dirsize = *current_dirsize;

            if let Some(parent_dir) = dir_stack.last() {
                if let Some(parent_dirsize) = dirsize_map.get_mut(parent_dir) {
                    *parent_dirsize += current_dirsize;
                }
            }
        }

        match order::comparator(ctx.sort, ctx.dir_order) {
            Some(comparator) => match ctx.sort_type {
                SortType::Flat if matches!(ctx.layout, Layout::Flat) => {
                    for (dir_id, dirsize) in dirsize_map.into_iter() {
                        let dir = arena[dir_id].get_mut();
                        *dir.size_mut() += dirsize;
                    }

                    let mut all_dirents = branches
                        .values()
                        .flatten()
                        .filter_map(|n| (*n != root_id).then_some(*n))
                        .collect::<Vec<_>>();

                    all_dirents.sort_by(|id_a, id_b| {
                        let node_a = arena[*id_a].get();
                        let node_b = arena[*id_b].get();
                        comparator(node_a, node_b)
                    });

                    all_dirents
                        .into_iter()
                        .for_each(|n| root_id.append(n, &mut arena));
                }
                _ => {
                    for (dir_id, dirsize) in dirsize_map.into_iter() {
                        let dir = arena[dir_id].get_mut();
                        *dir.size_mut() += dirsize;

                        if let Some(mut dirents) = branches.remove(dir.path()) {
                            dirents.sort_by(|id_a, id_b| {
                                let node_a = arena[*id_a].get();
                                let node_b = arena[*id_b].get();
                                comparator(node_a, node_b)
                            });

                            for dirent_id in dirents {
                                dir_id.append(dirent_id, &mut arena);
                            }
                        }
                    }
                }
            },
            None => {
                for (dir_id, dirsize) in dirsize_map.into_iter() {
                    let dir = arena[dir_id].get_mut();
                    *dir.size_mut() += dirsize;

                    if let Some(dirents) = branches.remove(dir.path()) {
                        for dirent_id in dirents {
                            dir_id.append(dirent_id, &mut arena);
                        }
                    }
                }
            }
        }

        column_metadata.update_size_width(arena[root_id].get(), ctx);

        let tree = Self { root_id, arena };

        Ok((tree, accumulator, column_metadata))
    }

    pub fn init_without_disk_usage(
        ctx: &Context,
    ) -> Result<(Self, super::Accumulator, column::Metadata)> {
        let (
            TransitionState {
                mut arena,
                mut branches,
                mut column_metadata,
                root_id,
            },
            accumulator,
        ) = Self::load(ctx)?;

        #[cfg(unix)]
        macro_rules! update_metadata {
            ($dirent_id:expr) => {
                let dirent = arena[$dirent_id].get();
                if let Ok(inode) = dirent.inode() {
                    column_metadata.update_inode_attr_widths(&inode);
                }
            };
        }

        match ctx.sort_type {
            SortType::Flat if matches!(ctx.layout, Layout::Flat) => {
                let mut all_dirents = branches
                    .values()
                    .flatten()
                    .filter_map(|n| (*n != root_id).then_some(*n))
                    .collect::<Vec<_>>();

                if let Some(comparator) = order::comparator(Sort::None, ctx.dir_order) {
                    all_dirents.sort_by(|id_a, id_b| {
                        let node_a = arena[*id_a].get();
                        let node_b = arena[*id_b].get();
                        comparator(node_a, node_b)
                    });
                }

                for dirent_id in all_dirents {
                    root_id.append(dirent_id, &mut arena);

                    #[cfg(unix)]
                    update_metadata!(dirent_id);
                }
            }
            _ => {
                let dirs = arena
                    .iter()
                    .filter_map(|node| {
                        if node.get().is_dir() {
                            arena
                                .get_node_id(node)
                                .map(|n| (node.get().path().to_path_buf(), n))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                dirs.into_iter().for_each(|(path, dir)| {
                    if let Some(mut dirents) = branches.remove(&path) {
                        if let Some(comparator) = order::comparator(Sort::None, ctx.dir_order) {
                            dirents.sort_by(|id_a, id_b| {
                                let node_a = arena[*id_a].get();
                                let node_b = arena[*id_b].get();
                                comparator(node_a, node_b)
                            });
                        }

                        for dirent_id in dirents {
                            dir.append(dirent_id, &mut arena);

                            #[cfg(unix)]
                            update_metadata!(dirent_id);
                        }
                    }
                });
            }
        }

        let tree = Self { root_id, arena };

        Ok((tree, accumulator, column_metadata))
    }

    /// Reads data from disk and aggregates data along with metadata into a [`TransitionState`]
    /// which callers would then consume to construct a [`Tree`].
    fn load(ctx: &Context) -> Result<(TransitionState, super::Accumulator)> {
        let mut arena = Arena::new();
        let mut branches = HashMap::<PathBuf, Vec<NodeId>>::default();
        let mut column_metadata = column::Metadata::default();
        let mut maybe_root_id = None;
        let mut accumulator = super::Accumulator::default();

        // To notify the progress indicator
        let notifier = progress::Indicator::use_notifier();

        traverse::run(ctx, |file| {
            #[cfg(unix)]
            column_metadata.update_unix_attrs_widths(&file, ctx);

            let node_id = arena.new_node(file);
            let file = arena[node_id].get();
            let file_path = file.path();

            accumulator.increment(file.file_type());
            progress::Indicator::notify(&notifier, accumulator.total());

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

        progress::Indicator::finish(&notifier);

        let root_id = maybe_root_id
            .ok_or(TreeError::RootDir)
            .into_report(ErrorCategory::Internal)
            .context(error_source!())?;

        let ts = TransitionState {
            arena,
            branches,
            column_metadata,
            root_id,
        };

        Ok((ts, accumulator))
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
