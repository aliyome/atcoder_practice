use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    // edge[u] := u から出る辺の行き先
    let mut edge = vec![vec![]; n + 1];
    for (a, b) in ab {
        edge[a].push(b);
    }

    // 各頂点から到達可能な頂点を BFS で求める
    let mut ans = 0;
    for u in 1..=n {
        let mut queue = VecDeque::new();
        for &v in &edge[u] {
            queue.push_back(v);
        }

        let mut visit = vec![false; n + 1];
        visit[u] = true;
        while let Some(v) = queue.pop_front() {
            visit[v] = true;
            for &w in &edge[v] {
                if !visit[w] {
                    queue.push_back(w);
                }
            }
        }

        ans += visit.iter().filter(|&&b| b).count();
    }

    println!("{}", ans);
}
