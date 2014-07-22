use std::collections::{HashSet, HashMap};

type NodeIndex = uint;
type NodeIndexSet = HashSet<NodeIndex>;

#[deriving(Show)]
struct Node<T> {
    data: T,
    index: NodeIndex,
    parent: Option<NodeIndex>,
}

#[deriving(Show)]
pub struct Tree<T> {
    root: Option<NodeIndex>,
    nodes: HashMap<NodeIndex, Node<T>>,
    children: HashMap<NodeIndex, NodeIndexSet>,
    num_nodes: uint,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None, nodes: HashMap::new(), 
               children: HashMap::new(), num_nodes: 0 }
    }

    pub fn with_root(val: T) -> Tree<T> {
        let root = Node { data: val, index: 0, parent: None };
        let mut map = HashMap::new();
        map.insert(0, root);
        Tree { root: Some(0), nodes: map, 
               children: HashMap::new(), num_nodes: 1 }
    }

    /// Add a root to any empty tree
    pub fn add_root(&mut self, val: T) {
        self.add_node(None, val);
    }

    pub fn add_child(&mut self, parent: NodeIndex, val: T) {
        self.add_node(Some(parent), val);
    }

    fn add_node(&mut self, parent: Option<NodeIndex>, val: T) -> NodeIndex {
        // TODO: check whether NodeIndex is valid?
        let ind = self.num_nodes;
        let node = Node { data: val, index: ind, parent: parent };
        self.nodes.insert(ind, node);
        self.children.insert(ind, HashSet::new());

        if parent.is_some() {
            self.children.find_mut(&parent.unwrap()).unwrap().insert(ind);
        }

        self.num_nodes += 1;
        ind
    }
}
