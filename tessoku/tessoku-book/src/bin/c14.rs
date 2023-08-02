use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

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

    let mut dist = vec![1_000_000_000; n + 1];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 1));
    dist[1] = 0;

    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &(u, c) in &edge[v] {
            if dist[u] > dist[v] + c {
                dist[u] = dist[v] + c;
                heap.push((Reverse(dist[u]), u));
            }
        }
    }

    // 逆側から辿る
    let mut stack = vec![];
    stack.push(n);

    let mut set = HashSet::new();
    set.insert(n);
    set.insert(1);
    while let Some(v) = stack.pop() {
        if v == 1 {
            break;
        }
        for &(u, c) in &edge[v] {
            if dist[u] + c == dist[v] {
                stack.push(u);
                set.insert(u);
            }
        }
    }

    println!("{}", set.len());
}
