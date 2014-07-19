struct TreeNode<T> {
    data: T,
    children: Vec<TreeNode<T>>,
}

pub struct Tree<T> {
    root: Option<TreeNode<T>>
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn leaf(val: T) -> Tree<T> {
        Tree { root: Some(TreeNode { data: val, children: vec!() }) }
    }
}
