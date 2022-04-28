#![allow(dead_code)]

mod error;
mod tree_node;

use std::path::Path;
use std::fs;
use std::io;
use tree_node::{TreeNode, FileType};

pub type FileTreeResult<'a> = Result<FileTree<'a>, io::Error>;

pub struct FileTree<'a> {
    root_location: &'a Path,
    root_node: TreeNode
}

impl<'a> FileTree<'a> {
    pub fn new<S>(root_location: &'a S) -> FileTreeResult<'a>
        where S: AsRef<Path> + ?Sized
    {
        let root_md = fs::metadata(root_location)?;

        if !root_md.is_dir() {
            return Err(error::not_dir_err())
        }

        let root_node = TreeNode::new(
            root_location,
            FileType::Dir,
            ".".to_string(),
            0
        );

        Ok(Self {
            root_location: root_location.as_ref(),
            root_node
        })
    }

    pub fn get_root_node(&self) -> &TreeNode {
        &self.root_node
    }

    pub fn len(&self) -> u64 {
        self.root_node.get_metadata().len()
    }

    pub fn display(&self) {
        let root_node = self.get_root_node();
        let buffer = "".to_string();
        let result = Self::sprintf_output(&root_node, buffer);

        println!("{}", result);
    }

    fn sprintf_output(node: &TreeNode, mut buffer: String) -> String {
        buffer.push_str(&Self::sprintf_row(&node));
        buffer.push_str("\n");

        for child in node.iter_children() {
            if child.is_dir() {
                buffer = Self::sprintf_output(child, buffer);
            } else {
                buffer.push_str(&Self::sprintf_row(&child));
                buffer.push_str("\n");
            }
        }

        buffer
    }

    fn sprintf_row(node: &TreeNode) -> String {
        let mut prefix = "".to_string();

        for _ in 0..node.get_generation() {
            prefix.push_str("\t")
        }

        format!("{}{} {}", prefix, node.get_file_name(), node.len())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_tree() {
        use super::FileTree;
        use super::tree_node::FileType;

        let file_tree = FileTree::new("./assets/").unwrap();
        let root_node = file_tree.get_root_node();

        assert_eq!(root_node.get_generation(), 0);
        assert_eq!(root_node.num_children(), 3);

        let first_gen_child = root_node.iter_children().nth(0).unwrap();
        
        assert_eq!(first_gen_child.get_generation(), 1);

        let second_gen_child = root_node
            .iter_children() 
            .find(|child| child.get_file_type() == &FileType::Dir)
            .unwrap()
            .iter_children()
            .nth(0)
            .unwrap();

        assert_eq!(second_gen_child.get_generation(), 2);
    }
}
