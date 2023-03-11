/// CLI rules and definitions and context wherein [Tree] will operate.
///
/// [`Tree`]: tree::Tree
pub mod context;

/// Operations that decide how to present info about disk usage.
pub mod disk_usage;

/// Contains components of the [`Tree`] data structure that derive from [`DirEntry`].
///
/// [`Tree`]: tree::Tree
/// [`DirEntry`]: ignore::DirEntry
pub mod node;

/// Ordering operations for printing.
pub mod order;

/// Encapsulates everything related to the in-memory representation of the root directory and its
/// contents.
pub mod tree;

pub use tree::ui::get_ls_colors;
