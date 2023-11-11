use proconio::input;
use proconio::marker::Usize1;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root != y_root {
            self.parent[y_root] = x_root;
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        edges: [(Usize1, Usize1, i64); m],
    }

    let mut edges = edges;
    edges.sort_by_key(|k| k.2);

    let mut uf = UnionFind::new(n);
    let mut min_cost = 0;

    for (u, v, w) in edges {
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            min_cost = (min_cost + w) % k;
        }
    }

    println!("{}", min_cost);
}
