use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m]
    }

    let mut edge = vec![vec![]; n + 1];
    for &(a, b, c) in &abc {
        edge[a].push((b, c));
        edge[b].push((a, c));
    }

    let mut dist_from_1 = vec![1_000_000_000; n + 1];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 1));
    dist_from_1[1] = 0;
    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist_from_1[v] {
            continue;
        }
        for &(u, c) in &edge[v] {
            if dist_from_1[u] > dist_from_1[v] + c {
                dist_from_1[u] = dist_from_1[v] + c;
                heap.push((Reverse(dist_from_1[u]), u));
            }
        }
    }

    let mut dist_from_n = vec![1_000_000_000; n + 1];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), n));
    dist_from_n[n] = 0;
    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist_from_n[v] {
            continue;
        }
        for &(u, c) in &edge[v] {
            if dist_from_n[u] > dist_from_n[v] + c {
                dist_from_n[u] = dist_from_n[v] + c;
                heap.push((Reverse(dist_from_n[u]), u));
            }
        }
    }

    let mut ans = 0usize;
    for i in 1..=n {
        if dist_from_1[i] + dist_from_n[i] == dist_from_1[n] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
