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

/// Grabs the comparator for two non-dir type [Node]s.
fn base_comparator(sort_type: sort::Type) -> Box<NodeComparator> {
    Box::new(match sort_type {
        sort::Type::Name => naming::comparator,
        sort::Type::NameRev => naming::rev_comparator,

        sort::Type::Size => sizing::comparator,
        sort::Type::SizeRev => sizing::rev_comparator,

        _ => unreachable!("Not yet supported.")

        //sort::Type::Access => time_stamping::accessed::comparator,
        //sort::Type::AccessRev => time_stamping::accessed::rev_comparator,

        //sort::Type::Creation => time_stamping::created::comparator,
        //sort::Type::CreationRev => time_stamping::created::rev_comparator,

        //sort::Type::Modification => time_stamping::modified::comparator,
        //sort::Type::ModificationRev => time_stamping::modified::rev_comparator,
    })
}

//mod time_stamping {
//pub mod accessed {
//use crate::render::tree::node::Node;
//use core::cmp::Ordering;
//use std::time::SystemTime;

///// Comparator that sorts [Node]s by Last Access timestamp, newer to older.
//pub fn comparator(a: &Node, b: &Node) -> Ordering {
//let a_stamp = a.accessed().unwrap_or_else(SystemTime::now);
//let b_stamp = b.accessed().unwrap_or_else(SystemTime::now);
//a_stamp.cmp(&b_stamp)
//}

///// Comparator that sorts [Node]s by Access timestamp, older to newer.
//pub fn rev_comparator(a: &Node, b: &Node) -> Ordering {
//comparator(b, a)
//}
//}

//pub mod created {
//use crate::render::tree::node::Node;
//use core::cmp::Ordering;
//use std::time::SystemTime;

///// Comparator that sorts [Node]s by Creation timestamp, newer to older.
//pub fn comparator(a: &Node, b: &Node) -> Ordering {
//let a_stamp = a.created().unwrap_or_else(SystemTime::now);
//let b_stamp = b.created().unwrap_or_else(SystemTime::now);
//a_stamp.cmp(&b_stamp)
//}

///// Comparator that sorts [Node]s by Creation timestamp, older to newer.
//pub fn rev_comparator(a: &Node, b: &Node) -> Ordering {
//comparator(b, a)
//}
//}

//pub mod modified {
//use crate::render::tree::node::Node;
//use core::cmp::Ordering;
//use std::time::SystemTime;

///// Comparator that sorts [Node]s by Alteration timestamp, newer to older.
//pub fn comparator(a: &Node, b: &Node) -> Ordering {
//let a_stamp = a.modified().unwrap_or_else(SystemTime::now);
//let b_stamp = b.modified().unwrap_or_else(SystemTime::now);
//a_stamp.cmp(&b_stamp)
//}

///// Comparator that sorts [Node]s by Alteration timestamp, older to newer.
//pub fn rev_comparator(a: &Node, b: &Node) -> Ordering {
//comparator(b, a)
//}
//}
//}

mod sizing {
    use crate::disk_usage::file_size::FileSize;
    use crate::tree::node::Node;
    use core::cmp::Ordering;

    /// Comparator that sorts [Node]s by size, largest to smallest.
    pub fn comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size().map_or(0, FileSize::value);
        let b_size = b.file_size().map_or(0, FileSize::value);
        b_size.cmp(&a_size)
    }
    /// Comparator that sorts [Node]s by size, smallest to largest.
    pub fn rev_comparator(a: &Node, b: &Node) -> Ordering {
        comparator(b, a)
    }
}

mod naming {
    use crate::tree::node::Node;
    use core::cmp::Ordering;

    /// Comparator based on [Node] file names in lexicographical order.
    pub fn comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator based on [Node] file names in reversed lexicographical order.
    pub fn rev_comparator(a: &Node, b: &Node) -> Ordering {
        comparator(b, a)
    }
}
