use super::{context::Context, tree::node::Node};
use clap::ValueEnum;
use std::{cmp::Ordering, convert::From};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortType {
    /// Sort entries by file name
    Name,

    /// Sort entries by size smallest to largest, top to bottom
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    SizeRev,
}

/// Order in which to print directories.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectoryOrdering {
    /// Order directories before files
    First,

    /// Order directories after files
    Last,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    sort: Option<SortType>,
    dir: Option<DirectoryOrdering>,
}

/// Comparator type used to sort [Node]s.
pub type NodeComparator = dyn Fn(&Node, &Node) -> Ordering;

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparators(&self) -> impl Iterator<Item = Box<NodeComparator>> {
        [
            self.sort.as_ref().map(SortType::comparator),
            self.dir.as_ref().map(DirectoryOrdering::comparator),
        ]
        .into_iter()
        .filter(|comparator| comparator.is_some())
        // UNWRAP: we just filtered Nones out
        .map(|comparator| comparator.unwrap())
    }
}

impl SortType {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Box<dyn Fn(&Node, &Node) -> Ordering> {
        let comparator = match self {
            Self::Name => Self::name_comparator,
            Self::Size => Self::size_comparator,
            Self::SizeRev => Self::size_rev_comparator,
        };

        Box::new(comparator)
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

impl DirectoryOrdering {
    /// Yields function pointer to the appropriate directory comparator.
    pub fn comparator(&self) -> Box<NodeComparator> {
        let comparator = match self {
            Self::First => Self::first_comparator,
            Self::Last => Self::last_comparator,
        };

        Box::new(comparator)
    }

    /// Comparator based on directory presedence.
    fn first_comparator(a: &Node, b: &Node) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    /// Comparator based on non-directory presedence.
    fn last_comparator(a: &Node, b: &Node) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (false, true) => Ordering::Less,
            (true, false) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl<'a> From<&'a Context> for Order {
    fn from(ctx: &'a Context) -> Self {
        Self {
            sort: ctx.sort(),
            dir: ctx.dir_ordering(),
        }
    }
}

impl From<(Option<SortType>, Option<DirectoryOrdering>)> for Order {
    fn from((sort, dir): (Option<SortType>, Option<DirectoryOrdering>)) -> Self {
        Self { sort, dir }
    }
}
