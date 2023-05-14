use crate::{ansi::Escaped, tree::node::Node, Context};
use cell::Cell;
use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

/// Concerned with rules to construct and a single cell in a given row.
pub mod cell;

pub struct Row<'a, T> {
    prefix: Option<&'a str>,
    ctx: &'a Context,
    node: &'a Node,
    layout: PhantomData<T>,
}

/// For both the [`super::Regular`] and [`super::Inverted`] layout variants.
pub struct Tree;

/// For the [`super::Flat`] variant.
pub struct Flat;

impl<'a, T> Row<'a, T> {
    pub const fn new(node: &'a Node, ctx: &'a Context, prefix: Option<&'a str>) -> Row<'a, T> {
        Self {
            prefix,
            node,
            ctx,
            layout: PhantomData,
        }
    }
}

#[cfg(unix)]
impl Display for Row<'_, Tree> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let size = Cell::new(node, ctx, cell::Kind::FileSize);
        let name = Cell::new(
            node,
            ctx,
            cell::Kind::FileName {
                prefix: self.prefix,
            },
        );

        let row = if ctx.long {
            let ino = Cell::new(node, ctx, cell::Kind::Ino);
            let perms = Cell::new(node, ctx, cell::Kind::Permissions);
            let nlink = Cell::new(node, ctx, cell::Kind::Nlink);
            let blocks = Cell::new(node, ctx, cell::Kind::Blocks);
            let time = Cell::new(node, ctx, cell::Kind::Datetime);

            format!("{ino} {perms} {nlink} {blocks} {time} {size} {name}")
        } else {
            format!("{size} {name}")
        };

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&row, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{row}")
        }
    }
}

#[cfg(not(unix))]
impl Display for Row<'_, Tree> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let size = Cell::new(node, ctx, cell::Kind::FileSize);
        let name = Cell::new(
            node,
            ctx,
            cell::Kind::FileName {
                prefix: self.prefix,
            },
        );

        let row = format!("{size} {name}");

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&row, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{row}")
        }
    }
}

#[cfg(unix)]
impl Display for Row<'_, Flat> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let size = Cell::new(node, ctx, cell::Kind::FileSize);
        let path = Cell::new(node, ctx, cell::Kind::FilePath);

        let row = if ctx.long {
            let ino = Cell::new(node, ctx, cell::Kind::Ino);
            let perms = Cell::new(node, ctx, cell::Kind::Permissions);
            let nlink = Cell::new(node, ctx, cell::Kind::Nlink);
            let blocks = Cell::new(node, ctx, cell::Kind::Blocks);
            let time = Cell::new(node, ctx, cell::Kind::Datetime);

            format!("{ino} {perms} {nlink} {blocks} {time} {size}   {path}")
        } else {
            format!("{size}   {path}")
        };

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&row, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{row}")
        }
    }
}

#[cfg(not(unix))]
impl Display for Row<'_, Flat> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.node;
        let ctx = self.ctx;

        let size = Cell::new(node, ctx, cell::Kind::FileSize);
        let path = Cell::new(node, ctx, cell::Kind::FilePath);

        let row = format!("{size}   {path}");

        if ctx.truncate && ctx.window_width.is_some() {
            let window_width = ctx.window_width.unwrap();
            let out = <str as Escaped>::truncate(&row, window_width);
            write!(f, "{out}")
        } else {
            write!(f, "{row}")
        }
    }
}
