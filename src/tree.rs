use {HashSet};

type NodeIndex = uint;

#[deriving(Show)]
struct Node<T> {
    data: T,
    index: NodeIndex,
    parent: Option<NodeIndex>,
}

#[deriving(Show)]
pub struct Tree<T> {
    root: Option<NodeIndex>,
    nodes: Vec<Node<T>>,
    children: Vec<HashSet<NodeIndex>>,
    num_nodes: uint,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None, nodes: vec!(), children: vec!(), num_nodes: 0 }
    }

    pub fn with_root(val: T) -> Tree<T> {
        let root = Node { data: val, index: 0, parent: None };
        Tree { root: Some(0), nodes: vec!(root), children: vec!(), num_nodes: 1 }
    }

    /// Add a root to any empty tree
    pub fn add_root(&mut self, val: T) {
        self.add_node(None, val);
    }

    pub fn add_child(&mut self, parent: NodeIndex, val: T) {
        self.add_node(Some(parent), val);
    }

    fn add_node(&mut self, parent: Option<NodeIndex>, val: T) {
        // TODO: check whether NodeIndex is valid?
        let node = Node { data: val, index: self.num_nodes, parent: parent };
        self.nodes.push(node);
        self.children.push(HashSet::new());

        if parent.is_some() {
            self.children.get_mut(parent.unwrap()).insert(self.num_nodes);
        }

        self.num_nodes += 1;
    }
}
