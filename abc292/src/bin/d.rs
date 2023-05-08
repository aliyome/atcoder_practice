use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    // 連結成分の統合
    let mut uf = UnionFind::new(n + 1);
    for (u, v) in &uv {
        uf.unite(*u, *v);
    }
    // 連結成分の数を数える
    let mut roots = HashSet::new();
    for i in 1..=n {
        let root = uf.find(i);
        roots.insert(root);
    }
    // 連結成分ごとの辺数を数える
    for root_v in roots {
        let vert_count = uf.size[root_v];
        let mut edge_count = 0;
        for (u, v) in &uv {
            if uf.same(root_v, *u) {
                edge_count += 1;
            }
        }
        if edge_count != vert_count {
            println!("No");
            return;
        }
    }

    println!("Yes");
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
