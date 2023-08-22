use petgraph::{algo::dijkstra, prelude::*};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    };

    let mut graph: Graph<(), (), Undirected> = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..n).map(|_| graph.add_node(())).collect();
    graph.extend_with_edges(ab.iter().map(|&(a, b)| (nodes[a - 1], nodes[b - 1])));
    let dist = dijkstra(&graph, nodes[0], None, |_| 1);

    let mut max = 0;
    let mut max_node = NodeIndex::new(0);
    for (k, v) in dist.iter() {
        if *v > max {
            max = *v;
            max_node = *k;
        }
    }

    let dist = dijkstra(&graph, max_node, None, |_| 1);
    let mut max = 0;
    for (k, v) in dist.iter() {
        if *v > max {
            max = *v;
        }
    }
    println!("{}", max + 1);
}
