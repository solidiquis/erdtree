use crate::{context::Context, tree::Tree};
use std::marker::PhantomData;

/// Module containing all of the layout variants.
pub mod layout;

/// Concerned with how to construct a single row in the output grid.
pub mod grid;

/// Utility module to fetch the appropriate theme used to paint the box-drawing characters of the
/// output tree.
pub mod theme;

/// The struct that is generic over T, which is generally expected to be a unit-struct that
/// ultimately determines which variant to use for the output.
pub struct Engine<T> {
    ctx: Context,
    tree: Tree,
    layout: PhantomData<T>,
}

/// The flat output that is similar to `du`, without the ASCII tree.
pub struct Flat;

/// The tree output with the root directory at the bottom of the output.
pub struct Regular;

/// The tree output with the root directory at the top of the output. More like the traditional
/// `tree` command.
pub struct Inverted;

impl<T> Engine<T> {
    /// Initializes a new [Engine].
    pub const fn new(tree: Tree, ctx: Context) -> Self {
        Self {
            ctx,
            tree,
            layout: PhantomData,
        }
    }

    /// Getter for the inner [Context] object.
    const fn context(&self) -> &Context {
        &self.ctx
    }

    /// Getter for the inner [Tree] data structure.
    const fn tree(&self) -> &Tree {
        &self.tree
    }
}
