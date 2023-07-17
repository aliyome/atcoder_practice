use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m],
    }

    let mut graph = vec![HashMap::new(); n + 1];
    for &(a, b, c) in abc.iter() {
        graph[a].insert(b, c);
        graph[b].insert(a, c);
    }

    // dist[i] := 頂点iの最短距離
    let mut dist = vec![std::usize::MAX; n + 1];
    // (dist, node)
    let mut heap = BinaryHeap::new();

    dist[1] = 0;
    heap.push((Reverse(0usize), 1));

    while let Some((Reverse(d), node)) = heap.pop() {
        if dist[node] < d {
            continue;
        }
        for (&next, &cost) in graph[node].iter() {
            if d + cost < dist[next] {
                dist[next] = d + cost;
                heap.push((Reverse(d + cost), next));
            }
        }
    }

    for i in 1..=n {
        if dist[i] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}
