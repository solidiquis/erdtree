use clap::ValueEnum;

/// Which layout to use when rendering the tree.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Type {
    /// Outputs the tree with the root node at the bottom of the output
    #[default]
    Regular,

    /// Outputs the tree with the root node at the top of the output
    Inverted,

    /// Outputs a flat layout using paths rather than an ASCII tree
    Flat,

    /// Outputs an inverted flat layout with the root at the to
    InvFlat,
}
