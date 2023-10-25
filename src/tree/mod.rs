use crate::{error::prelude::*, file::File, user::Context};
use ignore::{Walk, WalkBuilder};
use indextree::{Arena, NodeId, Traverse};
use std::{collections::HashMap, convert::TryFrom, ops::Deref};

/// Representation of the file-tree that is traversed starting from the root directory whose index
/// in the underlying `arena` is `root_id`.
pub struct FileTree {
    root_id: NodeId,
    arena: Arena<File>,
}

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

    /// Initializes a [`FileTree`] completely on one thread.
    pub fn init(ctx: &Context) -> Result<Self> {
        let mut walker = Walk::try_from(ctx)?;

        let root_entry = walker
            .next()
            .ok_or(TreeError::RootDirMissing)
            .into_report(ErrorCategory::Internal)?;

        let root_node = root_entry
            .into_report(ErrorCategory::Internal)
            .and_then(|data| File::init(data, ctx))?;

        let mut arena = Arena::new();
        let root_node_id = arena.new_node(root_node);
        let mut current_dir_id = root_node_id;

        let mut dirsize_map = HashMap::new();
        let mut dir_stack = vec![];

        dirsize_map.insert(root_node_id, 0);

        for dent in walker {
            let node = match dent
                .into_report(ErrorCategory::Warning)
                .and_then(|data| File::init(data, ctx))
            {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{e}");
                    continue;
                },
            };

            let is_dir = node.file_type().is_some_and(|ft| ft.is_dir());
            let size = node.size().value();

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

        println!("{dirsize_map:?}");

        Ok(Self::new(root_node_id, arena))
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

/// Initializes a single-threaded [`Walk`] instance from [`Context`].
impl TryFrom<&Context> for Walk {
    type Error = Error;

    fn try_from(ctx: &Context) -> Result<Self> {
        let path = match ctx.dir() {
            Some(d) => d.to_path_buf(),
            None => Context::get_current_dir()?,
        };

        let walker = WalkBuilder::new(path)
            .follow_links(ctx.follow)
            .git_ignore(!ctx.no_ignore)
            .git_global(!ctx.no_ignore)
            .hidden(!ctx.hidden)
            .same_file_system(ctx.same_fs)
            .build();

        Ok(walker)
    }
}
