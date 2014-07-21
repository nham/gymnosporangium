use {HashSet, HashMap};
use std::collections::{RingBuf, Deque};
use std::fmt;

// call this Unigraph instead?
pub trait Graph<T> {
    /// Insert a new node, returning its index
    fn add_node(&mut self, val: T) -> NodeIndex;

    /// Insert a new edge, returning an error if one of the indices is invalid.
    /// Otherwise, return true if the edge was not already present.
    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool>;

    /// Remove a node, returning an error if the index is invalid.
    fn remove_node(&mut self, i: NodeIndex) -> GraphResult<()>;

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

/// Undirected graph. Allows loops.
pub struct Ungraph<T> {
    nodes: HashMap<NodeIndex, Node<T>>,
    adj: HashMap<NodeIndex, NodeIndexSet>,
    num_nodes: uint,
}

impl<T: fmt::Show> fmt::Show for Ungraph<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
        write!(fmt, "{{");
        for n in self.nodes.values() {
            write!(fmt, " {} ", n.data);
        }
        write!(fmt, "}}  ");

        for (&i, adj) in self.adj.iter() {
            for &j in adj.iter() {
                write!(fmt, "({}, {})", self.get_node(i).data, 
                                        self.get_node(j).data);
            }
        }

        Ok(())
    }
}

/// Directed graph. Allows loops.
pub struct Digraph<T> {
    nodes: HashMap<NodeIndex, Node<T>>,
    in_adj: HashMap<NodeIndex, NodeIndexSet>,
    out_adj: HashMap<NodeIndex, NodeIndexSet>,
    num_nodes: uint,
}

impl<T: fmt::Show> fmt::Show for Digraph<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
        write!(fmt, "{{");
        for n in self.nodes.values() {
            write!(fmt, " {} ", n.data);
        }
        write!(fmt, "}}  ");

        for (&i, adj) in self.out_adj.iter() {
            for &j in adj.iter() {
                write!(fmt, "({}, {})", self.get_node(i).data, 
                                        self.get_node(j).data);
            }
        }

        Ok(())
    }
}

impl<T> Ungraph<T> {
    pub fn new() -> Ungraph<T> {
        Ungraph { nodes: HashMap::new(), adj: HashMap::new(), num_nodes: 0 }
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

impl<T: Clone> Ungraph<T> {
    /// Returns a new graph induced by a set of node indices
    pub fn induced_subgraph(&self, nodes: &NodeIndexSet) -> Ungraph<T> {
        let mut new = Ungraph::new();
        let mut ind_map = HashMap::new(); // maps old indices to new

        for &ind in nodes.iter() {
            let new_ind = new.add_node(self.get_node(ind).data.clone());
            ind_map.insert(ind, new_ind);
        }

        for i in nodes.iter() {
            let actual_i = *ind_map.find(i).unwrap();
            for j in self.get_adj(*i).iter() {
                if nodes.contains(j) {
                    new.add_edge(actual_i, *ind_map.find(j).unwrap());
                }
            }
        }

        new
    }
}

impl<T> Graph<T> for Ungraph<T> {
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

    fn remove_node(&mut self, i: NodeIndex) -> GraphResult<()> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else {
            self.nodes.remove(&i);

            for j in self.adj(i) {
                self.adj.find_mut(&j).unwrap().remove(&i);
            }
            self.adj.remove(&i);
            self.num_nodes -= 1;
            Ok(())
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

    fn in_adj(&self, i: NodeIndex) -> NodeIndices {
        FromIterator::from_iter(self.get_in_adj(i).iter().map(|&x| x))
    }

    pub fn is_dag(&self) -> bool {
        false
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
        let mut ind_map = HashMap::new(); // maps old indices to new

        for &ind in nodes.iter() {
            let new_ind = new.add_node(self.get_node(ind).data.clone());
            ind_map.insert(ind, new_ind);
        }

        for i in nodes.iter() {
            let actual_i = *ind_map.find(i).unwrap();
            for j in self.get_in_adj(*i).iter() {
                if nodes.contains(j) {
                    new.add_edge(*ind_map.find(j).unwrap(), actual_i);
                }
            }

            for j in self.get_out_adj(*i).iter() {
                if nodes.contains(j) {
                    new.add_edge(actual_i, *ind_map.find(j).unwrap());
                }
            }
        }

        new
    }

    // Returns the transpose of the graph
    pub fn transpose(&self) -> Digraph<T> {
        let mut new = Digraph::new();
        let mut ind_map = HashMap::new(); // maps old indices to new

        for n in self.nodes.values() {
            let new_ind = new.add_node(n.data.clone());
            ind_map.insert(n.index, new_ind);
        }

        for &i in self.nodes.keys() {
            let actual_i = *ind_map.find(&i).unwrap();

            for j in self.get_in_adj(i).iter() {
                new.add_edge(actual_i, *ind_map.find(j).unwrap());
            }

            for j in self.get_out_adj(i).iter() {
                new.add_edge(*ind_map.find(j).unwrap(), actual_i);
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

    fn remove_node(&mut self, i: NodeIndex) -> GraphResult<()> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else {
            self.nodes.remove(&i);

            for j in self.adj(i) {
                self.in_adj.find_mut(&j).unwrap().remove(&i);
            }
            self.out_adj.remove(&i);

            for j in self.in_adj(i) {
                self.out_adj.find_mut(&j).unwrap().remove(&i);
            }
            self.in_adj.remove(&i);

            self.num_nodes -= 1;
            Ok(())
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
