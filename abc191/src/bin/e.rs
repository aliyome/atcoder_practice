use std::collections::VecDeque;

use proconio::input;

const INF: usize = std::usize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    for (a, b, c) in edges {
        graph[a].push((b, c));
    }

    let mut dist = vec![vec![INF; n + 1]; n + 1];
    for i in 1..=n {
        let mut q = VecDeque::new();
        q.push_back((i, 0));
        while let Some((v, d)) = q.pop_front() {
            for &(u, c) in &graph[v] {
                if dist[i][u] > d + c {
                    dist[i][u] = d + c;
                    q.push_back((u, d + c));
                    for j in 1..=n {
                        dist[i][j] = dist[i][j].min(d + dist[u][j]);
                    }
                }
            }
        }
    }

    for i in 1..=n {
        if dist[i][i] == INF {
            println!("-1");
        } else {
            println!("{}", dist[i][i]);
        }
    }
}
