use crate::{error::prelude::*, file::File, user::Context};
use ignore::{DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkState};
use std::{
    ops::Deref,
    result::Result as StdResult,
    sync::mpsc::{self, Sender},
    thread,
};

/// Parallel traversal algorithm. `op` takes in a single argument which is the [`File`] that is
/// retrieved from disk, returning a [`Result`]. If `op` returns an `Err` then traversal will
/// immediately conclude.
pub fn run<F>(ctx: &Context, mut op: F) -> Result<()>
where
    F: FnMut(File) -> Result<()> + Send,
{
    let parallel_walker = walker::init(ctx)?;

    let (tx, rx) = mpsc::channel::<TraversalState>();
    let mut builder = VisitorBuilder::new(tx.clone(), ctx);

    thread::scope(move |scope| {
        let handle = scope.spawn(move || {
            loop {
                match rx.recv().into_report(ErrorCategory::Internal) {
                    Ok(TraversalState::Ongoing(file)) => op(file)?,
                    Ok(TraversalState::Done) => break,
                    Err(e) => return Err(e),
                    _ => (),
                }
            }
            Ok(())
        });

        parallel_walker.visit(&mut builder);
        let _ = tx.send(TraversalState::Done);

        handle.join().unwrap()
    })?;

    Ok(())
}

pub enum TraversalState {
    Error(Error),
    Ongoing(File),
    Done,
}

pub struct Visitor<'a> {
    tx: Sender<TraversalState>,
    ctx: &'a Context,
}

pub struct VisitorBuilder<'a> {
    tx: Sender<TraversalState>,
    ctx: &'a Context,
}

impl<'a> VisitorBuilder<'a> {
    pub fn new(tx: Sender<TraversalState>, ctx: &'a Context) -> Self {
        Self { tx, ctx }
    }
}

impl<'a> Visitor<'a> {
    pub fn new(tx: Sender<TraversalState>, ctx: &'a Context) -> Self {
        Self { tx, ctx }
    }
}

impl ParallelVisitor for Visitor<'_> {
    fn visit(&mut self, entry: StdResult<DirEntry, ignore::Error>) -> WalkState {
        let entry = match entry.into_report(ErrorCategory::Warning) {
            Ok(entry) => entry,
            Err(e) => {
                let _ = self.send(TraversalState::Error(e));
                return WalkState::Continue;
            }
        };

        match File::init(entry, self.ctx).into_report(ErrorCategory::Warning) {
            Ok(file) => {
                let _ = self.send(TraversalState::Ongoing(file));
            }
            Err(e) => {
                let _ = self.send(TraversalState::Error(e));
            }
        }

        WalkState::Continue
    }
}

impl<'a> ParallelVisitorBuilder<'a> for VisitorBuilder<'a> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(Visitor::new(Sender::clone(&self.tx), self.ctx))
    }
}

impl Deref for Visitor<'_> {
    type Target = Sender<TraversalState>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

mod walker {
    use crate::{error::prelude::*, user::Context};
    use ignore::{
        overrides::{Override, OverrideBuilder},
        WalkBuilder, WalkParallel,
    };
    use std::path::Path;

    pub fn init(ctx: &Context) -> Result<WalkParallel> {
        let path = ctx.dir_canonical()?;
        let mut builder = WalkBuilder::new(&path);

        let walker = builder
            .follow_links(ctx.follow)
            .git_ignore(ctx.gitignore)
            .git_global(ctx.global_gitignore)
            .threads(ctx.threads)
            .hidden(ctx.no_hidden)
            .same_file_system(ctx.same_fs);

        if ctx.suppress_size {
            walker.max_depth(ctx.level);
        }

        let overrides = build_overrides(ctx, &path)?;
        walker.overrides(overrides);

        Ok(walker.build_parallel())
    }

    pub fn build_overrides(ctx: &Context, path: &Path) -> Result<Override> {
        let mut builder = OverrideBuilder::new(path);

        if ctx.no_git {
            builder
                .add("!.git")
                .into_report(ErrorCategory::Internal)
                .context(error_source!())?;
        }

        builder
            .build()
            .into_report(ErrorCategory::Internal)
            .context(error_source!())
    }
}