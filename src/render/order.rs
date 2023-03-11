use super::node::Node;
use clap::ValueEnum;
use serde::Deserialize;
use std::{cmp::Ordering, convert::From};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, Deserialize, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortType {
    /// Sort entries by file name
    Name,

    /// Sort entries by size smallest to largest, top to bottom
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    SizeRev,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    sort: SortType,
    dir_first: bool,
}

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<Box<dyn Fn(&Node, &Node) -> Ordering + '_>> {
        if self.dir_first {
            return Some(Box::new(|a, b| {
                Self::dir_comparator(a, b, self.sort.comparator())
            }));
        }

        self.sort.comparator()
    }

    fn dir_comparator(
        a: &Node,
        b: &Node,
        fallback: Option<impl Fn(&Node, &Node) -> Ordering>,
    ) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => fallback.map_or_else(|| Ordering::Equal, |sort| sort(a, b)),
        }
    }
}

impl SortType {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<Box<dyn Fn(&Node, &Node) -> Ordering>> {
        match self {
            Self::Name => Some(Box::new(Self::name_comparator)),
            Self::Size => Some(Box::new(Self::size_comparator)),
            Self::SizeRev => Some(Box::new(Self::size_rev_comparator)),
        }
    }

    /// Comparator based on `Node` file names.
    fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator that sorts [Node]s by size smallest to largest.
    fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size().map(|fs| fs.bytes).unwrap_or(0);
        let b_size = b.file_size().map(|fs| fs.bytes).unwrap_or(0);

        a_size.cmp(&b_size)
    }

    /// Comparator that sorts [Node]s by size largest to smallest.
    fn size_rev_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size().map(|fs| fs.bytes).unwrap_or(0);
        let b_size = b.file_size().map(|fs| fs.bytes).unwrap_or(0);
        b_size.cmp(&a_size)
    }
}

impl From<(SortType, bool)> for Order {
    fn from((sort, dir_first): (SortType, bool)) -> Self {
        Order { sort, dir_first }
    }
}
