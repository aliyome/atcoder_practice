use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m],
    }

    let mut graph = vec![HashMap::new(); n + 1];
    for &(a, b, c) in abc.iter() {
        graph[a].insert(b, c);
        graph[b].insert(a, c);
    }

    let mut dist = vec![std::usize::MAX; n + 1];
    dist[1] = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back((1, 0));

    while let Some((v, d)) = q.pop_front() {
        for (&u, &c) in graph[v].iter() {
            if dist[u] <= d + c {
                continue;
            }
            dist[u] = d + c;
            q.push_back((u, d + c));
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
