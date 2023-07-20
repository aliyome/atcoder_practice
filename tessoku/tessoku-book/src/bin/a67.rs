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

    // 貪欲法であるクラスカル法を用いて最小全域木を求める
    let mut abc = abc;

    // 辺の長さでソート
    abc.sort_by(|a, b| a.2.cmp(&b.2));

    // 短い辺から順にUnionFindで結合していく
    let mut uf = UnionFind::new(n + 1);
    let mut ans = 0;
    for &(a, b, c) in &abc {
        if !uf.same(a, b) {
            uf.unite(a, b);
            ans += c;
        }
    }
    println!("{}", ans);

    // 以下、木の直径を求める方法を実装したが、今回の問題は閉路があるため適用出来なかった。

    // let mut graph = vec![HashMap::new(); n + 1];
    // for &(a, b, c) in &abc {
    //     graph[a].insert(b, c);
    //     graph[b].insert(a, c);
    // }

    // // ダイクストラで頂点1から各頂点への距離を求める
    // let mut dist = vec![std::usize::MAX; n + 1];
    // dist[1] = 0;
    // let mut heap = BinaryHeap::new();
    // heap.push((Reverse(0), 1));
    // while let Some((Reverse(d), v)) = heap.pop() {
    //     if dist[v] < d {
    //         continue;
    //     }
    //     for (&u, &c) in graph[v].iter() {
    //         if dist[v] + c < dist[u] {
    //             dist[u] = dist[v] + c;
    //             heap.push((Reverse(d + c), u));
    //         }
    //     }
    // }

    // // 頂点1から最も遠い頂点から再度ダイクストラで各頂点への距離を求める
    // let mut max = 0;
    // let mut max_i = 0;
    // for i in 1..=n {
    //     if max < dist[i] {
    //         max = dist[i];
    //         max_i = i;
    //     }
    // }

    // let mut dist = vec![std::usize::MAX; n + 1];
    // dist[max_i] = 0;
    // let mut heap = BinaryHeap::new();
    // heap.push((Reverse(0), max_i));
    // while let Some((Reverse(d), v)) = heap.pop() {
    //     if dist[v] < d {
    //         continue;
    //     }
    //     for (&u, &c) in graph[v].iter() {
    //         if dist[v] + c < dist[u] {
    //             dist[u] = dist[v] + c;
    //             heap.push((Reverse(d + c), u));
    //         }
    //     }
    // }

    // // 最小全域木の辺の長さの総和
    // let mut max = 0;
    // for i in 1..=n {
    //     max = max.max(dist[i]);
    // }

    // println!("{}", max);
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // O(N)
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    // O(α(N)) ≒ O(1)
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }

    // O(α(N)) ≒ O(1)
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = x;
            self.size[x] += self.size[y];
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    // O(α(N)) ≒ O(1)
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
