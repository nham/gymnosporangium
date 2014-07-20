use {HashSet};
use std::collections::{HashMap, RingBuf, Deque};
use graph::{Graph, NodeIndex, Digraph};

/// Do a breadth-first search of the graph, returning the resulting breadth-
/// first forest.
fn bfs_forest<T, G: Graph<T>>(g: &G, start: NodeIndex) -> Digraph<NodeIndex> {
    let mut forest = Digraph::new();

    if g.num_nodes() == 0 {
        return forest;
    }

    let mut unvisited = HashSet::new();
    let mut visited = HashSet::new();
    let mut discovered = RingBuf::new();

    for i in g.node_indices() {
        unvisited.insert(i);
    }

    discovered.push_back((start, None));
    loop {
        match discovered.pop_front() {
            None => {
                if unvisited.len() == 0 {
                    break;
                } else {
                    let another = unvisited.iter().next().unwrap();
                    discovered.push_back((*another, None));
                    continue;
                }
            },
            Some((ind, parent)) => {
                forest.add_node(ind);
                if parent.is_some() {
                    forest.add_edge(parent.unwrap(), ind);
                }
                visited.insert(ind);
                unvisited.remove(&ind);

                for i in g.adj(ind) {
                    if !visited.contains(&i) {
                        discovered.push_back((i, Some(ind)));
                    }
                }
            }
        }
    }
    return forest;
}

