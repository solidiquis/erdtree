use crossbeam::channel::{self, Sender};
use ignore::{WalkParallel, WalkState};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    path::PathBuf,
    slice::Iter,
    thread,
};
use super::{
    node::Node,
    super::error::Error
};

/// Used for padding between tree branches.
pub const SEP: &'static str = "   ";

/// The `│` box drawing character.
pub const VT: &'static str = "\u{2502}  ";

/// The `└─` box drawing characters.
pub const UPRT: &'static str = "\u{2514}\u{2500} ";

/// The `├─` box drawing characters.
pub const VTRT: &'static str = "\u{251C}\u{2500} ";


pub struct Tree {
    root: Node,
}

pub type TreeResult<T> = Result<T, Error>;
pub type Branches = HashMap::<PathBuf, Vec<Node>>;
pub type TreeComponents = (Node, Branches);

impl Tree {
    pub fn new(walker: WalkParallel) -> TreeResult<Self> {
        let root = Self::traverse(walker)?;

        Ok(Self { root })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    fn traverse(walker: WalkParallel) -> TreeResult<Node> {
        let (tx, rx) = channel::unbounded::<Node>();

        let tree_components = thread::spawn(move || -> TreeResult<TreeComponents> {
            let mut branches: Branches = HashMap::new();
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

                let parent = node
                    .parent_path_buf()
                    .ok_or(Error::ExpectedParent)?;

                let update = branches
                    .get_mut(&parent)
                    .map(|mut_ref| mut_ref.push(node));

                if let None = update {
                    branches.insert(parent, vec![]);
                }
            }

            let root_node = root.ok_or(Error::MissingRoot)?;

            Ok((root_node, branches))
        });

        walker.run(|| Box::new(|entry_res| {
            let tx = Sender::clone(&tx);

            entry_res
                .map(|entry| Node::from(entry)) 
                .map(|node| tx.send(node).unwrap())
                .map(|_| WalkState::Continue)
                .unwrap_or(WalkState::Skip)
        }));

        drop(tx);

        let (mut root, mut branches) = tree_components.join().unwrap()?;

        Self::assemble_tree(&mut root, &mut branches);

        Ok(root)
    }

    fn assemble_tree(current_dir: &mut Node, branches: &mut Branches) {
        let dir_node = branches.remove(current_dir.path())
            .and_then(|children| {
                current_dir.set_children(children);
                Some(current_dir)
            });

        if let Some(node) = dir_node {
            let mut dir_size = 0;

            node.children_mut()
                .map(|nodes| nodes.iter_mut())
                .map(|node_iter| {
                    node_iter.for_each(|node| {
                        if node.is_dir() {
                            Self::assemble_tree(node, branches);
                            dir_size += node.file_size.unwrap_or(0);
                        } else {
                           dir_size += node.file_size.expect("Non-dir filetypes should have sizes");
                        }
                    });
                });

            if dir_size > 0 { node.set_file_size(dir_size) }
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let root = self.root();
        let mut output = String::from("");

        fn extend_output(output: &mut String, node: &Node, prefix: &str) {
            output.push_str(format!("{}{}\n", prefix, node).as_str());
        }

        fn traverse(output: &mut String, children: Iter<Node>, base_prefix: &str) {
            let mut peekable = children.peekable();

            loop {
                if let Some(child) = peekable.next() {
                    let last_entry =  peekable.peek().is_none();

                    let mut prefix = base_prefix.to_owned();

                    if last_entry {
                        prefix.push_str(UPRT);
                    } else {
                        prefix.push_str(VTRT);
                    }
                    
                    extend_output(output, child, prefix.as_str());

                    let mut new_base = base_prefix.to_owned();

                    if child.is_dir() && last_entry {
                        new_base.push_str(SEP);
                    } else {
                        new_base.push_str(VT);
                    }

                    child
                        .children()
                        .map(|iter_children| traverse(output, iter_children, new_base.as_str()));

                    continue;
                }
                break;
            }
        }

        extend_output(&mut output, root, "");
        root
            .children()
            .map(|iter_children| traverse(&mut output, iter_children, ""));

        write!(f, "{output}")
    }
}
