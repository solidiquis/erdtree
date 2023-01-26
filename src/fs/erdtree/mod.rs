pub mod tree;

/// A directory entry represented as nodes in a tree data structure. The reason for `SharedNode`
/// and `LocalNode` has to do with the fact that traversing the filesystem to construct the nodes
/// happens in a multi-threaded context, but processing the nodes to assemble the `Tree` structure
/// happens in a single-threaded context.
///
/// The filesystem is traversed in parallel to create `SharedNode`s, which is then converted into
/// `LocalNode`s to make use of single-threaded reference counters i.e. `Rc<LocalNode>` when
/// constructing the `Tree` data structure.
pub mod node;
