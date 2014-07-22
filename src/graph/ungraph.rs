use std::fmt;
use super::{HashSet, HashMap};
use super::{Node, NodeIndex, NodeIndexSet, NodeIndices, Graph, GraphError, GraphResult};

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

impl<T> Ungraph<T> {
    pub fn new() -> Ungraph<T> {
        Ungraph { nodes: HashMap::new(), adj: HashMap::new(), num_nodes: 0 }
    }

    pub fn degree(&mut self, ind: NodeIndex) -> GraphResult<uint> {
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

    // return true if the nodes i and j are adjacent, and false otherwise
    pub fn are_adj(&self, i: NodeIndex, j: NodeIndex) -> bool {
        self.get_adj(i).contains(&j)
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

    fn remove_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else if !self.nodes.contains_key(&j) {
            Err(GraphError::invalid_index(j))
        } else {
            Ok(self.are_adj(i, j))
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
