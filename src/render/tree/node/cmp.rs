use super::Node;
use crate::render::context::{sort::SortType, Context};
use std::cmp::Ordering;

/// Comparator type used to sort [Node]s.
pub type NodeComparator = dyn Fn(&Node, &Node) -> Ordering;

/// Yields function pointer to the appropriate `Node` comparator.
pub fn comparator(ctx: &Context) -> Option<Box<NodeComparator>> {
    let sort_type = ctx.sort;

    if ctx.dirs_first {
        return Some(Box::new(move |a, b| {
            dir_comparator(a, b, base_comparator(sort_type))
        }));
    }

    base_comparator(sort_type)
}

/// Grabs the comparator for two non-dir type [Node]s.
fn base_comparator(sort_type: SortType) -> Option<Box<NodeComparator>> {
    match sort_type {
        SortType::Name => Some(Box::new(name_comparator)),
        SortType::Size => Some(Box::new(size_comparator)),
        SortType::SizeRev => Some(Box::new(size_rev_comparator)),
        SortType::None => None,
    }
}

/// Orders directories first. Provides a fallback if inputs are not directories.
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

/// Comparator that sorts [Node]s by size, smallest to largest.
fn size_comparator(a: &Node, b: &Node) -> Ordering {
    let a_size = a.file_size().map_or(0, |fs| fs.bytes);
    let b_size = b.file_size().map_or(0, |fs| fs.bytes);

    a_size.cmp(&b_size)
}

/// Comparator that sorts [Node]s by size, largest to smallest.
fn size_rev_comparator(a: &Node, b: &Node) -> Ordering {
    let a_size = a.file_size().map_or(0, |fs| fs.bytes);
    let b_size = b.file_size().map_or(0, |fs| fs.bytes);
    b_size.cmp(&a_size)
}

/// Comparator based on [Node] file names.
fn name_comparator(a: &Node, b: &Node) -> Ordering {
    a.file_name().cmp(b.file_name())
}
