use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // 10^5
        m: usize, // 10^5
        uv: [(usize, usize); m],
        k: usize, // 10^5
        xy: [(usize, usize); k],
        q: usize, // 10^5
        qp: [(usize, usize); q],
    }

    let mut uf = UnionFind::new(n + 1);
    for &(u, v) in &uv {
        uf.unite(u, v);
    }

    let mut roots = vec![0; n + 1];
    for i in 1..=n {
        roots[i] = uf.find(i);
    }

    let mut bad = HashSet::new();
    for &(x, y) in &xy {
        bad.insert((roots[x], roots[y]));
        bad.insert((roots[y], roots[x]));
    }

    for (p, q) in qp {
        let root_p = uf.find(p);
        let root_q = uf.find(q);
        if bad.contains(&(root_p, root_q)) {
            println!("No");
        } else {
            println!("Yes");
        }
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
}
