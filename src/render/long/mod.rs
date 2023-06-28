use super::grid::cell::{self, Cell};
use crate::{context::Context, tree::node::Node};
use std::{convert::From, fmt};

/// Concerned with displaying that actual attributes associated with the long view.
pub struct Display<'a> {
    node: &'a Node,
    ctx: &'a Context,
    optional: Optionals,
}

/// Optionals fields that are displayed when `--long` is specified. Each field is a boolean,
/// specifying whether or not a particular field should be included in the output when the long
/// view is enabled.
pub struct Optionals {
    #[allow(dead_code)]
    perms: bool,
    #[allow(dead_code)]
    owner: bool,
    group: bool,
    ino: bool,
    #[allow(dead_code)]
    nlink: bool,
    #[allow(dead_code)]
    time: bool,
}

impl<'a> Display<'a> {
    /// Constructor for [`Display`].
    pub const fn new(optional: Optionals, node: &'a Node, ctx: &'a Context) -> Self {
        Self {
            node,
            ctx,
            optional,
        }
    }
}

/// Default implementation for [`Optionals`]. Fields that are `true` are the default fields that
/// should display when the long view is enabled.
impl Default for Optionals {
    fn default() -> Self {
        Self {
            perms: true,
            owner: true,
            group: false,
            ino: false,
            nlink: false,
            time: true,
        }
    }
}

impl fmt::Display for Display<'_> {
    /// Formatting the attributes associated with the long view.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Optionals {
            group, ino, nlink, ..
        } = self.optional;
        let node = self.node;
        let ctx = self.ctx;

        let perms = Cell::new(node, ctx, cell::Kind::Permissions);
        let owner = Cell::new(node, ctx, cell::Kind::Owner);
        let time = Cell::new(node, ctx, cell::Kind::Datetime);

        match (group, ino, nlink) {
            (false, false, false) => {
                write!(f, "{perms} {owner} {time}")
            },

            (true, true, true) => {
                let group_out = Cell::new(node, ctx, cell::Kind::Group);
                let ino_out = Cell::new(node, ctx, cell::Kind::Ino);
                let nlink_out = Cell::new(node, ctx, cell::Kind::Nlink);

                write!(
                    f,
                    "{ino_out} {perms} {nlink_out} {owner} {group_out} {time}"
                )
            },

            (true, false, false) => {
                let group_out = Cell::new(node, ctx, cell::Kind::Group);

                write!(f, "{perms} {owner} {group_out} {time}")
            },

            (true, true, false) => {
                let group_out = Cell::new(node, ctx, cell::Kind::Group);
                let ino_out = Cell::new(node, ctx, cell::Kind::Ino);

                write!(f, "{ino_out} {perms} {owner} {group_out} {time}")
            },

            (false, false, true) => {
                let nlink_out = Cell::new(node, ctx, cell::Kind::Nlink);

                write!(f, "{perms} {nlink_out} {owner} {time}")
            },

            (true, false, true) => {
                let group_out = Cell::new(node, ctx, cell::Kind::Group);
                let nlink_out = Cell::new(node, ctx, cell::Kind::Nlink);

                write!(f, "{perms} {nlink_out} {owner} {group_out} {time}")
            },

            (false, true, false) => {
                let ino_out = Cell::new(node, ctx, cell::Kind::Ino);

                write!(f, "{ino_out} {perms} {owner} {time}")
            },

            (false, true, true) => {
                let ino_out = Cell::new(node, ctx, cell::Kind::Ino);
                let nlink_out = Cell::new(node, ctx, cell::Kind::Nlink);

                write!(f, "{ino_out} {perms} {nlink_out} {owner} {time}")
            },
        }
    }
}

impl From<&Context> for Optionals {
    fn from(ctx: &Context) -> Self {
        let Context {
            group, ino, nlink, ..
        } = *ctx;

        Self {
            group,
            ino,
            nlink,
            ..Self::default()
        }
    }
}
