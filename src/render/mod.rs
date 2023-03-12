/// CLI rules and definitions and context wherein [Tree] will operate.
///
/// [`Tree`]: tree::Tree
pub mod context;

/// Operations that decide how to present info about disk usage.
pub mod disk_usage;

/// Ordering operations for printing.
pub mod order;

/// Encapsulates everything related to the in-memory representation of the root directory and its
/// contents.
pub mod tree;

pub use tree::ui::get_ls_colors;
