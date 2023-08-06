use std::{cmp::Reverse, collections::BinaryHeap, fmt::Binary};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    };

    let mut edge = vec![BinaryHeap::new(); n + 1];
    let mut edge_from_1 = BinaryHeap::new();
    for &(a, b) in &ab {
        if a == 1 {
            edge_from_1.push(Reverse(b));
        } else if b == 1 {
            edge_from_1.push(Reverse(a));
        } else {
            edge[a].push(Reverse(b));
            edge[b].push(Reverse(a));
        }
    }

    let mut visited = vec![false; n + 1];
    let mut path = vec![1usize];
    visited[1] = true;
    while path.len() > 0 {
        let v = *path.last().unwrap();
        print!("{} ", v);

        let mut ok = false;
        if v == 1 {
            while let Some(Reverse(u)) = edge_from_1.pop() {
                if !visited[u] {
                    visited[u] = true;
                    path.push(u);
                    ok = true;
                    break;
                }
            }
        } else {
            while let Some(Reverse(u)) = edge[v].pop() {
                if !visited[u] {
                    visited[u] = true;
                    path.push(u);
                    ok = true;
                    break;
                }
            }
        }

        if !ok {
            path.pop();
        }
    }
}
