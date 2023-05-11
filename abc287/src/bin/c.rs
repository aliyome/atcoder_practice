use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    };

    // 1. 頂点数が辺数+1であること
    if n != m + 1 {
        println!("No");
        return;
    }

    // 頂点から出ている辺の数を数える
    let mut cnt = HashMap::new();
    let mut uf = UnionFind::new(n);
    for (u, v) in uv {
        *cnt.entry(u).or_insert(0) += 1;
        *cnt.entry(v).or_insert(0) += 1;
        uf.unite(u - 1, v - 1);
    }
    let mut term = 0;
    for (_, c) in cnt {
        if c == 1 {
            term += 1;
        }
        if c > 2 {
            println!("No");
            return;
        }
    }

    // 2. 頂点から出ている辺の数が1の頂点（終端）が2つであること
    if term != 2 {
        println!("No");
        return;
    }

    // 3. 全頂点が連結していること
    for i in 0..n {
        if !uf.same(0, i) {
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
