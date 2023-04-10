use std::sync::mpsc::Sender;

use super::{Context, Node};
use ignore::{DirEntry, Error as IgnoreError, ParallelVisitor, ParallelVisitorBuilder, WalkState};

pub enum TraversalState {
    Ongoing(Node),
    Done,
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
        let Ok(dir_entry) = entry else {
            return WalkState::Skip;
        };

        match Node::try_from((dir_entry, self.ctx)) {
            Ok(node) => {
                self.tx.send(TraversalState::from(node)).unwrap();
                WalkState::Continue
            }
            _ => WalkState::Skip,
        }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for BranchVisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        let visitor = Branch::new(self.ctx, self.tx.clone());
        Box::new(visitor)
    }
}
