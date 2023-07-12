use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n - 1],
        q: usize,
        k: usize,
        xy: [(usize, usize); q]
    };

    let mut graph = vec![vec![]; n + 1];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    // ダイクストラ法で頂点 k から各頂点への最短距離を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    let mut heap = BinaryHeap::new();
    dist[k] = 0;
    heap.push((0, k));
    while let Some((d, v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for &(u, c) in &graph[v] {
            if dist[u] > dist[v] + c {
                dist[u] = dist[v] + c;
                heap.push((dist[u], u));
            }
        }
    }

    for &(x, y) in &xy {
        println!("{}", dist[x] + dist[y]);
    }
}
