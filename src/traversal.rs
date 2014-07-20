use {HashSet};
use std::collections::{HashMap, RingBuf, Deque};
use graph::{Graph, NodeIndex, Digraph};

/// Do a breadth-first search of the graph, returning the resulting breadth-
/// first tree (a tree on the connected component containing the stard node)
fn bfs_tree<T, G: Graph<T>>(g: &G, start: NodeIndex) -> Digraph<NodeIndex> {
    let mut tree = Digraph::new();

    if g.num_nodes() == 0 {
        return tree;
    }

    let mut visited = HashSet::new();
    let mut discovered = RingBuf::new();

    discovered.push_back((start, None));
    loop {
        match discovered.pop_front() {
            None => break,
            Some((ind, parent)) => {
                tree.add_node(ind);
                if parent.is_some() {
                    tree.add_edge(parent.unwrap(), ind);
                }
                visited.insert(ind);

                for i in g.adj(ind) {
                    if !visited.contains(&i) {
                        discovered.push_back((i, Some(ind)));
                    }
                }
            }
        }
    }
    return tree;
}

