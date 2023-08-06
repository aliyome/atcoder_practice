use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n + 1];
    for &(u, v) in &uv {
        edges[u].push(v);
    }

    let mut uf = UnionFind::new(n + 1);
    for &(u, v) in &uv {
        uf.unite(u, v);
    }

    // root_v -> count
    let mut map = HashMap::new();
    let mut map_edge = HashMap::new();
    for i in 1..=n {
        *map.entry(uf.find(i)).or_insert(0) += 1;
        *map_edge.entry(uf.find(i)).or_insert(0) += edges[i].len();
    }

    for (root_v, v_count) in map.into_iter() {
        if v_count != *map_edge.get(&root_v).unwrap() {
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
