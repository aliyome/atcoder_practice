use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![std::usize::MAX; n + 1];
    let mut q = VecDeque::new();
    q.push_back((1, 0));
    while let Some((v, d)) = q.pop_front() {
        dist[v] = dist[v].min(d);
        for &u in &graph[v] {
            if dist[u] <= d + 1 {
                continue;
            }
            q.push_back((u, d + 1));
        }
    }

    for i in 1..=n {
        if dist[i] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}
