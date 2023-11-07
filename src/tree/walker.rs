use crate::{error::prelude::*, user::Context};
use ignore::{Walk, WalkBuilder, WalkParallel};
use std::convert::TryFrom;

/// Initializes a [`Walk`] instance from [`Context`] for single-threaded disk-reads as well as
/// [`FileTree`] processing.
impl TryFrom<&Context> for Walk {
    type Error = Error;

    fn try_from(ctx: &Context) -> Result<Self> {
        init_builder(ctx).map(|builder| builder.build())
    }
}

/// Initializes a [`WalkParallel`] instance from [`Context`] for multi-threaded disk-reads as well
/// as [`FileTree`] processing.
impl TryFrom<&Context> for WalkParallel {
    type Error = Error;

    fn try_from(ctx: &Context) -> Result<Self> {
        init_builder(ctx).map(|builder| builder.build_parallel())
    }
}

/// Initializes a [`WalkBuilder`] from [`Context`].
fn init_builder(ctx: &Context) -> Result<WalkBuilder> {
    let path = ctx.dir_canonical()?;
    let mut builder = WalkBuilder::new(path);

    builder
        .follow_links(ctx.follow)
        .git_ignore(!ctx.no_ignore)
        .git_global(!ctx.no_ignore)
        .threads(ctx.threads)
        .hidden(!ctx.hidden)
        .same_file_system(ctx.same_fs);

    Ok(builder)
}
