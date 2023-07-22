use crate::{
    context::{column, Context},
    disk_usage::file_size::FileSize,
    fs::inode::Inode,
    progress::{IndicatorHandle, Message},
    utils,
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
    path::PathBuf,
    result::Result as StdResult,
    sync::mpsc::{self, Sender},
    thread,
};
use visitor::{BranchVisitorBuilder, TraversalState};

/// Operations to handle and display aggregate file counts based on their type.
pub mod count;

/// Errors related to traversal, [Tree] construction, and the like.
pub mod error;

/// Contains components of the [`Tree`] data structure that derive from [`ignore::DirEntry`].
pub mod node;

/// Custom visitor that operates on each thread during filesystem traversal.
mod visitor;

/// Virtual data structure that represents local file-system hierarchy.
pub struct Tree {
    arena: Arena<Node>,
    root_id: NodeId,
}

pub type Result<T> = StdResult<T, Error>;

impl Tree {
    /// Constructor for [Tree].
    pub const fn new(arena: Arena<Node>, root_id: NodeId) -> Self {
        Self { arena, root_id }
    }

    /// Initiates file-system traversal and [Tree] as well as updates the [Context] object with
    /// various properties necessary to render output.
    pub fn try_init(
        mut ctx: Context,
        indicator: Option<&IndicatorHandle>,
    ) -> Result<(Self, Context)> {
        let mut column_properties = column::Properties::from(&ctx);

        let (arena, root_id) = Self::traverse(&ctx, &mut column_properties, indicator)?;

        ctx.update_column_properties(&column_properties);

        if ctx.truncate {
            ctx.set_window_width();
        }

        let tree = Self::new(arena, root_id);

        if tree.is_stump() {
            return Err(Error::NoMatches);
        }

        Ok((tree, ctx))
    }

    /// Returns `true` if there are no entries to show excluding the `root_id`.
    pub fn is_stump(&self) -> bool {
        self.root_id
            .descendants(self.arena())
            .skip(1)
            .peekable()
            .next()
            .is_none()
    }

    /// Grab a reference to `root_id`.
    pub const fn root_id(&self) -> NodeId {
        self.root_id
    }

    /// Grabs a reference to `arena`.
    pub const fn arena(&self) -> &Arena<Node> {
        &self.arena
    }

    /// Parallel traversal of the `root_id` directory and its contents. Parallel traversal relies on
    /// `WalkParallel`. Any filesystem I/O or related system calls are expected to occur during
    /// parallel traversal; post-processing post-processing of all directory entries should
    /// be completely CPU-bound.
    fn traverse(
        ctx: &Context,
        column_properties: &mut column::Properties,
        indicator: Option<&IndicatorHandle>,
    ) -> Result<(Arena<Node>, NodeId)> {
        let walker = WalkParallel::try_from(ctx)?;
        let (tx, rx) = mpsc::channel();

        let progress_indicator_mailbox = indicator.map(IndicatorHandle::mailbox);

        thread::scope(|s| {
            let res = s.spawn(move || {
                let mut tree = Arena::new();
                let mut branches: HashMap<PathBuf, Vec<NodeId>> = HashMap::new();
                let mut root_id = None;

                while let Ok(TraversalState::Ongoing(node)) = rx.recv() {
                    if let Some(ref mailbox) = progress_indicator_mailbox {
                        if mailbox.send(Message::Index).is_err() {
                            return Err(Error::Terminated);
                        }
                    }

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

                if let Some(ref mailbox) = progress_indicator_mailbox {
                    if mailbox.send(Message::DoneIndexing).is_err() {
                        return Err(Error::Terminated);
                    }
                }

                let root_id = root_id.ok_or(Error::MissingRoot)?;
                let node_comparator = node::cmp::comparator(ctx);
                let mut inodes = HashSet::new();

                Self::assemble_tree(
                    &mut tree,
                    root_id,
                    &mut branches,
                    &node_comparator,
                    &mut inodes,
                    column_properties,
                    ctx,
                );

                if ctx.prune || ctx.pattern.is_some() {
                    Self::prune_directories(root_id, &mut tree);
                }

                if ctx.dirs_only {
                    Self::filter_directories(root_id, &mut tree);
                }

                Ok((tree, root_id))
            });

            let mut visitor_builder = BranchVisitorBuilder::new(ctx, Sender::clone(&tx));

            walker.visit(&mut visitor_builder);

            let _ = tx.send(TraversalState::Done);

            res.join().unwrap()
        })
    }

    /// Takes the results of the parallel traversal and uses it to construct the [Tree] data
    /// structure. Sorting occurs if specified. The amount of columns needed to fit all of the disk
    /// usages is also computed here.
    fn assemble_tree(
        tree: &mut Arena<Node>,
        current_node_id: NodeId,
        branches: &mut HashMap<PathBuf, Vec<NodeId>>,
        node_comparator: &NodeComparator,
        inode_set: &mut HashSet<Inode>,
        column_properties: &mut column::Properties,
        ctx: &Context,
    ) {
        let current_node = tree[current_node_id].get_mut();

        let mut children = branches.remove(current_node.path()).unwrap();

        let mut dir_size = FileSize::from(ctx);

        for child_id in &children {
            let index = *child_id;

            let is_dir = {
                let arena = tree[index].get();
                arena.is_dir()
            };

            if is_dir {
                Self::assemble_tree(
                    tree,
                    index,
                    branches,
                    node_comparator,
                    inode_set,
                    column_properties,
                    ctx,
                );
            }

            let node = tree[index].get();

            #[cfg(unix)]
            Self::update_column_properties(column_properties, node, ctx);

            #[cfg(not(unix))]
            Self::update_column_properties(column_properties, node, ctx);

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

        if dir_size.value() > 0 {
            let dir = tree[current_node_id].get_mut();

            dir.set_file_size(dir_size);
        }

        let dir = tree[current_node_id].get();

        #[cfg(unix)]
        Self::update_column_properties(column_properties, dir, ctx);

        #[cfg(not(unix))]
        Self::update_column_properties(column_properties, dir, ctx);

        children.sort_by(|&id_a, &id_b| {
            let node_a = tree[id_a].get();
            let node_b = tree[id_b].get();
            node_comparator(node_a, node_b)
        });

        // Append children to current node.
        for child_id in children {
            current_node_id.append(child_id, tree);
        }
    }

    /// Function to remove empty directories.
    fn prune_directories(root_id: NodeId, tree: &mut Arena<Node>) {
        let to_prune = root_id
            .descendants(tree)
            .skip(1)
            .filter(|node_id| {
                tree[*node_id]
                    .get()
                    .is_dir()
                    .then(|| node_id.children(tree).count() == 0)
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        if to_prune.is_empty() {
            return;
        }

        to_prune
            .iter()
            .for_each(|node_id| node_id.remove_subtree(tree));

        Self::prune_directories(root_id, tree);
    }

    /// Filter `arena` for only directories.
    fn filter_directories(root_id: NodeId, tree: &mut Arena<Node>) {
        let to_detach = root_id
            .descendants(tree)
            .skip(1)
            .filter(|&descendant_id| !tree[descendant_id].get().is_dir())
            .collect::<Vec<_>>();

        if to_detach.is_empty() {
            return;
        }

        to_detach.iter().for_each(|node_id| node_id.detach(tree));
    }

    /// Compute total number of files for a single directory without recurring into child
    /// directories. Files are grouped into three categories: directories, regular files, and
    /// symlinks.
    pub fn compute_file_count(node_id: NodeId, tree: &Arena<Node>) -> FileCount {
        node_id
            .children(tree)
            .map(|child_id| tree[child_id].get())
            .fold(FileCount::default(), |acc, node| acc + node)
    }

    /// Updates [`column::Properties`] with provided [`Node`].
    #[cfg(unix)]
    fn update_column_properties(col_props: &mut column::Properties, node: &Node, ctx: &Context) {
        if let Some(file_size) = node.file_size() {
            if ctx.byte_metric() && ctx.human {
                let out = format!("{file_size}");
                let [size, unit]: [&str; 2] =
                    out.split(' ').collect::<Vec<&str>>().try_into().unwrap();

                let file_size_cols = size.len();
                let file_size_unit_cols = unit.len();

                if file_size_cols > col_props.max_size_width {
                    col_props.max_size_width = file_size_cols;
                }

                if file_size_unit_cols > col_props.max_size_unit_width {
                    col_props.max_size_unit_width = file_size_unit_cols;
                }
            } else {
                let file_size_cols = utils::num_integral(file_size.value());

                if file_size_cols > col_props.max_size_width {
                    col_props.max_size_width = file_size_cols;
                }
            };
        }

        if ctx.long {
            if let Some(owner) = node.owner() {
                let owner_len = owner.len();

                if owner_len > col_props.max_owner_width {
                    col_props.max_owner_width = owner_len;
                }
            }

            if let Some(group) = node.group() {
                let group_len = group.len();

                if group_len > col_props.max_group_width {
                    col_props.max_group_width = group_len;
                }
            }

            if let Some(ino) = node.ino() {
                let ino_num_integral = utils::num_integral(ino);

                if ino_num_integral > col_props.max_ino_width {
                    col_props.max_ino_width = ino_num_integral;
                }
            }

            if let Some(nlink) = node.nlink() {
                let nlink_num_integral = utils::num_integral(nlink);

                if nlink_num_integral > col_props.max_nlink_width {
                    col_props.max_nlink_width = nlink_num_integral;
                }
            }

            if let Some(blocks) = node.blocks() {
                let blocks_num_integral = utils::num_integral(blocks);

                if blocks_num_integral > col_props.max_block_width {
                    col_props.max_block_width = blocks_num_integral;
                }
            }
        }
    }

    /// Updates [column::Properties] with provided [Node].
    #[cfg(not(unix))]
    fn update_column_properties(col_props: &mut column::Properties, node: &Node, ctx: &Context) {
        if let Some(file_size) = node.file_size() {
            if ctx.byte_metric() && ctx.human {
                let out = format!("{file_size}");
                let [size, unit]: [&str; 2] =
                    out.split(' ').collect::<Vec<&str>>().try_into().unwrap();

                let file_size_cols = size.len();
                let file_size_unit_cols = unit.len();

                if file_size_cols > col_props.max_size_width {
                    col_props.max_size_width = file_size_cols;
                }

                if file_size_unit_cols > col_props.max_size_unit_width {
                    col_props.max_size_unit_width = file_size_unit_cols;
                }
            } else {
                let file_size_cols = utils::num_integral(file_size.value());

                if file_size_cols > col_props.max_size_width {
                    col_props.max_size_width = file_size_cols;
                }
            };
        }
    }
}

impl TryFrom<&Context> for WalkParallel {
    type Error = Error;

    fn try_from(ctx: &Context) -> StdResult<Self, Self::Error> {
        let root_id = fs::canonicalize(ctx.dir())?;

        fs::metadata(&root_id)
            .map_err(|e| Error::DirNotFound(format!("{}: {e}", root_id.display())))?;

        let mut builder = WalkBuilder::new(root_id);

        builder
            .follow_links(ctx.follow)
            .git_ignore(!ctx.no_ignore)
            .git_global(!ctx.no_ignore)
            .hidden(!ctx.hidden)
            .overrides(ctx.no_git_override()?)
            .same_file_system(ctx.same_fs)
            .threads(ctx.threads);

        if ctx.suppress_size && ctx.level() == 1 {
            builder.max_depth(Some(1)).threads(1);
        }

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
