use crate::{
    fs::inode::Inode,
    render::{
        context::Context,
        disk_usage::file_size::FileSize,
        styles,
    },
};
use count::FileCount;
use error::Error;
use ignore::{WalkBuilder, WalkParallel};
use indextree::{Arena, NodeId};
use node::{cmp::NodeComparator, Node};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fs,
    marker::PhantomData,
    path::PathBuf,
    result::Result as StdResult,
    sync::mpsc::{self, Sender},
    thread,
};
use visitor::{BranchVisitorBuilder, TraversalState};

/// Operations to handle and display aggregate file counts based on their type.
mod count;

/// Display variants for [Tree].
pub mod display;

/// Errors related to traversal, [Tree] construction, and the like.
pub mod error;

/// Contains components of the [`Tree`] data structure that derive from [`DirEntry`].
///
/// [`Tree`]: Tree
/// [`DirEntry`]: ignore::DirEntry
pub mod node;

/// Custom visitor that operates on each thread during filesystem traversal.
mod visitor;

/// Virtual data structure that represents local file-system hierarchy.
pub struct Tree<T>
where
    T: display::TreeVariant,
{
    inner: Arena<Node>,
    root: NodeId,
    ctx: Context,
    display_variant: PhantomData<T>,
}

pub type Result<T> = StdResult<T, Error>;

impl<T> Tree<T>
where
    T: display::TreeVariant,
{
    /// Constructor for [Tree].
    pub const fn new(inner: Arena<Node>, root: NodeId, ctx: Context) -> Self {
        Self {
            inner,
            root,
            ctx,
            display_variant: PhantomData,
        }
    }

    /// Initiates file-system traversal and [Tree construction].
    pub fn try_init(mut ctx: Context) -> Result<Self> {
        let (inner, root) = Self::traverse(&ctx)?;

        let max_du_width = Self::compute_max_du_column_width(root, &inner);
        ctx.set_max_du_width(max_du_width);

        #[cfg(unix)]
        if ctx.long {
            let (max_nlink_width, max_ino_width, max_block_width) =
                Self::compute_max_column_widths(root, &inner, &ctx);

            ctx.set_max_nlink_width(max_nlink_width);
            ctx.set_max_ino_width(max_ino_width);
            ctx.set_max_block_width(max_block_width);
        }

        let tree = Self::new(inner, root, ctx);

        if tree.is_stump() {
            return Err(Error::NoMatches);
        }

        Ok(tree)
    }

    /// Returns `true` if there are no entries to show excluding the root.
    pub fn is_stump(&self) -> bool {
        self.root
            .descendants(self.inner())
            .skip(1)
            .peekable()
            .next()
            .is_none()
    }

    /// Grab a reference to [Context].
    pub const fn context(&self) -> &Context {
        &self.ctx
    }

    /// Grab a reference to `root`.
    const fn root(&self) -> NodeId {
        self.root
    }

    /// Grabs a reference to `inner`.
    const fn inner(&self) -> &Arena<Node> {
        &self.inner
    }

    /// Parallel traversal of the root directory and its contents. Parallel traversal relies on
    /// `WalkParallel`. Any filesystem I/O or related system calls are expected to occur during
    /// parallel traversal; post-processing post-processing of all directory entries should
    /// be completely CPU-bound.
    fn traverse(ctx: &Context) -> Result<(Arena<Node>, NodeId)> {
        let walker = WalkParallel::try_from(ctx)?;
        let (tx, rx) = mpsc::channel();

        thread::scope(|s| {
            let res = s.spawn(move || {
                let mut tree = Arena::new();
                let mut branches: HashMap<PathBuf, Vec<NodeId>> = HashMap::new();
                let mut root_id = None;

                while let Ok(TraversalState::Ongoing(node)) = rx.recv() {
                    if node.is_dir() {
                        let node_path = node.path();

                        if !branches.contains_key(node_path) {
                            branches.insert(node_path.to_owned(), vec![]);
                        }

                        if node.depth() == 0 {
                            root_id = Some(tree.new_node(node));
                            continue;
                        }
                    }

                    let parent = node.parent_path().ok_or(Error::ExpectedParent)?.to_owned();

                    let node_id = tree.new_node(node);

                    if branches
                        .get_mut(&parent)
                        .map(|mut_ref| mut_ref.push(node_id))
                        .is_none()
                    {
                        branches.insert(parent, vec![]);
                    }
                }

                let root = root_id.ok_or(Error::MissingRoot)?;
                let node_comparator = node::cmp::comparator(ctx);
                let mut inodes = HashSet::new();

                Self::assemble_tree(
                    &mut tree,
                    root,
                    &mut branches,
                    &node_comparator,
                    &mut inodes,
                    ctx,
                );

                if ctx.prune {
                    Self::prune_directories(root, &mut tree);
                }

                if ctx.dirs_only {
                    Self::filter_directories(root, &mut tree);
                }

                Ok::<(Arena<Node>, NodeId), Error>((tree, root))
            });

            let mut visitor_builder = BranchVisitorBuilder::new(ctx, Sender::clone(&tx));

            walker.visit(&mut visitor_builder);

            tx.send(TraversalState::Done).unwrap();

            res.join().unwrap()
        })
    }

    /// Takes the results of the parallel traversal and uses it to construct the [Tree] data
    /// structure. Sorting occurs if specified.
    fn assemble_tree(
        tree: &mut Arena<Node>,
        current_node_id: NodeId,
        branches: &mut HashMap<PathBuf, Vec<NodeId>>,
        node_comparator: &NodeComparator,
        inode_set: &mut HashSet<Inode>,
        ctx: &Context,
    ) {
        let current_node = tree[current_node_id].get_mut();

        let mut children = branches.remove(current_node.path()).unwrap();

        let mut dir_size = FileSize::new(0, ctx.disk_usage, ctx.unit, ctx.scale);

        for child_id in &children {
            let index = *child_id;

            let is_dir = {
                let inner = tree[index].get();
                inner.is_dir()
            };

            if is_dir {
                Self::assemble_tree(tree, index, branches, node_comparator, inode_set, ctx);
            }

            let node = tree[index].get();

            // If a hard-link is already accounted for then don't increment parent dir size.
            if let Some(inode) = node.inode() {
                if inode.nlink > 1 && !inode_set.insert(inode) {
                    continue;
                }
            }

            if let Some(file_size) = node.file_size() {
                dir_size += file_size;
            }
        }

        if dir_size.bytes > 0 {
            tree[current_node_id].get_mut().set_file_size(dir_size);
        }

        children.sort_by(|id_a, id_b| {
            let node_a = tree[*id_a].get();
            let node_b = tree[*id_b].get();
            node_comparator(node_a, node_b)
        });

        // Append children to current node.
        for child_id in children {
            current_node_id.append(child_id, tree);
        }
    }

    /// Function to remove empty directories.
    fn prune_directories(root_id: NodeId, tree: &mut Arena<Node>) {
        let mut to_prune = vec![];

        for node_id in root_id.descendants(tree).skip(1) {
            let node = tree[node_id].get();

            if !node.is_dir() {
                continue;
            }

            if node_id.children(tree).count() == 0 {
                to_prune.push(node_id);
            }
        }

        if to_prune.is_empty() {
            return;
        }

        for node_id in to_prune {
            node_id.remove_subtree(tree);
        }

        Self::prune_directories(root_id, tree);
    }

    /// Filter for only directories.
    fn filter_directories(root: NodeId, tree: &mut Arena<Node>) {
        let mut to_detach = vec![];

        for descendant_id in root.descendants(tree).skip(1) {
            if !tree[descendant_id].get().is_dir() {
                to_detach.push(descendant_id);
            }
        }

        for descendant_id in to_detach {
            descendant_id.detach(tree);
        }
    }

    /// Compute total number of files for a single directory without recurring into child
    /// directories. Files are grouped into three categories: directories, regular files, and
    /// symlinks.
    fn compute_file_count(node_id: NodeId, tree: &Arena<Node>) -> FileCount {
        let mut count = FileCount::default();

        for child_id in node_id.children(tree) {
            count.update(tree[child_id].get());
        }

        count
    }

    fn compute_max_du_column_width(root: NodeId, tree: &Arena<Node>) -> usize {
        tree[root]
            .get()
            .file_size()
            .map(|size| size.bytes)
            .map_or(0, crate::utils::num_integral)
    }

    #[cfg(unix)]
    fn compute_max_column_widths(
        root: NodeId,
        tree: &Arena<Node>,
        ctx: &Context,
    ) -> (usize, usize, usize) {
        let mut max_nlink = 0;
        let mut max_ino = 0;
        let mut max_blocks = 0;

        let max_depth = ctx.level();

        for id in root.descendants(tree) {
            let node = tree[id].get();

            if node.depth() > max_depth {
                continue;
            }

            if let Some(ino) = node.ino() {
                if ino > max_ino {
                    max_ino = ino;
                }
            }

            if let Some(nlink) = node.nlink() {
                if nlink > max_nlink {
                    max_nlink = nlink;
                }
            }

            if let Some(blocks) = node.blocks() {
                if blocks > max_blocks {
                    max_blocks = blocks;
                }
            }
        }

        let max_nlink_width = crate::utils::num_integral(max_nlink);
        let max_ino_width = crate::utils::num_integral(max_ino);
        let max_block_width = crate::utils::num_integral(max_blocks);

        (max_nlink_width, max_ino_width, max_block_width)
    }
}

impl TryFrom<&Context> for WalkParallel {
    type Error = Error;

    fn try_from(ctx: &Context) -> StdResult<Self, Self::Error> {
        let root = fs::canonicalize(ctx.dir())?;

        fs::metadata(&root).map_err(|e| Error::DirNotFound(format!("{}: {e}", root.display())))?;

        let mut builder = WalkBuilder::new(root);

        builder
            .follow_links(ctx.follow)
            .git_ignore(!ctx.no_ignore)
            .hidden(!ctx.hidden)
            .threads(ctx.threads);

        if ctx.pattern.is_some() {
            if ctx.glob || ctx.iglob {
                builder.filter_entry(ctx.glob_predicate()?);
            } else {
                builder.filter_entry(ctx.regex_predicate()?);
            }
        }

        Ok(builder.build_parallel())
    }
}
