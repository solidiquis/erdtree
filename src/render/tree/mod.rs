use crate::render::{
    context::Context,
    disk_usage::FileSize,
    order::{Order, SortType},
};
use crossbeam::channel::{self, Sender};
use error::Error;
use ignore::{WalkBuilder, WalkParallel, WalkState};
use node::Node;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
    slice::Iter,
    thread,
};

use super::order::DirectoryOrdering;

/// Errors related to traversal, [Tree] construction, and the like.
pub mod error;

/// Contains components of the [`Tree`] data structure that derive from [`DirEntry`].
///
/// [`Tree`]: Tree
/// [`DirEntry`]: ignore::DirEntry
pub mod node;

/// [ui::LS_COLORS] initialization and ui theme for [Tree].
pub mod ui;

/// In-memory representation of the root-directory and its contents which respects `.gitignore` and
/// hidden file rules depending on [WalkParallel] config.
#[derive(Debug)]
pub struct Tree {
    root: Node,
    ctx: Context,
}

pub type TreeResult<T> = Result<T, Error>;
pub type Branches = HashMap<PathBuf, Vec<Node>>;
pub type TreeComponents = (Node, Branches);

impl Tree {
    /// Constructor for [Tree].
    pub fn new(root: Node, ctx: Context) -> Self {
        Self { root, ctx }
    }

    /// Initiates file-system traversal and [Tree construction].
    pub fn init(ctx: Context) -> TreeResult<Self> {
        let root = Self::traverse(&ctx)?;

        Ok(Self::new(root, ctx))
    }

    /// Returns a reference to the root [Node].
    fn root(&self) -> &Node {
        &self.root
    }

    /// Maximum depth to display
    fn level(&self) -> usize {
        self.ctx.level.unwrap_or(usize::MAX)
    }

    fn context(&self) -> &Context {
        &self.ctx
    }

    /// Parallel traversal of the root directory and its contents taking `.gitignore` into
    /// consideration. Parallel traversal relies on `WalkParallel`. Any filesystem I/O or related
    /// system calls are expected to occur during parallel traversal; thus post-processing of all
    /// directory entries should be completely CPU-bound. If filesystem I/O or system calls occur
    /// outside of the parallel traversal step please report an issue.
    fn traverse(ctx: &Context) -> TreeResult<Node> {
        let walker = WalkParallel::try_from(ctx)?;
        let (tx, rx) = channel::unbounded::<Node>();

        // Receives directory entries from the workers used for parallel traversal to construct the
        // components needed to assemble a `Tree`.
        let tree_components = thread::spawn(move || -> TreeResult<TreeComponents> {
            let mut branches: Branches = HashMap::new();
            let mut inodes = HashSet::new();
            let mut root = None;

            while let Ok(node) = rx.recv() {
                if node.is_dir() {
                    let node_path = node.path();

                    if !branches.contains_key(node_path) {
                        branches.insert(node_path.to_owned(), vec![]);
                    }

                    if node.depth == 0 {
                        root = Some(node);
                        continue;
                    }
                }

                if let Some(inode) = node.inode() {
                    if inode.nlink > 1 {
                        // If a hard-link is already accounted for skip the subsequent one.
                        if !inodes.insert(inode.properties()) {
                            continue;
                        }
                    }
                }

                let parent = node.parent_path_buf().ok_or(Error::ExpectedParent)?;

                let update = branches.get_mut(&parent).map(|mut_ref| mut_ref.push(node));

                if update.is_none() {
                    branches.insert(parent, vec![]);
                }
            }

            let root_node = root.ok_or(Error::MissingRoot)?;

            Ok((root_node, branches))
        });

        // All filesystem I/O and related system-calls should be relegated to this. Directory
        // entries that are encountered are sent to the above thread for processing.
        walker.run(|| {
            Box::new(|entry_res| {
                let tx = Sender::clone(&tx);

                entry_res
                    .map(|entry| Node::from((&entry, ctx)))
                    .map(|node| tx.send(node).unwrap())
                    .map(|_| WalkState::Continue)
                    .unwrap_or(WalkState::Skip)
            })
        });

        drop(tx);

        let (mut root, mut branches) = tree_components.join().unwrap()?;

        Self::assemble_tree(&mut root, &mut branches, ctx);

        if ctx.prune {
            root.prune_directories()
        }

        Ok(root)
    }

    /// Takes the results of the parallel traversal and uses it to construct the [Tree] data
    /// structure. Sorting occurs if specified.
    fn assemble_tree(current_node: &mut Node, branches: &mut Branches, ctx: &Context) {
        let children = branches.remove(current_node.path()).unwrap();

        if children.len() > 0 {
            current_node.set_children(children);
        }

        let mut dir_size = FileSize::new(0, ctx.disk_usage, ctx.prefix, ctx.scale);

        current_node.children_mut().for_each(|node| {
            if node.is_dir() {
                Self::assemble_tree(node, branches, ctx);
            }

            if let Some(fs) = node.file_size() {
                dir_size += fs
            }
        });

        if dir_size.bytes > 0 {
            current_node.set_file_size(dir_size)
        }

        let apply_comparator = |comparator| current_node.sort_children(comparator);
        Order::from((ctx.sort(), ctx.dir_ordering()))
            .comparators()
            .for_each(apply_comparator);
    }
}

impl TryFrom<&Context> for WalkParallel {
    type Error = Error;

    fn try_from(clargs: &Context) -> Result<Self, Self::Error> {
        let root = fs::canonicalize(clargs.dir())?;

        fs::metadata(&root).map_err(|e| Error::DirNotFound(format!("{}: {e}", root.display())))?;

        Ok(WalkBuilder::new(root)
            .follow_links(clargs.follow_links)
            .git_ignore(!clargs.ignore_git_ignore)
            .hidden(!clargs.hidden)
            .threads(clargs.threads)
            .overrides(clargs.overrides()?)
            .build_parallel())
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let root = self.root();
        let level = self.level();
        let theme = ui::get_theme();
        let prune = self.context().prune;
        let mut output = String::from("");

        #[inline]
        fn extend_output(output: &mut String, node: &Node, prefix: &str) {
            output.push_str(&format!("{prefix}{node}\n"));
        }

        #[inline]
        fn traverse(
            output: &mut String,
            children: Iter<Node>,
            base_prefix: &str,
            level: usize,
            theme: &ui::ThemesMap,
            prune: bool,
        ) {
            let mut peekable = children.peekable();

            loop {
                if let Some(child) = peekable.next() {
                    let last_entry = peekable.peek().is_none();

                    let mut prefix = base_prefix.to_owned();

                    if last_entry {
                        prefix.push_str(theme.get("uprt").unwrap());
                    } else {
                        prefix.push_str(theme.get("vtrt").unwrap());
                    }

                    extend_output(output, child, &prefix);

                    if !child.is_dir() || child.depth + 1 > level {
                        continue;
                    }

                    if child.has_children() {
                        let children = child.children();

                        let mut new_base = base_prefix.to_owned();

                        let new_theme = child
                            .is_symlink()
                            .then(|| ui::get_link_theme())
                            .unwrap_or(theme);

                        if last_entry {
                            new_base.push_str(ui::SEP);
                        } else {
                            new_base.push_str(theme.get("vt").unwrap());
                        }

                        traverse(output, children, &new_base, level, new_theme, prune);
                    }

                    continue;
                }
                break;
            }
        }

        extend_output(&mut output, root, "");

        if root.has_children() {
            traverse(&mut output, root.children(), "", level, theme, prune)
        }

        write!(f, "{output}")
    }
}
