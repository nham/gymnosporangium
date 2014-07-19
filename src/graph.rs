use std::collections::{HashSet, RingBuf, Deque};
use std::collections::hashmap::SetItems;

trait Graph<T> {
    /// Insert a new node, returning its index
    fn add_node(&mut self, val: T) -> NodeIndex;

    /// Insert a new edge, returning an error if one of the indices is invalid
    /// and a result otherwise
    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool>;

    fn adj<'a>(&'a self, i: NodeIndex) -> NodeIndices<'a>;

    /// Return the number of nodes in the graph
    fn num_nodes(&self) -> uint;

    fn bfs(&self) -> Digraph<NodeIndex> {
        let mut tree = Digraph::new();

        if self.num_nodes() == 0 {
            return tree;
        }

        let mut visited = HashSet::new();
        let mut ondeck = RingBuf::new();

        ondeck.push_back((0, None));
        loop {
            match ondeck.pop_front() {
                None => break,
                Some((ind, parent)) => {
                    tree.add_node(ind);
                    if parent.is_some() {
                        tree.add_edge(parent.unwrap(), ind);
                    }
                    visited.insert(ind);

                    for i in self.adj(ind) {
                        if !visited.contains(i) {
                            ondeck.push_back((*i, Some(ind)));
                        }
                    }
                }
            }
        }
        return tree;
    }
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

type NodeIndex = uint;

#[deriving(Show)]
struct Node<T> {
    data: T,
    index: NodeIndex,
}

/// Undirected graph. Allows loops.
#[deriving(Show)]
struct UnGraph<T> {
    nodes: Vec<Node<T>>,
    adj: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

/// Directed graph. Allows loops.
#[deriving(Show)]
struct Digraph<T> {
    nodes: Vec<Node<T>>,
    in_adj: Vec<HashSet<NodeIndex>>,
    out_adj: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

impl<T> UnGraph<T> {
    pub fn new() -> UnGraph<T> {
        UnGraph { nodes: vec!(), adj: vec!(), num_nodes: 0 }
    }

    pub fn degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.adj[ind].len())
        }
    }
}

impl<T> Graph<T> for UnGraph<T> {
    fn add_node(&mut self, val: T) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.push(Node { data: val, index: ind });
        self.adj.push(HashSet::new());
        self.num_nodes += 1;
        ind
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if i >= self.num_nodes {
            Err(GraphError::invalid_index(i))
        } else if j >= self.num_nodes {
            Err(GraphError::invalid_index(j))
        } else {
            if self.adj[i].contains(&j) {
                Ok(false)
            } else {
                self.adj.get_mut(i).insert(j);
                self.adj.get_mut(j).insert(i);
                Ok(true)
            }
        }
    }

    fn adj<'a>(&'a self, i: NodeIndex) -> NodeIndices<'a> {
        NodeIndices::from_set(&self.adj[i])
    }

    fn num_nodes(&self) -> uint {
        self.num_nodes
    }
}

impl<T> Digraph<T> {
    pub fn new() -> Digraph<T> {
        Digraph { nodes: vec!(), in_adj: vec!(), out_adj: vec!(), num_nodes: 0 }
    }

    pub fn out_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.out_adj[ind].len())
        }
    }

    pub fn in_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.in_adj[ind].len())
        }
    }
}

impl<T> Graph<T> for Digraph<T> {
    fn add_node(&mut self, val: T) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.push(Node { data: val, index: ind });
        self.in_adj.push(HashSet::new());
        self.out_adj.push(HashSet::new());
        self.num_nodes += 1;
        ind
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if i >= self.num_nodes {
            Err(GraphError::invalid_index(i))
        } else if j >= self.num_nodes {
            Err(GraphError::invalid_index(j))
        } else {
            if self.out_adj[i].contains(&j) {
                Ok(false)
            } else {
                self.out_adj.get_mut(i).insert(j);
                self.in_adj.get_mut(j).insert(i);
                Ok(true)
            }
        }
    }

    fn adj<'a>(&'a self, i: NodeIndex) -> NodeIndices<'a> {
        NodeIndices::from_set(&self.out_adj[i])
    }

    fn num_nodes(&self) -> uint {
        self.num_nodes
    }
}

struct NodeIndices<'a> {
    indices: SetItems<'a, NodeIndex>,
}

impl<'a> Iterator<&'a NodeIndex> for NodeIndices<'a> {
    fn next(&mut self) -> Option<&'a NodeIndex> {
        self.indices.next()
    }
}

impl<'a> NodeIndices<'a> {
    fn from_set(set: &'a HashSet<NodeIndex>) -> NodeIndices<'a> {
        NodeIndices { indices: set.iter() }
    }
}
