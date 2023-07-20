use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m]
    }

    let mut graph = vec![HashMap::new(); n + 1];
    for &(a, b, c) in &abc {
        graph[a].insert(b, c);
        graph[b].insert(a, c);
    }

    // ダイクストラで頂点1から各頂点への距離を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    dist[1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 1));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for (&u, &c) in graph[v].iter() {
            if dist[v] + c < dist[u] {
                dist[u] = dist[v] + c;
                heap.push((Reverse(d + c), u));
            }
        }
    }

    // 頂点1から最も遠い頂点から再度ダイクストラで各頂点への距離を求める
    let mut max = 0;
    let mut max_i = 0;
    for i in 1..=n {
        if max < dist[i] {
            max = dist[i];
            max_i = i;
        }
    }

    let mut dist = vec![std::usize::MAX; n + 1];
    dist[max_i] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), max_i));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for (&u, &c) in graph[v].iter() {
            if dist[v] + c < dist[u] {
                dist[u] = dist[v] + c;
                heap.push((Reverse(d + c), u));
            }
        }
    }

    // 最小全域木の辺の長さの総和
    let mut max = 0;
    for i in 1..=n {
        max = max.max(dist[i]);
    }

    println!("{}", max);
}
