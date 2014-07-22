pub use std::collections::{HashSet, HashMap};
use std::collections::{RingBuf, Deque};

pub use self::ungraph::Ungraph;
pub use self::digraph::Digraph;

mod ungraph;
mod digraph;

// call this Unigraph instead?
pub trait Graph<T> {
    /// Insert a new node, returning its index
    fn add_node(&mut self, val: T) -> NodeIndex;

    /// Insert a new edge, returning an error if one of the indices is invalid.
    /// Otherwise, return true if the edge was not already present.
    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool>;

    /// Remove a node, returning an error if the index is invalid.
    fn remove_node(&mut self, i: NodeIndex) -> GraphResult<()>;

    /// Remove an edge, returning an error if one of the indices is invalid
    /// Otherwise, return true if the edge was already present.
    fn remove_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool>;

    /// Return an iterator over the out-neighbors for a given node
    fn adj(&self, i: NodeIndex) -> NodeIndices;

    /// Return the number of nodes in the graph
    fn num_nodes(&self) -> uint;

    /// Returns an iterator over the indices of all the nodes in the graph
    fn node_indices(&self) -> NodeIndices;
}

struct GraphError {
    error_type: GraphErrorType,
}

impl GraphError {
    fn invalid_index(ind: NodeIndex) -> GraphError {
        GraphError { error_type: InvalidNodeIndex(ind) }
    }
}

type GraphResult<T> = Result<T, GraphError>;

enum GraphErrorType {
    InvalidNodeIndex(NodeIndex),
}

pub type NodeIndex = uint;
type NodeIndexSet = HashSet<NodeIndex>;

#[deriving(Show, Clone)]
struct Node<T> {
    data: T,
    index: NodeIndex,
}


struct NodeIndices {
    indices: Vec<NodeIndex>,
    curr: uint,
}

impl Iterator<NodeIndex> for NodeIndices {
    fn next(&mut self) -> Option<NodeIndex> {
        if self.curr < self.indices.len() {
            self.curr += 1;
            Some(self.indices[self.curr - 1])
        } else {
            None
        }
    }
}

impl FromIterator<NodeIndex> for NodeIndices {
    fn from_iter<T: Iterator<NodeIndex>>(mut it: T) -> NodeIndices {
        let mut vec = vec!();
        for i in it {
            vec.push(i);
        }
        NodeIndices { indices: vec, curr: 0 }
    }
}
