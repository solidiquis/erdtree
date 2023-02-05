use clap::ValueEnum;
use std::cmp::Ordering;
use super::node::Node;

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Order {
    /// Sort entries by file name
    Filename,
    
    /// Sort entries by size in descending order
    Size,

    /// No sorting
    None
}

impl Order {
    pub fn comparator(&self) -> Option<fn(a: &Node, b: &Node) -> Ordering> {
        match self {
            Self::Filename => Some(Self::name_comparator),
            Self::Size => Some(Self::size_comparator),
            _ => None
        }
    }

    pub fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    pub fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size.expect("Expected file size to be set");
        let b_size = b.file_size.expect("Expected file size to be set");

        b_size.cmp(&a_size)
    }
}
