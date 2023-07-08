use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    // 頂点1を含むグラフg1と、頂点N1+N2を含むグラフg2を作る
    let mut g1 = vec![vec![]; n1 + 1];
    let mut g2 = vec![vec![]; n1 + n2 + 1];
    for &(a, b) in &ab {
        if a <= n1 {
            g1[a].push(b);
            g1[b].push(a);
        } else {
            g2[a].push(b);
            g2[b].push(a);
        }
    }

    // 頂点1から各頂点までの距離をBFSで求める
    let mut dist1 = vec![None; n1 + 1];
    let mut q = VecDeque::new();
    let mut max = 0;
    q.push_back((1, 0));
    while let Some((v, d)) = q.pop_front() {
        if dist1[v].is_some() {
            continue;
        }
        dist1[v] = Some(d);
        max = max.max(d);
        for &u in &g1[v] {
            q.push_back((u, d + 1));
        }
    }

    // 頂点N1+N2から各頂点までの距離をBFSで求める
    let mut dist2 = vec![None; n1 + n2 + 1];
    let mut max2 = 0;
    q.push_back((n1 + n2, 0));
    while let Some((v, d)) = q.pop_front() {
        if dist2[v].is_some() {
            continue;
        }
        dist2[v] = Some(d);
        for &u in &g2[v] {
            max2 = max2.max(d);
            q.push_back((u, d + 1));
        }
    }

    println!("{}", max + max2 + 1);
}
