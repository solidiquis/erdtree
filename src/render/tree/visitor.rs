use crossbeam::channel::Sender;
use ignore::{
    DirEntry,
    Error as IgnoreError,
    ParallelVisitor,
    ParallelVisitorBuilder,
    WalkState,
};
use super::{Context, Node};

pub enum TraversalState {
    Ongoing(Node),
    Done
}

pub struct BranchVisitor<'a> {
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

impl<'a> BranchVisitor<'a> {
    pub fn new(ctx: &'a Context, tx: Sender<TraversalState>) -> Self {
        Self { ctx, tx }
    }
}

impl From<Node> for TraversalState {
    fn from(node: Node) -> Self {
        TraversalState::Ongoing(node)
    }
}

impl ParallelVisitor for BranchVisitor<'_> {
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
        let visitor = BranchVisitor::new(self.ctx, self.tx.clone());
        Box::new(visitor)
    }
}
