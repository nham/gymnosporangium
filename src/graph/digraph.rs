use std::fmt;
use super::{HashSet, HashMap};
use super::{Node, NodeIndex, NodeIndexSet, NodeIndices, Graph, GraphError, GraphResult};

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

impl<T> Digraph<T> {
    pub fn new() -> Digraph<T> {
        Digraph { nodes: HashMap::new(),
                  in_adj: HashMap::new(),
                  out_adj: HashMap::new(),
                  num_nodes: 0 }
    }

    pub fn out_degree(&mut self, ind: NodeIndex) -> GraphResult<uint> {
        if !self.nodes.contains_key(&ind) {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.get_out_adj(ind).len())
        }
    }

    pub fn in_degree(&mut self, ind: NodeIndex) -> GraphResult<uint> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.get_in_adj(ind).len())
        }
    }

    fn in_adj(&self, i: NodeIndex) -> NodeIndices {
        FromIterator::from_iter(self.get_in_adj(i).iter().map(|&x| x))
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

    // return true if j is an out-neighbor of i, and false otherwise
    pub fn is_out_adj_to(&self, i: NodeIndex, j: NodeIndex) -> bool {
        self.get_out_adj(i).contains(&j)
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

            for j in self.reachable(i) {
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

    fn remove_edge(&mut self, i: NodeIndex, j: NodeIndex) -> GraphResult<bool> {
        if !self.nodes.contains_key(&i) {
            Err(GraphError::invalid_index(i))
        } else if !self.nodes.contains_key(&j) {
            Err(GraphError::invalid_index(j))
        } else {
            Ok(self.is_out_adj_to(i, j))
        }
    }

    fn reachable(&self, i: NodeIndex) -> NodeIndices {
        FromIterator::from_iter(self.get_out_adj(i).iter().map(|&x| x))
    }

    fn num_nodes(&self) -> uint {
        self.num_nodes
    }

    fn node_indices(&self) -> NodeIndices {
        FromIterator::from_iter(self.nodes.keys().map(|&x| x))
    }
}
