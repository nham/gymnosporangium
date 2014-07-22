use std::collections::{HashSet, HashMap, RingBuf, Deque};

use graph::{Graph, NodeIndex};
use tree::Tree;

type Forest<T> = Vec<Tree<T>>;

/// Do a breadth-first traversal of the graph, returning the resulting breadth-
/// first tree (a tree on the connected component containing the start node)
///
/// Pseudocode:
///
///    BFS(G, v):
///        S = new queue
///        S.insert(v)
///        mark v as discovered
///        while S is not empty:
///            curr = S.pop()
///            for all nodes w reachable from curr
///                if w is not yet discovered
///                    S.insert(w)
///                    mark w as discovered
fn bf_trav<T, G: Graph<T>>(g: &G, start: NodeIndex) -> Tree<NodeIndex> {
    let mut tree = Tree::new();

    if g.num_nodes() == 0 {
        return tree;
    }

    let mut discovered = HashSet::new();
    let mut queue = RingBuf::new();

    queue.push_back((start, None));
    discovered.insert(start);
    loop {
        match queue.pop_front() {
            None => break,
            Some((ind, parent)) => {
                match parent {
                    None => tree.add_root(ind),
                    Some(p_ind) => tree.add_child(p_ind, ind),
                }

                for i in g.reachable(ind) {
                    if !discovered.contains(&i) {
                        queue.push_back((i, Some(ind)));
                        discovered.insert(i);
                    }
                }
            }
        }
    }
    return tree;
}


/// Do a depth-first traversal of the graph, returning the resulting depth-
/// first tree
///
/// Pseudocode:
///    DFS(G, v):
///        S = new stack
///        S.push(v)
///        while S is not empty:
///            curr = S.pop()
///            if curr is not yet visited:
///                add curr to visited
///                for all nodes w reachable from curr
///                    S.push(w)
fn df_trav<T, G: Graph<T>>(g: &G, start: NodeIndex) -> Tree<NodeIndex> {
    let mut tree = Tree::new();

    if g.num_nodes() == 0 {
        return tree;
    }

    let mut visited = HashSet::new();
    let mut stack = vec!();

    stack.push((start, None));
    loop {
        match stack.pop() {
            None => break,
            Some((ind, parent)) => {
                if !visited.contains(&ind) {
                    match parent {
                        None => tree.add_root(ind),
                        Some(p_ind) => tree.add_child(p_ind, ind),
                    }
                    visited.insert(ind);

                    for i in g.reachable(ind) {
                        stack.push((i, Some(ind)));
                    }
                }
            }
        }
    }
    return tree;
}

fn df_trav_all<T, G: Graph<T>>(g: &G, start: NodeIndex) -> Forest<NodeIndex> {
    let mut forest = vec!();
    let mut tree = Tree::new();

    if g.num_nodes() == 0 {
        return forest;
    }

    let mut unvisited: HashSet<NodeIndex> = g.node_indices().collect();
    let mut visited = HashSet::new();
    let mut stack = vec!();

    stack.push((start, None));
    loop {
        match stack.pop() {
            None => {
                if unvisited.is_empty() {
                    break;
                } else {
                    // we've exhausted the nodes reachable from our last start point
                    // so restart the traversal with an unvisited node
                    forest.push(tree);
                    tree = Tree::new();
                    let another = *unvisited.iter().next().unwrap();
                    stack.push((another, None));
                }
            }
            Some((ind, parent)) => {
                if !visited.contains(&ind) {
                    match parent {
                        None => tree.add_root(ind),
                        Some(p_ind) => tree.add_child(p_ind, ind),
                    }
                    visited.insert(ind);
                    unvisited.remove(&ind);

                    for i in g.reachable(ind) {
                        stack.push((i, Some(ind)));
                    }
                }
            }
        }
    }
    forest
}
