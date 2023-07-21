use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      a: [usize; n],
      xyz: [(usize, usize, usize); m]
    }

    // 1 << n で 2^n 通りの状態を表す
    // 各状態をグラフの頂点とみなす
    let mut graph = vec![vec![]; 1 << n];
    for &(x, y, z) in &xyz {
        for i in 0..1 << n {
            let target = (1 << (x - 1)) | (1 << (y - 1)) | (1 << (z - 1));
            graph[i].push(i ^ target);
        }
    }
    // for i in 0..1 << n {
    //     for j in &graph[i] {
    //         println!("{:04b} {:04b}", i, j);
    //     }
    // }

    let mut start = 0;
    for i in 0..n {
        if a[i] == 1 {
            start |= 1 << i;
        }
    }
    // println!("{:b}", start);

    let mut dist = vec![std::usize::MAX; 1 << n];
    let mut q = BinaryHeap::new();
    dist[start] = 0;
    q.push((Reverse(start), 0));

    while let Some((Reverse(v), d)) = q.pop() {
        if dist[v] < d {
            continue;
        }

        for &u in &graph[v] {
            if dist[u] <= d + 1 {
                continue;
            }
            dist[u] = d + 1;
            q.push((Reverse(u), d + 1));
        }
    }

    if dist[(1 << n) - 1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[(1 << n) - 1]);
    }
}
