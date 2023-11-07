use crate::{error::prelude::*, file::File, user::Context};
use ignore::{DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkParallel, WalkState};
use std::{
    ops::Deref,
    result::Result as StdResult,
    sync::mpsc::{self, Sender},
    thread,
};

pub fn run<F>(ctx: &Context, mut op: F) -> Result<()>
where
    F: FnMut(File) -> Result<()> + Send,
{
    let parallel_walker = WalkParallel::try_from(ctx)?;

    let (tx, rx) = mpsc::channel::<TraversalState>();
    let mut builder = VisitorBuilder::new(tx.clone(), ctx);

    thread::scope(move |scope| {
        let handle = scope.spawn(move || {
            loop {
                match rx.recv().into_report(ErrorCategory::Internal) {
                    Ok(TraversalState::Ongoing(file)) => op(file)?,
                    Ok(TraversalState::Error(e)) => log::warn!("{e}"),
                    Ok(TraversalState::Done) => break,
                    Err(e) => return Err(e),
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

pub struct Visitor<'a> {
    tx: Sender<TraversalState>,
    ctx: &'a Context,
}

pub struct VisitorBuilder<'a> {
    tx: Sender<TraversalState>,
    ctx: &'a Context,
}

pub enum TraversalState {
    Error(Error),
    Ongoing(File),
    Done,
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
            },
        };

        match File::init(entry, self.ctx).into_report(ErrorCategory::Warning) {
            Ok(file) => {
                let _ = self.send(TraversalState::Ongoing(file));
            },
            Err(e) => {
                let _ = self.send(TraversalState::Error(e));
            },
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
