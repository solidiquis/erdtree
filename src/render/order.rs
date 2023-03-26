use super::tree::node::Node;
use clap::ValueEnum;
use std::{cmp::Ordering, convert::From};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SortType {
    /// Sort entries by file name
    Name,

    /// Sort entries by size smallest to largest, top to bottom
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    SizeRev,

    /// Do not sort entries
    #[default]
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    sort: SortType,
    dir_first: bool,
}

/// Comparator type used to sort [Node]s.
pub type NodeComparator<'a> = dyn Fn(&Node, &Node) -> Ordering + 'a;

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<Box<NodeComparator<'_>>> {
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
    pub fn comparator(self) -> Option<Box<dyn Fn(&Node, &Node) -> Ordering>> {
        match self {
            Self::Name => Some(Box::new(Self::name_comparator)),
            Self::Size => Some(Box::new(Self::size_comparator)),
            Self::SizeRev => Some(Box::new(Self::size_rev_comparator)),
            Self::None => None,
        }
    }

    /// Comparator based on `Node` file names.
    fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator that sorts [Node]s by size smallest to largest.
    fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size().map_or(0, |fs| fs.bytes);
        let b_size = b.file_size().map_or(0, |fs| fs.bytes);

        a_size.cmp(&b_size)
    }

    /// Comparator that sorts [Node]s by size largest to smallest.
    fn size_rev_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size().map_or(0, |fs| fs.bytes);
        let b_size = b.file_size().map_or(0, |fs| fs.bytes);
        b_size.cmp(&a_size)
    }
}

impl From<(SortType, bool)> for Order {
    fn from((sort, dir_first): (SortType, bool)) -> Self {
        Self { sort, dir_first }
    }
}
