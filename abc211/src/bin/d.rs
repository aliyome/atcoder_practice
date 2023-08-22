use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut edge = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        edge[a].push(b);
        edge[b].push(a);
    }

    // ダイクストラで最短経路を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    let mut heap = BinaryHeap::new();
    let mut list = vec![];
    dist[1] = 0;
    heap.push((Reverse(0), 1));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        list.push(v);
        for &u in &edge[v] {
            if dist[u] > d + 1 {
                dist[u] = d + 1;
                heap.push((Reverse(d + 1), u));
            }
        }
    }
    // 最短コスト
    let min = dist[n];

    if min == std::usize::MAX {
        println!("0");
        return;
    }

    let mut count = vec![0; n + 1];
    count[1] = 1;
    for &v in &list {
        for &u in &edge[v] {
            if dist[v] == dist[u] + 1 {
                count[v] += count[u];
                count[v] %= 1000000007;
            }
        }
    }

    println!("{}", count[n]);
}
