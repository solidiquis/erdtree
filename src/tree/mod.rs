use crate::{error::Result, file::File};

use indextree::{Arena, NodeId};

mod traversal;

pub struct FileTree {
    root_id: NodeId,
    arena: Arena<File>,
}

impl FileTree {
    pub fn new(root_id: NodeId, arena: Arena<File>) -> Self {
        Self { root_id, arena }
    }
}
