pub use context::Context;

/// Operations to wrangle ANSI escaped strings.
pub mod ansi;

/// CLI rules and definitions as well as context to be injected throughout the entire program.
pub mod context;

/// Operations relevant to the computation and presentation of disk usage.
pub mod disk_usage;

/// Filesystem operations.
pub mod fs;

/// All things related to icons on how to map certain files to the appropriate icons.
pub mod icons;

/// Concerned with displaying a progress indicator when stdout is a tty.
pub mod progress;

/// Concerned with taking an initialized [`tree::Tree`] and its [`tree::node::Node`]s and rendering the output.
pub mod render;

/// Global used throughout the program to paint the output.
pub mod styles;

/// Houses the primary data structures that are used to virtualize the filesystem, containing also
/// information on how the tree output should be ultimately rendered.
pub mod tree;

/// Utilities relating to interacting with tty properties.
pub mod tty;

/// Common utilities across all pub modules.
pub mod utils;
