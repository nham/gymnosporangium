pub use std::collections::{HashSet, HashMap};
use graph::{Graph, Ungraph, Digraph};

mod graph;
mod traversal;
mod tree;

fn main() {
    let mut g = Ungraph::new();

    println!("{}", g);
    g.add_node('a');
    g.add_node('b');
    g.add_node('c');
    println!("{}", g);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    println!("{}", g);


    let mut h = Digraph::new();

    println!("{}", h);
    h.add_node('a');
    h.add_node('b');
    h.add_node('c');
    println!("{}", h);
    h.add_edge(0, 1);
    h.add_edge(0, 2);
    println!("{}", h);
}
