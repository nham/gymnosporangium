pub use std::collections::HashSet;
use graph::{Graph, Ungraph, Digraph};

mod graph;
mod algorithms;
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
    let a = h.add_node('a');
    let b = h.add_node('b');
    let c = h.add_node('c');
    println!("{}", h);
    h.add_edge(a, b);
    h.add_edge(a, c);
    println!("{}", h);

    let d = h.add_node('d');
    let e = h.add_node('e');
    h.add_edge(d, a);
    h.add_edge(e, a);
    h.add_edge(d, e);

    println!("-----------");
    println!("{}", h);
    println!("{}", h.transpose());
    let mut nix_d = HashSet::new();
    nix_d.insert(a);
    nix_d.insert(b);
    nix_d.insert(c);
    nix_d.insert(e);
    println!("{}", h.induced_subgraph(&nix_d));

    println!("-----------");
    let x: Digraph<()> = Digraph::new();
    println!("{}", x);
    println!("{}", x.transpose());
    println!("-----------");

    // testing induced_subgraph for an undirected graph
    let mut y = Ungraph::new();
    let a = y.add_node('a');
    let b = y.add_node('b');
    let c = y.add_node('c');
    let d = y.add_node('d');
    let e = y.add_node('e');
    let f = y.add_node('f');
    y.add_edge(a, b);
    y.add_edge(a, d);
    y.add_edge(a, e);
    y.add_edge(b, c);
    y.add_edge(c, e);
    y.add_edge(d, e);
    y.add_edge(d, f);
    y.add_edge(e, f);
    println!("{}", y);
    let mut nix_ab = HashSet::new();
    nix_ab.insert(c);
    nix_ab.insert(d);
    nix_ab.insert(e);
    nix_ab.insert(f);
    println!("{}", y.induced_subgraph(&nix_ab));
}
