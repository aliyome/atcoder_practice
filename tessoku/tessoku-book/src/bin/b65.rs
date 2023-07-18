use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      n: usize,
      t: usize,
      ab: [(usize, usize); n - 1]
    }

    // 木構造を作る
    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    // ルートから辿って親子関係を確定させる
    let mut fixed = vec![false; n + 1];
    let mut children = vec![vec![]; n + 1];
    let mut parents = vec![vec![]; n + 1];
    let mut child_count = vec![0; n + 1];
    let mut q = VecDeque::new();
    q.push_back(t);
    while let Some(v) = q.pop_front() {
        for &u in &graph[v] {
            if fixed[u] {
                continue;
            }
            children[v].push(u);
            q.push_back(u);
            child_count[v] += 1;
            parents[u].push(v);
        }
        fixed[v] = true;
    }

    // 葉を探す
    let mut targets = VecDeque::new();
    for i in 1..=n {
        if children[i].is_empty() {
            targets.push_back(i);
        }
    }

    // BFSで子から親に向かって階級を求める
    let mut rank = vec![0isize; n + 1];
    while let Some(v) = targets.pop_front() {
        let mut max = -1;
        for &child in &children[v] {
            max = max.max(rank[child]);
        }
        rank[v] = max + 1;

        for &p in &parents[v] {
            child_count[p] -= 1;
            if child_count[p] == 0 {
                targets.push_back(p);
            }
        }
    }

    // 出力
    for i in 1..=n {
        print!("{} ", rank[i]);
    }
}
