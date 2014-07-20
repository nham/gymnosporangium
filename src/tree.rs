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

    pub fn add_child(&mut self, parent: NodeIndex, val: T) {
        // TODO: check whether NodeIndex is valid?
        let node = Node { data: val, index: self.num_nodes, parent: Some(parent) };
        self.nodes.push(node);
        self.children.push(HashSet::new());
        self.children.get_mut(parent).insert(self.num_nodes);
        self.num_nodes += 1;
    }
}
