#![allow(dead_code)]

mod error;
mod tree_node;

use std::path::Path;
use std::fs;
use std::io;
use tree_node::{TreeNode, FileType};

const BRANCH_SEP: &'static str = "  ";

pub type FileTreeResult<'a> = Result<FileTree<'a>, io::Error>;

pub struct FileTree<'a> {
    root_location: &'a Path,
    ignore_patterns: Option<Vec<&'a str>>,
    root_node: TreeNode,
    depth: Option<u64>
}

impl<'a> FileTree<'a> {
    pub fn new<S>(root_location: &'a S, ignore_patterns: Option<&'a str>, depth: Option<u64>) -> FileTreeResult<'a>
        where S: AsRef<Path> + ?Sized
    {
        let root_node_md = fs::metadata(root_location)?;

        if !root_node_md.is_dir() {
            return Err(error::not_dir_err())
        }

        let ignore_patterns = if let Some(patterns) = ignore_patterns {
            Some(patterns.split(",").into_iter().collect::<Vec<&'a str>>())
        } else {
            None
        };

        let root_node = TreeNode::new(
            root_location,
            FileType::Dir,
            ".".to_string(),
            &ignore_patterns,
            0
        );

        Ok(Self {
            root_location: root_location.as_ref(),
            ignore_patterns,
            root_node,
            depth
        })
    }

    pub fn get_root_node(&self) -> &TreeNode {
        &self.root_node
    }

    pub fn len(&self) -> u64 {
        self.root_node.len()
    }

    pub fn display(&self) {
        let root_node = self.get_root_node();
        let mut buffer = "".to_string();

        let max_depth = match self.depth {
            Some(depth) => depth,
            None => u64::MAX
        };
        
        Self::sprintf_row(&root_node, &mut buffer, "");
        Self::sprintf_branches(&root_node, &mut buffer, "", max_depth);

        println!("{}", buffer);
    }

    fn sprintf_branches(node: &TreeNode, buffer: &mut String, base_prefix: &str, depth: u64) {
        if node.get_generation() >= depth { return }

        let iter_childen = node.iter_children();
        
        let (last_child, children) = match iter_childen.as_slice().split_last() {
            Some((child, children)) => (Some(child), children.iter()),
            None => (None, iter_childen)
        };

        for child in children {
            let prefix = format!("{}\x1B[36m\u{251C}\u{2500}\x1B[0m{}", base_prefix, BRANCH_SEP);
            Self::sprintf_row(child, buffer, &prefix);

            if child.is_dir() {
                let base = format!("{}\x1B[36m\u{2502} \x1B[0m{}", base_prefix, BRANCH_SEP);
                Self::sprintf_branches(child, buffer, &base, depth);
            }
        }

        if let Some(child) = last_child {
            let prefix = format!("{}\x1B[36m\u{2514}\u{2500}\x1B[0m{}", base_prefix, BRANCH_SEP);
            Self::sprintf_row(child, buffer, &prefix);

            if child.is_dir() {
                let base = format!("{}  {}", base_prefix, BRANCH_SEP);
                Self::sprintf_branches(child, buffer, &base, depth);
            }
        }
    }

    fn sprintf_row(node: &TreeNode, buffer: &mut String, prefix: &str) {
        let fmt_row = format!("{}{} ({})\n", prefix, node.sprintf_file_name(), node.sprintf_len());

        buffer.push_str(&fmt_row);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_tree() {
        use super::FileTree;
        use super::tree_node::FileType;

        let file_tree = FileTree::new("./assets/", Some(".")).unwrap();
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

        let tree_len = file_tree.len();

        assert_eq!(tree_len, 13);
    }
}
