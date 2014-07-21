use {HashSet};
use std::collections::{HashMap, RingBuf, Deque};

// call this Unigraph instead?
pub trait Graph<T> {
    /// Insert a new node, returning its index
    fn add_node(&mut self, val: T) -> NodeIndex;

    /// Insert a new edge, returning an error if one of the indices is invalid
    /// and a result otherwise
    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool>;

    /// Return an iterator over the out-neighbors for a given node
    fn adj(&self, i: NodeIndex) -> NodeIndices;

    /// Return the number of nodes in the graph
    fn num_nodes(&self) -> uint;

    fn node_indices(&self) -> NodeIndices;

    fn dfs(&self) -> Digraph<NodeIndex> {
        let mut tree = Digraph::new();

        if self.num_nodes() == 0 {
            return tree;
        }

        let mut visited = HashSet::new();
        let mut discovered = vec!();

        discovered.push((0, None));
        loop {
            match discovered.pop() {
                None => break,
                Some((ind, parent)) => {
                    tree.add_node(ind);
                    if parent.is_some() {
                        tree.add_edge(parent.unwrap(), ind);
                    }
                    visited.insert(ind);

                    for i in self.adj(ind) {
                        if !visited.contains(i) {
                            discovered.push((*i, Some(ind)));
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

pub type NodeIndex = uint;
type NodeIndexSet = HashSet<NodeIndex>;

#[deriving(Show, Clone)]
struct Node<T> {
    data: T,
    index: NodeIndex,
}

/// Undirected graph. Allows loops.
#[deriving(Show)]
pub struct UnGraph<T> {
    nodes: HashMap<NodeIndex, Node<T>>,
    adj: HashMap<NodeIndex, NodeIndexSet>,
    num_nodes: uint,
}

/// Directed graph. Allows loops.
#[deriving(Show)]
pub struct Digraph<T> {
    nodes: HashMap<NodeIndex, Node<T>>,
    in_adj: HashMap<NodeIndex, NodeIndexSet>,
    out_adj: HashMap<NodeIndex, NodeIndexSet>,
    num_nodes: uint,
}

impl<T> UnGraph<T> {
    pub fn new() -> UnGraph<T> {
        UnGraph { nodes: HashMap::new(), adj: HashMap::new(), num_nodes: 0 }
    }

    pub fn degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if !self.adj.contains_key(&ind) {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok( self.get_adj(ind).len() )
        }
    }

    pub fn get_node<'a>(&'a self, i: NodeIndex) -> &'a Node<T> {
        self.nodes.find(&i).unwrap()
    }

    pub fn get_adj<'a>(&'a self, i: NodeIndex) -> &'a NodeIndexSet {
        self.adj.find(&i).unwrap()
    }
}
impl<T: Clone> UnGraph<T> {
    /// Returns a new graph induced by a set of node indices
    pub fn induced_subgraph(&self, nodes: &NodeIndexSet) -> UnGraph<T> {
        let mut new = UnGraph::new();
        let mut ind_map = HashMap::new();

        for (i, ind) in nodes.iter().enumerate() {
            ind_map.insert(ind, i);
            new.add_node(self.get_node(*ind).data.clone());
        }

        // Here we're assuming that NodeIndex = uint. Not sure how to easily do otherwise
        for i in range(0, self.num_nodes) {
            for j in self.get_adj(i).iter() {
                new.add_edge(i, *j);
            }
        }

        new
    }
}

impl<T> Graph<T> for UnGraph<T> {
    fn add_node(&mut self, val: T) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.insert(ind, Node { data: val, index: ind });
        self.adj.insert(ind, HashSet::new());
        self.num_nodes += 1;
        ind
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else if !self.nodes.contains_key(&j) {
            Err(GraphError::invalid_index(j))
        } else {
            if self.get_adj(i).contains(&j) {
                Ok(false)
            } else {
                self.adj.find_mut(&i).unwrap().insert(j);
                self.adj.find_mut(&j).unwrap().insert(i);
                Ok(true)
            }
        }
    }

    fn adj(&self, i: NodeIndex) -> NodeIndices {
        FromIterator::from_iter(self.get_adj(i).iter().map(|&x| x))
    }

    fn num_nodes(&self) -> uint {
        self.num_nodes
    }

    fn node_indices(&self) -> NodeIndices {
        FromIterator::from_iter(self.nodes.keys().map(|&x| x))
    }
}

impl<T> Digraph<T> {
    pub fn new() -> Digraph<T> {
        Digraph { nodes: HashMap::new(),
                  in_adj: HashMap::new(),
                  out_adj: HashMap::new(),
                  num_nodes: 0 }
    }

    pub fn out_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if !self.nodes.contains_key(&ind) {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.get_out_adj(ind).len())
        }
    }

    pub fn in_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.get_in_adj(ind).len())
        }
    }

    pub fn get_node<'a>(&'a self, i: NodeIndex) -> &'a Node<T> {
        self.nodes.find(&i).unwrap()
    }

    pub fn get_in_adj<'a>(&'a self, i: NodeIndex) -> &'a NodeIndexSet {
        self.in_adj.find(&i).unwrap()
    }

    pub fn get_out_adj<'a>(&'a self, i: NodeIndex) -> &'a NodeIndexSet {
        self.out_adj.find(&i).unwrap()
    }
}

impl<T: Clone> Digraph<T> {
    /// Returns a new graph induced by a set of node indices
    pub fn induced_subgraph(&self, nodes: &NodeIndexSet) -> Digraph<T> {
        let mut new = Digraph::new();
        let mut ind_map = HashMap::new();

        for (i, ind) in nodes.iter().enumerate() {
            ind_map.insert(ind, i);
            new.add_node(self.get_node(*ind).data.clone());
        }

        // Here we're assuming that NodeIndex = uint. Not sure how to easily do otherwise
        for i in range(0, self.num_nodes) {
            for j in self.get_in_adj(i).iter() {
                new.add_edge(*j, i);
            }

            for j in self.get_out_adj(i).iter() {
                new.add_edge(i, *j);
            }
        }

        new
    }
}

impl<T> Graph<T> for Digraph<T> {
    fn add_node(&mut self, val: T) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.insert(ind, Node { data: val, index: ind });
        self.in_adj.insert(ind, HashSet::new());
        self.out_adj.insert(ind, HashSet::new());
        self.num_nodes += 1;
        ind
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else if !self.nodes.contains_key(&j) {
            Err(GraphError::invalid_index(j))
        } else {
            if self.get_out_adj(i).contains(&j) {
                Ok(false)
            } else {
                self.out_adj.find_mut(&i).unwrap().insert(j);
                self.in_adj.find_mut(&j).unwrap().insert(i);
                Ok(true)
            }
        }
    }

    fn adj(&self, i: NodeIndex) -> NodeIndices {
        FromIterator::from_iter(self.get_out_adj(i).iter().map(|&x| x))
    }

    fn num_nodes(&self) -> uint {
        self.num_nodes
    }

    fn node_indices(&self) -> NodeIndices {
        FromIterator::from_iter(self.nodes.keys().map(|&x| x))
    }
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
