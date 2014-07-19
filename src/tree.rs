pub enum Tree<T> {
    Nil,
    Node(T, Vec<Tree<T>>),
}
