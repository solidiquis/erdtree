use crossbeam::channel::{self, Sender};
use ignore::{WalkParallel, WalkState};
use std::{
    collections::HashMap,
    path::PathBuf,
    thread,
};
use super::{
    node::Node,
    super::error::Error,
};

pub struct Tree {
    pub root: Node
}

pub type TreeResult<T> = Result<T, Error>;
type Branches = HashMap::<PathBuf, Vec<Node>>;
type TreeComponents = (Node, Branches);

impl Tree {
    pub fn new(walker: WalkParallel) -> TreeResult<Self> {
        let root = Self::traverse(walker)?;

        Ok(Self { root })
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

    #[inline]
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
