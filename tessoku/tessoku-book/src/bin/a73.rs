use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abcd: [(usize, usize, usize, usize); m]
    }

    // 重み付き無向グラフ
    let mut graph = vec![HashMap::new(); n + 1];
    for &(a, b, c, d) in &abcd {
        graph[a].insert(b, (c, d));
        graph[b].insert(a, (c, d));
    }

    // dist[i] := 頂点1から頂点iへの(最短距離, 木の数)
    let mut dist = vec![(std::usize::MAX, 0); n + 1];
    let mut heap = BinaryHeap::new();
    dist[1] = (0, 0);
    heap.push((Reverse(0), 0, 1)); // (Reverse(最短距離), 木の数, 頂点番号)
    while let Some((Reverse(d), tree, v)) = heap.pop() {
        if dist[v].0 < d {
            continue;
        }
        if dist[v].0 == d && dist[v].1 > tree {
            continue;
        }

        for (&u, &(c, t)) in graph[v].iter() {
            if dist[u].0 < d + c {
                continue;
            }
            if dist[u].0 == d + c && dist[u].1 >= tree + t {
                continue;
            }
            dist[u] = (d + c, tree + t);
            heap.push((Reverse(d + c), tree + t, u));
        }
    }

    println!("{} {}", dist[n].0, dist[n].1);
}
