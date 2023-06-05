use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize, // 10^5
        m: usize, // 10^5
        edges: [(usize, usize); m],
        k: usize, // 10^5
        unconnected: [(usize, usize); k],
        q: usize, // 10^5
        queries: [(usize, usize); q],
    }

    // O(M)
    // UnionFind で連結成分を求める
    let mut uf = UnionFind::new(n);
    for (u, v) in edges {
        uf.unite(u - 1, v - 1);
    }

    // O(N)
    // 各頂点ごとにどのツリーに属するかのマップを作る(連結成分ごとに集合を作る)
    let mut map = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        map.insert(i, root);
    }

    // O(K)
    // 連結しては行けない集合を求める
    let mut unconnected_map = HashMap::new();
    for (u, v) in unconnected {
        let u_root = uf.find(u - 1);
        let v_root = uf.find(v - 1);
        unconnected_map
            .entry(u_root)
            .or_insert(HashSet::new())
            .insert(v_root);
        unconnected_map
            .entry(v_root)
            .or_insert(HashSet::new())
            .insert(u_root);
    }

    // O(Q)
    // お互いの頂点が連結してはいけない集合に属しているかを調べる
    for &(p, q) in &queries {
        // どの集合に属しているか
        let u = map.get(&(p - 1)).unwrap();
        let v = map.get(&(q - 1)).unwrap();
        // 連結してはいけない集合に属しているか
        if unconnected_map
            .get(u)
            .unwrap_or(&HashSet::new())
            .contains(v)
        {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

#[derive(Debug, Clone)]
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
