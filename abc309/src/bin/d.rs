use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph = vec![vec![]; n1 + n2 + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![std::usize::MAX; n1 + n2 + 1];
    let mut heap = BinaryHeap::new();
    dist[1] = 0;
    heap.push((Reverse(0), 1));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for &u in &graph[v] {
            if dist[u] > d + 1 {
                dist[u] = d + 1;
                heap.push((Reverse(d + 1), u));
            }
        }
    }
    let max1 = dist
        .into_iter()
        .filter(|&x| x != std::usize::MAX)
        .max()
        .unwrap();

    let mut dist = vec![std::usize::MAX; n1 + n2 + 1];
    let mut heap = BinaryHeap::new();
    dist[n1 + n2] = 0;
    heap.push((Reverse(0), n1 + n2));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for &u in &graph[v] {
            if dist[u] > d + 1 {
                dist[u] = d + 1;
                heap.push((Reverse(d + 1), u));
            }
        }
    }
    let max2 = dist
        .into_iter()
        .filter(|&x| x != std::usize::MAX)
        .max()
        .unwrap();

    println!("{}", max1 + 1 + max2);
}
