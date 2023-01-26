use ignore::{WalkParallel, WalkState};
use std::{
    rc::Rc,
    sync::{Arc, Mutex}
};
use super::{
    node::{LocalNode, SharedNode},
    super::error::Error,
};

pub struct Tree {
    root: LocalNode
}

pub type TreeResult<T> = Result<T, Error>;

impl Tree {
    pub fn new(walker: WalkParallel) -> TreeResult<Self> {
        let nodes = Self::traverse(walker)?;

        let root = Self::assemble_tree(nodes)?;

        Ok(Self { root })
    }
    
    fn assemble_tree(nodes: Vec<LocalNode>) -> TreeResult<LocalNode> {
        let mut iter_nodes = nodes.into_iter();

        let root_node = iter_nodes.next()
            .ok_or(Error::TraversalError)
            .map(|node| Rc::new(node))?;

        let mut working_dir = unsafe {
            let ptr = Rc::into_raw(root_node.clone());
            Rc::decrement_strong_count(ptr);
            Rc::from_raw(ptr)
        };

        let mut prev_depth = 0;

        for node in iter_nodes {
            let depth_diff = (node.depth as i32) - prev_depth;
            let current_node = Rc::new(node);

            if depth_diff != 0 {
                if depth_diff > 0 {
                    prev_depth += 1;
                    Rc::get_mut(&mut working_dir)
                        .unwrap()
                        .push_child(current_node.clone());
                } else {
                    prev_depth -= 1;
                }

                working_dir = current_node;
            } else {
                Rc::get_mut(&mut working_dir)
                    .unwrap()
                    .push_child(current_node.clone());
            }
        }

        Rc::try_unwrap(root_node)
            .map_err(|_e| Error::TraversalError)
    }

    /// Parallel directory traversal. All read system calls related to filesystem I/O starts and
    /// ends here. `SharedNode`s are contructed during parallel traversal but is converted to
    /// `LocalNode`s before returning.
    fn traverse(walker: WalkParallel) -> TreeResult<Vec<LocalNode>> {
        let nodes = Arc::new(Mutex::new(vec![]));

        walker.run(|| Box::new(|entry_res| {
            let nodes = Arc::clone(&nodes);

            match entry_res.map(|entry| SharedNode::new(entry)) {
                Ok(node) => {
                    nodes.lock()
                        .map_err(|e| Error::from(e))
                        .unwrap()
                        .push(node);

                    WalkState::Continue
                },
                Err(_e) => WalkState::Skip
            }
        }));

        Arc::try_unwrap(nodes)
            .map_err(|e| Error::from(e))?
            .into_inner()
            .map_err(|e| Error::from(e))
            .map(|inner| {
                inner.into_iter()
                    .map(|node| LocalNode::from(node))
                    .collect::<Vec<LocalNode>>()
            })
    }
}

