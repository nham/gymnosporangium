use std::collections::HashSet;

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

struct Node<S> {
    data: S,
    index: NodeIndex,
}

impl<S> Graph<S> {
    fn add_node(&mut self, val: S) {
        self.nodes.push(Node { data: val, index: self.num_nodes });
        self.adj.push(HashSet::new());
        self.num_nodes += 1;
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> Result<bool, GraphError> {
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

    fn degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.adj[ind].len())
        }
    }
}

struct Digraph<S> {
    nodes: Vec<Node<S>>,
    in_adj: Vec<HashSet<NodeIndex>>,
    out_adj: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

impl<S> Digraph<S> {
    fn add_node(&mut self, val: S) {
        self.nodes.push(Node { data: val, index: self.num_nodes });
        self.in_adj.push(HashSet::new());
        self.out_adj.push(HashSet::new());
        self.num_nodes += 1;
    }

    fn add_edge(&mut self, i: NodeIndex, j: NodeIndex) -> Result<bool, GraphError> {
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

    fn out_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.out_adj[ind].len())
        }
    }

    fn in_degree(&mut self, ind: NodeIndex) -> Result<uint, GraphError> {
        if ind >= self.num_nodes {
            Err(GraphError::invalid_index(ind))
        } else {
            Ok(self.in_adj[ind].len())
        }
    }
}

fn main() {

}
