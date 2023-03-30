use std::sync::mpsc::Sender;

use super::{Context, Error, Node};
use ignore::{DirEntry, Error as IgnoreError, ParallelVisitor, ParallelVisitorBuilder, WalkState};

pub enum TraversalState {
    Ongoing(Node),
    Done,
    Error(Error),
}

pub struct Branch<'a> {
    ctx: &'a Context,
    tx: Sender<TraversalState>,
}

pub struct BranchVisitorBuilder<'a> {
    ctx: &'a Context,
    tx: Sender<TraversalState>,
}

impl<'a> BranchVisitorBuilder<'a> {
    pub fn new(ctx: &'a Context, tx: Sender<TraversalState>) -> Self {
        Self { ctx, tx }
    }
}

impl<'a> Branch<'a> {
    pub fn new(ctx: &'a Context, tx: Sender<TraversalState>) -> Self {
        Self { ctx, tx }
    }
}

impl From<Node> for TraversalState {
    fn from(node: Node) -> Self {
        Self::Ongoing(node)
    }
}

impl ParallelVisitor for Branch<'_> {
    fn visit(&mut self, entry: Result<DirEntry, IgnoreError>) -> WalkState {
        entry
            .map(|e| TraversalState::from(Node::from((&e, self.ctx))))
            .map(|n| self.tx.send(n).unwrap())
            .map(|_| WalkState::Continue)
            .unwrap_or(WalkState::Skip)
    }
}

impl<'s> ParallelVisitorBuilder<'s> for BranchVisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        let visitor = Branch::new(self.ctx, self.tx.clone());
        Box::new(visitor)
    }
}
