use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    };

    let mut edges = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    // dijkstra で頂点1からの距離を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    dist[1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 1));
    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &u in &edges[v] {
            if dist[u] <= d + 1 {
                continue;
            }
            dist[u] = d + 1;
            heap.push((Reverse(d + 1), u));
        }
    }
    // println!("{:?}", dist);

    // 一番遠い頂点を求める
    let mut max = 0;
    let mut max_i = 0;
    for i in 1..=n {
        if dist[i] > max {
            max = dist[i];
            max_i = i;
        }
    }

    // max_i からの距離を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    dist[max_i] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), max_i));
    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &u in &edges[v] {
            if dist[u] <= d + 1 {
                continue;
            }
            dist[u] = d + 1;
            heap.push((Reverse(d + 1), u));
        }
    }
    // println!("{:?}", dist);

    // 一番遠い頂点からの距離を求める
    let mut max = 0;
    for i in 1..=n {
        if dist[i] > max {
            max = dist[i];
        }
    }

    println!("{}", max + 1);
}
