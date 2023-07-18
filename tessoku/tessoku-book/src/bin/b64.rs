use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      abc: [(usize, usize, usize); m]
    }

    // グラフ構築
    let mut graph = vec![HashMap::new(); n + 1];
    for (a, b, c) in abc {
        graph[a].insert(b, c);
        graph[b].insert(a, c);
    }

    // ダイクストラ法で頂点1から各頂点への最短経路を求める
    let mut dist = vec![std::usize::MAX; n + 1];
    let mut heap = BinaryHeap::new();
    dist[1] = 0;
    heap.push((Reverse(0), 1));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] < d {
            continue;
        }

        for (&u, &c) in &graph[v] {
            if dist[u] > dist[v] + c {
                dist[u] = dist[v] + c;
                heap.push((Reverse(dist[u]), u));
            }
        }
    }

    // 頂点Nから1の経路を復元する
    let mut path = vec![n];
    while path.last().unwrap() != &1 {
        let cur = *path.last().unwrap();
        for (&u, &c) in graph[cur].iter() {
            if dist[u] + c == dist[cur] {
                path.push(u);
                break;
            }
        }
    }

    // 経路の出力
    path.reverse();
    for v in path {
        print!("{} ", v);
    }
}
