use std::collections::{HashMap, HashSet};

use proconio::input;

// WAAAAAAAAAAAAAAAAAAAAAAAAAA
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);
    let mut removed = HashSet::new();
    let mut history = vec![];

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: usize,
                v: usize,
            }
            uf.unite(u - 1, v - 1);
            history.push((u - 1, v - 1));
        } else {
            input! {
                u: usize,
            }
            removed.insert(u - 1);
            uf = UnionFind::new(n);

            for &(u, v) in &history {
                if !removed.contains(&u) && !removed.contains(&v) {
                    uf.unite(u, v);
                }
            }
        }
        let mut counts = vec![0; n];
        for i in 0..n {
            println!("u:{}, find:{}", i + 1, uf.find(i) + 1);
        }
        println!("{:?} {:?} {:?}", uf.parent, uf.size, uf.rank);
    }
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

    fn is_independent(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
}
