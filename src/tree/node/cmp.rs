use super::Node;
use crate::context::{dir, sort, Context};
use std::cmp::Ordering;

/// Comparator type used to sort [Node]s.
pub type NodeComparator = dyn Fn(&Node, &Node) -> Ordering;

/// Yields function pointer to the appropriate `Node` comparator.
pub fn comparator(ctx: &Context) -> Box<NodeComparator> {
    let sort_type = ctx.sort;

    match ctx.dir_order {
        dir::Order::None => (),
        dir::Order::First => {
            return Box::new(move |a, b| dir_first_comparator(a, b, base_comparator(sort_type)));
        }
        dir::Order::Last => {
            return Box::new(move |a, b| dir_last_comparator(a, b, base_comparator(sort_type)));
        }
    };

    base_comparator(sort_type)
}

/// Grabs the comparator for two non-dir type [Node]s.
fn base_comparator(sort_type: sort::Type) -> Box<NodeComparator> {
    match sort_type {
        sort::Type::Name => Box::new(name_comparator),
        sort::Type::Size => Box::new(size_comparator),
        sort::Type::SizeRev => Box::new(size_rev_comparator),
    }
}

/// Orders directories first. Provides a fallback if inputs are not directories.
fn dir_first_comparator(
    a: &Node,
    b: &Node,
    fallback: impl Fn(&Node, &Node) -> Ordering,
) -> Ordering {
    match (a.is_dir(), b.is_dir()) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => fallback(a, b),
    }
}

/// Orders directories last. Provides a fallback if inputs are not directories.
fn dir_last_comparator(
    a: &Node,
    b: &Node,
    fallback: impl Fn(&Node, &Node) -> Ordering,
) -> Ordering {
    match (a.is_dir(), b.is_dir()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => fallback(a, b),
    }
}

/// Comparator that sorts [Node]s by size, smallest to largest.
fn size_rev_comparator(a: &Node, b: &Node) -> Ordering {
    let a_size = a.file_size().map_or(0, |fs| fs.bytes);
    let b_size = b.file_size().map_or(0, |fs| fs.bytes);

    a_size.cmp(&b_size)
}

/// Comparator that sorts [Node]s by size, largest to smallest.
fn size_comparator(a: &Node, b: &Node) -> Ordering {
    let a_size = a.file_size().map_or(0, |fs| fs.bytes);
    let b_size = b.file_size().map_or(0, |fs| fs.bytes);
    b_size.cmp(&a_size)
}

/// Comparator based on [Node] file names.
fn name_comparator(a: &Node, b: &Node) -> Ordering {
    a.file_name().cmp(b.file_name())
}
