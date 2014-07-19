use std::collections::{HashSet, RingBuf, Deque};

struct GraphError {
    error_type: GraphErrorType,
}

impl GraphError {
    fn invalid_index(ind: NodeIndex) -> GraphError {
        GraphError { error_type: InvalidNodeIndex(ind) }
    }
}

enum GraphErrorType {
    InvalidNodeIndex(NodeIndex),
}

type NodeIndex = uint;

struct Graph<S> {
    nodes: Vec<Node<S>>,
    adj: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

#[deriving(Show)]
struct Node<S> {
    data: S,
    index: NodeIndex,
}

impl<S> Graph<S> {
    pub fn new() -> Graph<S> {
        Graph { nodes: vec!(), adj: vec!(), num_nodes: 0 }
    }

    /// Insert a new node, returning its index
    pub fn add_node(&mut self, val: S) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.push(Node { data: val, index: ind });
        self.adj.push(HashSet::new());
        self.num_nodes += 1;
        ind
    }

    pub fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> Result<bool, GraphError> {
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

    pub fn degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.adj[ind].len())
        }
    }

    pub fn bfs(&self) -> Digraph<NodeIndex> {
        let mut tree = Digraph::new();

        if self.num_nodes == 0 {
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

                    for i in self.adj[ind].iter() {
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

#[deriving(Show)]
struct Digraph<S> {
    nodes: Vec<Node<S>>,
    in_adj: Vec<HashSet<NodeIndex>>,
    out_adj: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

impl<S> Digraph<S> {
    pub fn new() -> Digraph<S> {
        Digraph { nodes: vec!(), in_adj: vec!(), out_adj: vec!(), num_nodes: 0 }
    }

    /// Insert a new node, returning its index
    pub fn add_node(&mut self, val: S) -> NodeIndex {
        let ind = self.num_nodes;
        self.nodes.push(Node { data: val, index: ind });
        self.in_adj.push(HashSet::new());
        self.out_adj.push(HashSet::new());
        self.num_nodes += 1;
        ind
    }

    pub fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> Result<bool, GraphError> {
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

    pub fn bfs(&self) -> Digraph<NodeIndex> {
        let mut tree = Digraph::new();

        if self.num_nodes == 0 {
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

                    for i in self.out_adj[ind].iter() {
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
