use crate::cli;
use super::node::Node;
use std::{
    convert::From,
    cmp::Ordering,
};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    Name,
    Size,
    None,
}

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<fn(a: &Node, b: &Node) -> Ordering> {
        match self {
            Self::Name => Some(Self::name_comparator),
            Self::Size => Some(Self::size_comparator),
            _ => None,
        }
    }

    /// Comparator based on `Node` file names.
    fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator based on `Node` file sizes
    fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size.unwrap_or(0);
        let b_size = b.file_size.unwrap_or(0);

        b_size.cmp(&a_size)
    }
}

impl From<cli::Order> for Order {
    fn from(ord: cli::Order) -> Self {
        match ord {
            cli::Order::Name => Order::Name,
            cli::Order::Size => Order::Size,
            cli::Order::None => Order::None
        }
    }
}
